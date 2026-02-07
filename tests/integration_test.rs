use simple_chat::server::connection::handle_connection;
use simple_chat::server::state::ServerState;
use simple_chat::transport::messages::{ClientMessage, ServerMessage};
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::sync::Mutex;

#[tokio::test]
async fn test_chat_interaction() {
    // Start Server
    let state = Arc::new(Mutex::new(ServerState::new()));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    let server_state = state.clone();
    tokio::spawn(async move {
        loop {
            let (socket, _) = listener.accept().await.unwrap();
            let state = server_state.clone();
            tokio::spawn(async move {
                handle_connection(socket, state).await.unwrap();
            });
        }
    });

    // Client 1 Connects (Alice)
    let client1 = TcpStream::connect(addr).await.unwrap();
    let (read1, mut write1) = client1.into_split();
    let mut reader1 = BufReader::new(read1).lines();

    // Alice Joins
    let join_msg = serde_json::to_string(&ClientMessage::Join {
        username: "Alice".into(),
    })
    .unwrap();
    write1.write_all(join_msg.as_bytes()).await.unwrap();
    write1.write_all(b"\n").await.unwrap();

    // Client 2 Connects (Bob)
    let client2 = TcpStream::connect(addr).await.unwrap();
    let (_read2, mut write2) = client2.into_split();
    // let mut reader2 = BufReader::new(read2).lines(); // Unused

    // Bob Joins
    let join_msg = serde_json::to_string(&ClientMessage::Join {
        username: "Bob".into(),
    })
    .unwrap();
    write2.write_all(join_msg.as_bytes()).await.unwrap();
    write2.write_all(b"\n").await.unwrap();

    // Test: Bob sends message to Alice
    let send_msg = serde_json::to_string(&ClientMessage::Send {
        msg: "Hello Alice".into(),
    })
    .unwrap();
    write2.write_all(send_msg.as_bytes()).await.unwrap();
    write2.write_all(b"\n").await.unwrap();

    // Verify Alice receives message
    let line = reader1
        .next_line()
        .await
        .unwrap()
        .expect("Alice should receive message");
    let msg: ServerMessage = serde_json::from_str(&line).expect("Should be valid JSON");

    match msg {
        ServerMessage::Message { from, msg } => {
            assert_eq!(from, "Bob");
            assert_eq!(msg, "Hello Alice");
        }
        _ => panic!("Expected Message type"),
    }

    // Test: Alice leaves (closes connection)
    let leave_msg = serde_json::to_string(&ClientMessage::Leave).unwrap();
    write1.write_all(leave_msg.as_bytes()).await.unwrap();
    write1.write_all(b"\n").await.unwrap();
}
