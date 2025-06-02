use tokio::io;
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    println!("Proxy listening on 0.0.0.0:8080");

    loop {
        let (client, addr) = listener.accept().await?;
        println!("New connection from {}", addr);

        tokio::spawn(async move {
            if let Err(e) = proxy_connection(client).await {
                eprintln!("Error: {}", e);
            }
        });
    }
}

async fn proxy_connection(client: TcpStream) -> io::Result<()> {
    let server = TcpStream::connect("127.0.0.1:8080").await?;

    let (mut client_read, mut client_write) = client.into_split();
    let (mut server_read, mut server_write) = server.into_split();

    tokio::select! {
        _ = tokio::io::copy(&mut client_read, &mut server_write) => {},
        _ = tokio::io::copy(&mut server_read, &mut client_write) => {},
    }

    Ok(())
}
