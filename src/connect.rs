#![allow(unused)]
use std::io;
use std::io::Write;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

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

    pub async fn connect(target_port: u16, ip: String) -> Result<(), std::io::Error> {
        println!(
            "Hello there rustacean got the target_port:{} and ip:{}",
            target_port, ip
        );
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
