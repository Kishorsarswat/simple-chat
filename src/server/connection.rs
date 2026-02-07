use std::sync::Arc;

use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::TcpStream;
use tokio::sync::Mutex;

use crate::server::state::ServerState;
use crate::transport::messages::ClientMessage;

pub async fn handle_connection(
    stream: TcpStream,
    state: Arc<Mutex<ServerState>>,
) -> anyhow::Result<()> {
    let peer = stream.peer_addr()?;
    let (read_half, mut write_half) = stream.into_split();
    let reader = BufReader::new(read_half);
    let mut lines = reader.lines();

    // 1. Wait for Join message
    let username = match lines.next_line().await? {
        Some(line) => match serde_json::from_str::<ClientMessage>(&line) {
            Ok(ClientMessage::Join { username: u }) => {
                let mut state_guard = state.lock().await;
                if state_guard.contains_user(&u) {
                    eprintln!("Username {} is already taken", u);
                    return Ok(());
                }

                let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<
                    crate::transport::messages::ServerMessage,
                >();
                state_guard.add_user(u.clone(), tx);
                println!("User {} joined from {}", u, peer);

                // Spawn task to write messages to the client
                tokio::spawn(async move {
                    use tokio::io::AsyncWriteExt;
                    while let Some(msg) = rx.recv().await {
                        if let Ok(json) = serde_json::to_string(&msg) {
                            if let Err(e) = write_half.write_all(json.as_bytes()).await {
                                eprintln!("Failed to write to socket: {}", e);
                                break;
                            }
                            if let Err(e) = write_half.write_all(b"\n").await {
                                eprintln!("Failed to write newline: {}", e);
                                break;
                            }
                        }
                    }
                });
                u
            }
            Ok(_) => {
                eprintln!("Expected Join message from {}", peer);
                return Ok(());
            }
            Err(e) => {
                eprintln!("Invalid handshake from {}: {}", peer, e);
                return Ok(());
            }
        },
        None => return Ok(()),
    };

    // 2. Main loop
    while let Some(line) = lines.next_line().await? {
        match serde_json::from_str::<ClientMessage>(&line) {
            Ok(ClientMessage::Send { msg }) => {
                let mut state_guard = state.lock().await;
                let server_msg = crate::transport::messages::ServerMessage::Message {
                    from: username.clone(),
                    msg,
                };
                state_guard.broadcast_message(&username, &server_msg);
            }
            Ok(ClientMessage::Leave) => {
                println!("User {} leaving", username);
                break;
            }
            Ok(ClientMessage::Join { .. }) => {
                eprintln!("Received unexpected Join from {}", username);
            }
            Err(e) => {
                eprintln!("Error parsing message from {}: {}", username, e);
            }
        }
    }

    // 3. Cleanup
    {
        let mut state_guard = state.lock().await;
        state_guard.remove_user(&username);
        // Note: The write task will eventually fail when socket closes or rx is dropped (rx dropped when we remove from state? No, tx is in state).
        // When we remove from state, tx is dropped. rx.recv() returns None. Write task exits.
    }
    println!("User {} disconnected", username);

    Ok(())
}
