use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:8080".parse::<SocketAddr>()?;
    let listener = TcpListener::bind(addr).await?;

    println!("Server listening on {}", addr);

    loop {
        let (mut stream, remote_addr) = listener.accept().await?;
        println!("Accepted connection from {}", remote_addr);

        tokio::spawn(async move {
            let mut buf = [0; 1024];
            loop {
                match stream.read(&mut buf).await {
                    Ok(0) => return,
                    Ok(n) => {
                        if let Err(e) = stream.write_all(&buf[..n]).await {
                            eprintln!("failed to write to socket; error = {:?}", e);
                            return;
                        }
                    }
                    Err(e) => {
                        eprintln!("failed to read from socket; error = {:?}", e);
                        return;
                    }
                }
            }
        });
    }
}