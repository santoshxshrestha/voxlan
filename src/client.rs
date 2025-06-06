use reqwest;
use std::io::{self, Write};

pub async fn client(bind_port: u16, path: String) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "  Starting Client on port: {} and path is {}",
        bind_port, path
    );
    println!(
        "Make sure your server is running on http://localhost:{}",
        bind_port
    );
    println!("Type messages to send (or 'quit' to exit):\n");

    let client = reqwest::Client::new();

    loop {
        print!("Message > ");
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

        match send_to_server(&client, message, bind_port, path.clone()).await {
            Ok(response) => println!("✓ Server replied: {}", response),
            Err(e) => println!("󰞇 Error: {}", e),
        }
        println!();
    }

    Ok(())
}

pub async fn send_to_server(
    client: &reqwest::Client,
    message: &str,
    port: u16,
    path: String,
) -> Result<String, reqwest::Error> {
    let response = client
        .post(format!("http://localhost:{}/{}", port, path))
        .header("Content-Type", "text/plain")
        .body(message.to_string())
        .send()
        .await?;

    response.text().await
}
