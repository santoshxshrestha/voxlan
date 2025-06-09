mod args;
use host::host;
mod host;
use std::sync::atomic::{self, AtomicU16};
mod client;
use clap::Parser;
mod animation;
mod proxy;
use crate::proxy::proxy;
use actix_web::{App, HttpServer, web};
use animation::{show_pulsing, start_spinner};
use args::VoxlanArgs;
use qr2term::print_qr;
use reqwest::Client;
mod net;
use client::client;
use net::{get_local_ip, get_port, scan_port};
use std::sync::{Arc, Mutex};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let art = r#"
  _    __           __    ___    _   __
 | |  / /___  _  __/ /   /   |  / | / /
 | | / / __ \| |/_/ /   / /| | /  |/ /
 | |/ / /_/ />  </ /___/ ___ |/ /|  /  
 |___/\____/_/|_/_____/_/  |_/_/ |_/   

    "#;
    println!("{art}");
    let local_ip = get_local_ip().unwrap_or_else(|| "localhost".to_string());

    let args = VoxlanArgs::parse();
    let link = Arc::new(Mutex::new(String::new()));
    let target_port_atomic = Arc::new(AtomicU16::new(0));
    let bind_port_atomic = Arc::new(AtomicU16::new(0));

    match args.command {
        args::Commands::Run(run_args) => {
            let target_port = run_args.target_port;
            let bind_port = run_args.bind_port;
            match (target_port, bind_port) {
                (Some(target), bind) => {
                    if scan_port(target as usize) {
                        println!("Got the port {}", target);
                        target_port_atomic.store(target, atomic::Ordering::Relaxed);
                        *link.lock().unwrap() = format!("http://{}:{}", local_ip, bind);
                        bind_port_atomic.store(bind, atomic::Ordering::Relaxed);
                    } else {
                        println!(
                            "The port {} is not active check the server again or list the port and try again",
                            target
                        );
                        return Ok(());
                    }
                }
                (None, bind) => {
                    let final_open_ports = get_port();
                    println!("\n=== PORT SCAN RESULTS ===");
                    if final_open_ports.is_empty() {
                        println!("No open ports found in range 1-9999.");
                        println!("Cannot start proxy without a backend service!");
                        return Ok(());
                    } else {
                        println!("Open ports found: {:?}", final_open_ports);
                    }

                    if final_open_ports.len() > 1 {
                        println!("Total open ports: {}", final_open_ports.len());
                        println!(
                            "You have to manually specify the port that you want to use by -p <port> flag"
                        );
                    }

                    // Use the first open port as the backend
                    target_port_atomic.store(final_open_ports[0] as u16, atomic::Ordering::Relaxed);
                    *link.lock().unwrap() = format!("http://{}:{}", local_ip, bind);
                    bind_port_atomic.store(bind, atomic::Ordering::Relaxed);
                }
            }
        }
        args::Commands::List => {
            let final_open_ports = get_port();
            println!("\n=== PORT SCAN RESULTS ===");
            if final_open_ports.is_empty() {
                println!("No open ports found in range 1-9999.");
                return Ok(());
            } else {
                println!("Open ports found: {:?}", final_open_ports);
                return Ok(());
            }
        }
        args::Commands::Client(client_args) => {
            let bind_port = client_args.bind_port;
            let path = client_args.path;
            match (bind_port, path) {
                (Some(port), path) => {
                    if scan_port(port as usize) {
                        client(port, path).await.unwrap();
                        return Ok(());
                    } else {
                        println!(
                            "The port is not active check the server again or list the port and try again"
                        );
                        return Ok(());
                    };
                }
                (None, path) => {
                    let open_ports = get_port();
                    println!("\n=== PORT SCAN RESULTS ===");
                    if open_ports.is_empty() {
                        println!("No open ports found in range 1-9999.");
                        println!("Cannot start proxy without a backend service!");
                        return Ok(());
                    } else {
                        println!("Open ports found: {:?}", open_ports);
                        client(open_ports[0] as u16, path).await.unwrap();
                    };

                    if open_ports.len() > 1 {
                        println!("Total open ports: {}", open_ports.len());
                        println!(
                            "You have to manually specify the port that you want to use by -b <bind_port> flag"
                        );
                    }

                    return Ok(());
                }
            }
        }
        args::Commands::Host(host_args) => {
            let bind_port = host_args.bind_port;
            host(bind_port).await?;
            return Ok(());
        }
    }

    println!("\n=== PROXY SERVER INFO ===");
    show_pulsing();
    println!("======================================================");
    println!(
        "Proxy running on: 0.0.0.0:{}",
        bind_port_atomic.load(atomic::Ordering::Relaxed)
    );
    println!("======================================================");
    println!(
        "Forwarding requests to: http://localhost:{}",
        target_port_atomic.load(atomic::Ordering::Relaxed)
    );
    println!(
        "Backend service: http://localhost:{}",
        target_port_atomic.load(atomic::Ordering::Relaxed)
    );

    println!();
    println!("======================================================");
    println!(
        "You can access the proxy at: {}\n on your other devices connected to the same network",
        link.lock().unwrap()
    );
    println!("Here is the qr for you easy access");
    if let Err(e) = print_qr(link.lock().unwrap().to_string()) {
        eprintln!("Failed to print QR code: {}", e);
    }
    println!("Happy coding :) ");

    start_spinner();

    let client = Client::new();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .app_data(web::Data::new(Arc::clone(&target_port_atomic)))
            .default_service(web::route().to(proxy))
    })
    .bind(format!(
        "0.0.0.0:{}",
        bind_port_atomic.load(atomic::Ordering::Relaxed)
    ))?
    .run()
    .await
}
