use reqwest;
use std::io::{self, Write};

#[tokio::main]
pub async fn client(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Echo Server Client");
    println!(
        "Make sure your server is running on http://localhost:{}",
        port
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
            println!("👋 Goodbye!");
            break;
        }

        if message.is_empty() {
            continue;
        }

        match send_to_server(&client, message).await {
            Ok(response) => println!("✅ Server replied: {}", response),
            Err(e) => println!("❌ Error: {}", e),
        }
        println!();
    }

    Ok(())
}

pub async fn send_to_server(
    client: &reqwest::Client,
    message: &str,
    port: u16,
) -> Result<String, reqwest::Error> {
    let response = client
        .post("http://localhost:{port}/echo")
        .header("Content-Type", "text/plain")
        .body(message.to_string())
        .send()
        .await?;

    response.text().await
}
