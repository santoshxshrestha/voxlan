#![allow(unused)]
use tokio::net::TcpStream;
pub fn connect(target_port: u16, ip: String) {
    println!(
        "Hello there rustacean got the target_port:{} and ip:{}",
        target_port, ip
    );
}
