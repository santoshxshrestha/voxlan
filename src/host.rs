#![allow(unused)]
use crate::show_pulsing;
use std::error::Error;
use std::io;
use std::io::Write;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};

pub async fn handle_write(mut owned_write_half: OwnedWriteHalf) -> io::Result<()> {
    loop {
        // These will work fine but   io::stdin().read_line(&mut input)?;  will block the fn so,
        // not to use it
        // let stdin = tokio::io::stdin();
        // let reader = BufReader::new(stdin);
        // let mut lines = reader.lines();
        let input = tokio::task::spawn_blocking(|| {
            print!(" 󰁕 ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            input
        })
        .await
        .unwrap();

        let message = input.trim();
        if message == "quit" || message == "q" || message == "exit" {
            println!("󱠡 Goodbye!");
            break;
        }
        if message.is_empty() {
            continue;
        }
        owned_write_half.write_all(message.as_bytes()).await?;
    }
    Ok(())
}

pub async fn handle_read(mut owned_read_half: OwnedReadHalf) -> io::Result<()> {
    loop {
        let mut buffer = vec![0; 1024];
        match owned_read_half.read(&mut buffer).await {
            Ok(0) => {
                println!("Client Disconnected gracefully");
                break;
            }
            Ok(n) => buffer.truncate(n),
            Err(e) if e.kind() == io::ErrorKind::ConnectionReset => {
                println!("Client disconnected abruptly (reset)");
                break;
            }
            Err(e) => {
                println!("Failed reading stream:{}", e);
            }
        }
        let message = String::from_utf8_lossy(&buffer[..]);
        println!("\n← {}", message.trim());
        print!(" 󰁕 ");
        io::stdout().flush().unwrap();
    }
    Ok(())
}

pub async fn host(bind_port: u16, local_ip: String) -> io::Result<()> {
    let listener = TcpListener::bind(format!("0.0.0.0:{}", bind_port)).await?;
    show_pulsing();
    // println!("Server listening on port {}", bind_port);
    println!(
        "connect to this server by following command \n voxlan connect -i {} -t {}",
        local_ip, bind_port
    );

    loop {
        let (stream, addr) = listener.accept().await?;
        println!("New connection {}", addr);
        let (owned_read_half, owned_write_half) = stream.into_split();

        tokio::spawn(async move {
            let (r, w) = tokio::join!(handle_read(owned_read_half), handle_write(owned_write_half));
            if let Err(e) = r {
                eprintln!("Read error: {}", e);
            }
            if let Err(e) = w {
                eprintln!("Write error: {}", e);
            }
        });
    }
}
