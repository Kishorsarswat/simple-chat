const HOST: &str = "127.0.0.1";
const PORT: u16 = 8080;

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    let addr = format!("{}:{}", HOST, PORT);
    let listner = tokio::net::TcpListener::bind(&addr).await?;
    
    println!("Chat server listening on {}", addr);

    loop {
        let (socket, addr) = listner.accept().await?;
        println!("New connection from {}", addr);
        tokio::spawn(async move {
            if let Err(e) = simple_chat::server::connection::handle_connection(socket).await {
                eprintln!("Error handling connection from {}: {:?}", addr, e);
            }
        });
    }
}

// Test powershell script to test messages
// $client = New-Object System.Net.Sockets.TcpClient("127.0.0.1", 8080)
// $stream = $client.GetStream()
// $writer = New-Object System.IO.StreamWriter($stream)
// $writer.AutoFlush = $true

// $writer.WriteLine('{"type":"join","username":"alice"}')
// $writer.WriteLine('{"type":"send","msg":"hello"}')

// $writer.WriteLine('{"type":"leave"}')
// $client.Close()

