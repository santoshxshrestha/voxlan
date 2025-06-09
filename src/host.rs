#![allow(unused)]

use std::io;
use tokio::net::TcpListener;

pub async fn process_socket<T>(socket: T) {}
pub async fn handle_connection(mut stream: TcpListener) -> Result {
    let mut buffer = [0; 1024];
}

pub async fn host(bind_port: usize) -> io::Result<()> {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", bind_port)).await?;
    loop {
        let (stream, addr) = listener.accept().await?;
        println!("New connection {}", addr);
        process_socket(socket).await;

        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream).await {
                eprintln!("Error handling connection: {}", e);
            }
        });
    }
}

