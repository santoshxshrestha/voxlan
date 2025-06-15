#![allow(unused)]
use std::error::Error;
use std::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

pub async fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buffer = [0; 1024];

    loop {
        let n = stream.read(&mut buffer).await?;

        if n == 0 {
            println!("Connection closed");
            break;
        }

        let message = String::from_utf8_lossy(&buffer[..n]);
        println!("Received message: {}", message.trim());

        stream
            .write_all(format!("Echo: {}", message).as_bytes())
            .await?;
    }

    Ok(())
}

pub async fn read_line(mut stream: TcpStream) -> Result<String, Box<dyn Error>> {
    let mut buffer = Vec::new();
    let mut byte = [0, 1];

    loop {
        stream.read_exact(&mut byte).await?;
        buffer.push(byte[0]);

        if byte[0] == b'\n' {
            break;
        }
    }

    let message = String::from_utf8(buffer)?;
    Ok(message.trim().to_string())
}

pub async fn host(bind_port: u16) -> io::Result<()> {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", bind_port)).await?;
    loop {
        //here the stream and the add are the socket and the ip of the connnected thinge
        let (stream, addr) = listener.accept().await?;
        println!("New connection {}", addr);

        tokio::spawn(async move {
            if let Err(e) = read_line(stream).await {
                eprintln!("Error handling connection: {}", e);
            }
        });
    }
}
