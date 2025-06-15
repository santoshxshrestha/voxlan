#![allow(unused)]
use std::io;
use std::io::Write;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub async fn connect(target_port: u16, ip: String) -> Result<(), std::io::Error> {
    println!(
        "Hello there rustacean got the target_port:{} and ip:{}",
        target_port, ip
    );
    let mut stream = TcpStream::connect(format!("{}:{}", ip, target_port)).await?;

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

        stream
            .write_all((format!("{}", message)).as_bytes())
            .await?;
    }
    Ok(())
}
