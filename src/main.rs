use std::net::{TcpStream, UdpSocket};
use std::{thread, time::Duration};

fn get_local_ip() -> Option<String> {
    let socket = UdpSocket::bind("0.0.0.0:0").ok()?;
    socket.connect("8.8.8.8:80").ok()?;
    let ip = socket.local_addr().ok()?.ip();
    Some(ip.to_string())
}

fn scan_port(ip: &str, port: u16) -> Option<u16> {
    let addr = format!("{}:{}", ip, port);
    if TcpStream::connect_timeout(&addr.parse().unwrap(), Duration::from_millis(100)).is_ok() {
        Some(port)
    } else {
        None
    }
}

fn main() {
    let ip = "127.0.0.1";
    let mut handles = vec![];

    for port in 1..10000 {
        let ip = ip.to_string();
        handles.push(thread::spawn(move || scan_port(&ip, port)));
    }

    let mut open_ports = Vec::new();

    for handle in handles {
        if let Ok(Some(open_port)) = handle.join() {
            println!("Port {} is open", open_port);
            open_ports.push(open_port);
        }
    }

    if let Some(first_port) = open_ports.first() {
        println!(
            "Go to link http://{}:{}",
            get_local_ip().unwrap(),
            first_port
        );
    } else {
        println!("No open ports found.");
    }
}
