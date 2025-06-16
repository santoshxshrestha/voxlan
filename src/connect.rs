#![allow(unused)]
use std::error::Error;
use std::io;
use std::io::Write;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};

pub async fn handle_write(mut owned_write_half: OwnedWriteHalf) -> Result<(), Box<dyn Error>> {
    loop {
        print!("Message 󰁕 ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let message = input.trim();

        if message == "quit" || message == "q" || message == "exit" {
            println!("󱠡 Goodbye!");
            break;
        }

        if message.is_empty() {
            continue;
        }

        owned_write_half
            .write_all((format!("{}", message)).as_bytes())
            .await?;
    }
    Ok(())
}

pub async fn handle_read(mut owned_read_half: OwnedReadHalf) -> Result<(), Box<dyn Error>> {
    loop {
        let mut buffer = vec![0; 1024];
        match owned_read_half.read(&mut buffer).await {
            Ok(0) => {
                println!("Client Disconnected gracefully");
                break; // Connection closed
            }
            Ok(n) => buffer.truncate(n),
            Err(e) if e.kind() == io::ErrorKind::ConnectionReset => {
                println!("Client disconnected abruptly (reset)");
                break; // loop gets exited
            }
            Err(e) => {
                println!("Failed reading stream:{}", e);
                return Err(Box::new(e));
            }
        }

        let message = String::from_utf8_lossy(&buffer[..]);
        println!("Received message: {}", message.trim());
    }
    Ok(())
}

pub async fn connect(target_port: u16, ip: String) -> Result<(), std::io::Error> {
    loop {
        let mut stream = TcpStream::connect(format!("{}:{}", ip, target_port)).await?;
        let (owned_read_half, owned_write_half) = stream.into_split();

        tokio::spawn(async move {
            if let Err(e) = handle_write(owned_write_half).await {
                eprintln!("Error handling the writer: {}", e);
            }
        });

        tokio::spawn(async move {
            if let Err(e) = handle_read(owned_read_half).await {
                eprintln!("Error handling reader: {}", e);
            }
        });
    }
    Ok(())
}
