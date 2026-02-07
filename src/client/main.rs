use std::env;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    let username = if args.len() > 1 {
        args[1].clone()
    } else {
        eprintln!("Usage: {} <username> [host] [port]", args[0]);
        return Ok(());
    };

    let host = if args.len() > 2 {
        args[2].clone()
    } else {
        host
    };
    let port = if args.len() > 3 {
        args[3].clone()
    } else {
        port
    };

    let addr = format!("{}:{}", host, port);
    println!("Connecting to {} as {}", addr, username);

    // will exit more gracefully later.... There are many places which might need to be fixed.
    let mut stream = TcpStream::connect(addr).await?;

    // Handshake: Send Join
    let join_msg = simple_chat::transport::messages::ClientMessage::Join {
        username: username.clone(),
    };
    let json = serde_json::to_string(&join_msg)?;
    stream.write_all(json.as_bytes()).await?;
    stream.write_all(b"\n").await?;

    let (read_half, mut write_half) = stream.into_split();

    // Task to read from server and print to stdout
    let mut reader = BufReader::new(read_half);
    tokio::spawn(async move {
        let mut line = String::new();
        while let Ok(n) = reader.read_line(&mut line).await {
            if n == 0 {
                break;
            } // EOF
            if let Ok(msg) =
                serde_json::from_str::<simple_chat::transport::messages::ServerMessage>(&line)
            {
                match msg {
                    simple_chat::transport::messages::ServerMessage::Message { from, msg } => {
                        println!("[{}]: {}", from, msg);
                    }
                    simple_chat::transport::messages::ServerMessage::Error { msg } => {
                        eprintln!("Error from server: {}", msg);
                    }
                }
            } else {
                // If it's not JSON, maybe simple text or debug?
                println!("Rate: {}", line.trim());
            }
            line.clear();
        }
        println!("Disconnected from server.");
        std::process::exit(0);
    });

    // Main task: Read from stdin
    let mut stdin = BufReader::new(tokio::io::stdin());
    let mut line = String::new();

    println!("Connected! Type 'send <msg>' to chat, or 'leave' to quit.");

    loop {
        line.clear();
        let n = stdin.read_line(&mut line).await?;
        if n == 0 {
            break;
        }

        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        if trimmed == "leave" {
            let msg = simple_chat::transport::messages::ClientMessage::Leave;
            let json = serde_json::to_string(&msg)?;
            write_half.write_all(json.as_bytes()).await?;
            write_half.write_all(b"\n").await?;
            break;
        } else if let Some(msg_content) = trimmed.strip_prefix("send ") {
            let msg = simple_chat::transport::messages::ClientMessage::Send {
                msg: msg_content.to_string(),
            };
            let json = serde_json::to_string(&msg)?;
            write_half.write_all(json.as_bytes()).await?;
            write_half.write_all(b"\n").await?;
        } else {
            println!("Unknown command. Use 'send <msg>' or 'leave'.");
        }
    }

    Ok(())
}
