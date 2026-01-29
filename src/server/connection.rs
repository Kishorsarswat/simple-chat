use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::TcpStream;

use crate::transport::messages::ClientMessage;

pub async fn handle_connection(stream: TcpStream) -> anyhow::Result<()> {
    let peer = stream.peer_addr()?;

    let reader = BufReader::new(stream);
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await? {
        match serde_json::from_str::<ClientMessage>(&line) {
            Ok(msg) => {
                println!("Received from {}: {:?}", peer, msg);
            }
            Err(err) => {
                eprintln!("Invalid message from {}: {}", peer, err);
            }
        }
    }

    // EOF reached (stream closed) → client disconnected
    println!("Client {} disconnected", peer);
    Ok(())
}
