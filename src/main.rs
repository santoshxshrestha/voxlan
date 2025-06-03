mod animation;
mod proxy;
use crate::proxy::proxy;
use actix_web::{web, App, HttpServer};
use animation::{show_pulsing, start_spinner};
use qr2term::print_qr;
use reqwest::Client;
use std::net::{TcpStream, UdpSocket};
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn get_local_ip() -> Option<String> {
    let socket = UdpSocket::bind("0.0.0.0:0").ok()?;
    socket.connect("8.8.8.8:80").ok()?;
    let ip = socket.local_addr().ok()?.ip();
    Some(ip.to_string())
}

fn scan_port(ip: &str, port: usize) -> bool {
    let addr = format!("{}:{}", ip, port);
    TcpStream::connect_timeout(&addr.parse().unwrap(), Duration::from_millis(100)).is_ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let the_ip = "127.0.0.1";
    let local_ip = get_local_ip().unwrap_or_else(|| "localhost".to_string());
    // println!("Local IP: {}", local_ip);
    // println!("Scanning ports 1-9999...");

    // Using Arc<Mutex<Vec<u16>>> to safely share open_ports between threads
    let open_ports = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];

    // Scan ports in chunks to avoid creating too many threads
    let chunk_size = 100;
    for chunk_start in (1..10000).step_by(chunk_size) {
        let chunk_end = std::cmp::min(chunk_start + chunk_size, 10000);
        let ip = "127.0.0.1";
        let open_ports_clone = Arc::clone(&open_ports);

        let handle = std::thread::spawn(move || {
            let mut local_open_ports = Vec::new();
            for port in chunk_start..chunk_end {
                if scan_port(&ip, port) {
                    local_open_ports.push(port);
                    println!("Port {} is open", port);
                }
            }

            // Adding found ports to the shared vector
            if !local_open_ports.is_empty() {
                let mut ports = open_ports_clone.lock().unwrap();
                ports.extend(local_open_ports);
            }
        });

        handles.push(handle);
    }

    // Waiting for all threads to complete
    for handle in handles {
        let _ = handle.join();
    }

    //The final list of open ports
    let final_open_ports = {
        let ports = open_ports.lock().unwrap();
        ports.clone()
    };

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
