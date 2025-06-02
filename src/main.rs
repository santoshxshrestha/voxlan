use std::{net::TcpStream, thread, time::Duration};

use std::net::UdpSocket;

fn get_local_ip() -> std::io::Result<std::net::IpAddr> {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("couldn't bind to address");
    socket.connect("8.8.8.8:80")?;
    Ok(socket.local_addr()?.ip())
}

fn scan_port(ip: &str, port: u16) {
    let addr = format!("{}:{}", ip, port);
    if TcpStream::connect_timeout(&addr.parse().unwrap(), Duration::from_millis(100)).is_ok() {
        println!("Port {} is open", port);
    }
}

fn main() {
    match get_local_ip() {
        Ok(ip) => println!("Got you ip {ip}"),
        Err(e) => eprint!("Error couldn't get you ip : {}", e),
    }
    let ip = "127.0.0.1";
    let mut handles = vec![];

    for port in 1..10000 {
        let ip = ip.to_string();
        handles.push(thread::spawn(move || {
            scan_port(&ip, port);
        }));
    }

    for handle in handles {
        let _ = handle.join();
    }
}
