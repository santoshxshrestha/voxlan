use std::env;
use std::io;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::time::timeout;

#[derive(Debug)]
struct ProxyConfig {
    bind_addr: String,
    bind_port: u16,
    target_host: String,
    target_port: u16,
    timeout_seconds: u64,
}

impl Default for ProxyConfig {
    fn default() -> Self {
        Self {
            bind_addr: "0.0.0.0".to_string(),
            bind_port: 8080,
            target_host: "127.0.0.1".to_string(),
            target_port: 8080,
            timeout_seconds: 30,
        }
    }
}

impl ProxyConfig {
    fn from_env() -> Self {
        let mut config = Self::default();

        if let Ok(addr) = env::var("PROXY_BIND_ADDR") {
            config.bind_addr = addr;
        }

        if let Ok(port) = env::var("PROXY_BIND_PORT") {
            if let Ok(port) = port.parse() {
                config.bind_port = port;
            }
        }

        if let Ok(host) = env::var("PROXY_TARGET_HOST") {
            config.target_host = host;
        }

        if let Ok(port) = env::var("PROXY_TARGET_PORT") {
            if let Ok(port) = port.parse() {
                config.target_port = port;
            }
        }

        if let Ok(timeout) = env::var("PROXY_TIMEOUT") {
            if let Ok(timeout) = timeout.parse() {
                config.timeout_seconds = timeout;
            }
        }

        config
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let config = ProxyConfig::from_env();

    println!("Starting proxy server with configuration:");
    println!("  Bind: {}:{}", config.bind_addr, config.bind_port);
    println!("  Target: {}:{}", config.target_host, config.target_port);
    println!("  Timeout: {}s", config.timeout_seconds);

    let bind_addr = format!("{}:{}", config.bind_addr, config.bind_port);
    let listener = TcpListener::bind(&bind_addr).await?;

    println!("\n✅ Proxy server listening on {}", bind_addr);

    if config.bind_addr == "0.0.0.0" {
        print_network_info(config.bind_port).await;
    }

    let mut connection_count = 0u64;

    loop {
        match listener.accept().await {
            Ok((client_stream, client_addr)) => {
                connection_count += 1;
                let conn_id = connection_count;

                println!("[{}] New connection from: {}", conn_id, client_addr);

                let config = ProxyConfig::from_env(); // Re-read config for each connection
                tokio::spawn(async move {
                    let result = handle_client(client_stream, client_addr, config, conn_id).await;
                    match result {
                        Ok(_) => println!("[{}] Connection closed normally", conn_id),
                        Err(e) => eprintln!("[{}] Connection error: {}", conn_id, e),
                    }
                });
            }
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        }
    }
}

async fn handle_client(
    client_stream: TcpStream,
    client_addr: std::net::SocketAddr,
    config: ProxyConfig,
    conn_id: u64,
) -> io::Result<()> {
    let target_addr = format!("{}:{}", config.target_host, config.target_port);
    let timeout_duration = Duration::from_secs(config.timeout_seconds);

    // Connect to target with timeout
    let target_stream = timeout(timeout_duration, TcpStream::connect(&target_addr)).await;

    let mut target_stream = match target_stream {
        Ok(Ok(stream)) => {
            println!("[{}] Connected to target {}", conn_id, target_addr);
            stream
        }
        Ok(Err(e)) => {
            eprintln!(
                "[{}] Failed to connect to target {}: {}",
                conn_id, target_addr, e
            );
            return Err(e);
        }
        Err(_) => {
            eprintln!("[{}] Timeout connecting to target {}", conn_id, target_addr);
            return Err(io::Error::new(
                io::ErrorKind::TimedOut,
                "Connection timeout",
            ));
        }
    };

    // Set TCP keepalive and nodelay for better performance
    if let Ok(()) = client_stream.set_nodelay(true) {
        // Nodelay set successfully
    }
    if let Ok(()) = target_stream.set_nodelay(true) {
        // Nodelay set successfully
    }

    // Split streams for bidirectional data transfer
    let (mut client_read, mut client_write) = client_stream.into_split();
    let (mut target_read, mut target_write) = target_stream.into_split();

    // Forward data bidirectionally with timeout
    let client_to_target = async {
        let mut buffer = vec![0u8; 8192];
        let mut total_bytes = 0u64;

        loop {
            match timeout(timeout_duration, client_read.read(&mut buffer)).await {
                Ok(Ok(0)) => break, // EOF
                Ok(Ok(n)) => {
                    if let Err(e) = target_write.write_all(&buffer[..n]).await {
                        eprintln!("[{}] Error writing to target: {}", conn_id, e);
                        break;
                    }
                    total_bytes += n as u64;
                }
                Ok(Err(e)) => {
                    eprintln!("[{}] Error reading from client: {}", conn_id, e);
                    break;
                }
                Err(_) => {
                    eprintln!("[{}] Timeout reading from client", conn_id);
                    break;
                }
            }
        }

        println!(
            "[{}] Client -> Target: {} bytes total",
            conn_id, total_bytes
        );
        let _ = target_write.shutdown().await;
    };

    let target_to_client = async {
        let mut buffer = vec![0u8; 8192];
        let mut total_bytes = 0u64;

        loop {
            match timeout(timeout_duration, target_read.read(&mut buffer)).await {
                Ok(Ok(0)) => break, // EOF
                Ok(Ok(n)) => {
                    if let Err(e) = client_write.write_all(&buffer[..n]).await {
                        eprintln!("[{}] Error writing to client: {}", conn_id, e);
                        break;
                    }
                    total_bytes += n as u64;
                }
                Ok(Err(e)) => {
                    eprintln!("[{}] Error reading from target: {}", conn_id, e);
                    break;
                }
                Err(_) => {
                    eprintln!("[{}] Timeout reading from target", conn_id);
                    break;
                }
            }
        }

        println!(
            "[{}] Target -> Client: {} bytes total",
            conn_id, total_bytes
        );
        let _ = client_write.shutdown().await;
    };

    // Run both directions concurrently
    tokio::select! {
        _ = client_to_target => {},
        _ = target_to_client => {},
    }

    Ok(())
}

async fn print_network_info(port: u16) {
    println!("🌐 Network Access Information:");
    println!("   Other devices on your network can connect to:");

    if let Some(ip) = get_local_ip().await {
        println!("   📱 http://{}:{}", ip, port);
        println!(
            "   📱 https://{}:{} (if target service supports HTTPS)",
            ip, port
        );
    }

    println!("   💻 localhost: http://127.0.0.1:{}", port);
    println!();
    println!(
        "💡 Tip: Make sure your firewall allows connections on port {}",
        port
    );
    println!("💡 Tip: Set environment variables to customize:");
    println!("   PROXY_BIND_ADDR=0.0.0.0");
    println!("   PROXY_BIND_PORT=8080");
    println!("   PROXY_TARGET_HOST=127.0.0.1");
    println!("   PROXY_TARGET_PORT=8080");
    println!("   PROXY_TIMEOUT=30");
    println!();
}

async fn get_local_ip() -> Option<String> {
    use std::net::UdpSocket;

    // Use UDP to determine the local IP that would be used to reach the internet
    if let Ok(socket) = UdpSocket::bind("0.0.0.0:0") {
        if socket.connect("1.1.1.1:80").is_ok() {
            if let Ok(addr) = socket.local_addr() {
                let ip = addr.ip().to_string();
                if !ip.starts_with("127.") {
                    return Some(ip);
                }
            }
        }
    }
    None
}
