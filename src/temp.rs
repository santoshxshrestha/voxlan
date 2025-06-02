use std::net::UdpSocket;

fn get_local_ip() -> std::io::Result<std::net::IpAddr> {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("couldn't bind to address");
    socket.connect("8.8.8.8:80")?;
    Ok(socket.local_addr()?.ip())
}

fn main() {
    match get_local_ip() {
        Ok(ip) => println!("Got you ip {ip}"),
        Err(e) => eprint!("Error couldn't get you ip : {}", e),
    }
}
