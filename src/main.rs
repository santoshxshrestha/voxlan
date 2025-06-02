use std::{net::TcpStream, thread, time::Duration};

fn scan_port(ip: &str, port: u16) {
    let addr = format!("{}:{}", ip, port);
    if TcpStream::connect_timeout(&addr.parse().unwrap(), Duration::from_millis(100)).is_ok() {
        println!("Port {} is open", port);
    }
}

fn main() {
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
