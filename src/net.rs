use std::net::{TcpStream, UdpSocket};
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub fn get_local_ip() -> Option<String> {
    let socket = UdpSocket::bind("0.0.0.0:0").ok()?;
    socket.connect("8.8.8.8:80").ok()?;
    let ip = socket.local_addr().ok()?.ip();
    Some(ip.to_string())
}

pub fn scan_port(ip: &str, port: usize) -> bool {
    let addr = format!("{}:{}", ip, port);
    TcpStream::connect_timeout(&addr.parse().unwrap(), Duration::from_millis(100)).is_ok()
}

pub fn get_port() -> Vec<usize> {
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
    return final_open_ports;
}
