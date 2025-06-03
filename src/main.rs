#![allow(unused)]
mod args;
use clap::Parser;
mod animation;
mod proxy;
use crate::proxy::proxy;
use actix_web::{web, App, HttpServer};
use animation::{show_pulsing, start_spinner};
use args::VoxlanArgs;
use qr2term::print_qr;
use reqwest::Client;
mod net;
use net::{get_local_ip, get_port};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = VoxlanArgs::parse();

    let local_ip = get_local_ip().unwrap_or_else(|| "localhost".to_string());

    let final_open_ports = get_port();
    println!("\n=== PORT SCAN RESULTS ===");
    if final_open_ports.is_empty() {
        println!("No open ports found in range 1-9999.");
        println!("Cannot start proxy without a backend service!");
        return Ok(());
    } else {
        println!("Open ports found: {:?}", final_open_ports);
        println!("Total open ports: {}", final_open_ports.len());
    }

    // Use the first open port as the backend
    let backend_port = final_open_ports[0] as u16;
    let link = format!("http://{}:8081", local_ip);

    println!("\n=== PROXY SERVER INFO ===");
    show_pulsing();
    println!("======================================================");
    println!("Proxy running on: http://{}:8081", local_ip);
    println!("======================================================");
    println!("Forwarding requests to: http://localhost:{}", backend_port);
    println!("Backend service: http://localhost:{}", backend_port);

    println!();
    println!("======================================================");
    println!(
        "You can access the proxy at: {}\n on your other devices connected to the same network",
        link
    );
    println!("Here is the qr for you easy access");
    print_qr(link).unwrap();
    println!("Happy coding :) ");

    start_spinner();

    let client = Client::new();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .app_data(web::Data::new(backend_port))
            .default_service(web::route().to(proxy))
    })
    .bind("0.0.0.0:8081")?
    .run()
    .await
}
