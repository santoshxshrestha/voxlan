use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use reqwest::Client;
use std::net::{TcpStream, UdpSocket};
use std::time::Duration;

fn get_local_ip() -> Option<String> {
    let socket = UdpSocket::bind("0.0.0.0:0").ok()?;
    socket.connect("8.8.8.8:80").ok()?;
    let ip = socket.local_addr().ok()?.ip();
    Some(ip.to_string())
}

fn scan_port(ip: &str, port: u16) -> Option<u16> {
    let addr = format!("{}:{}", ip, port);
    if let Ok(addr) = addr.parse() {
        if TcpStream::connect_timeout(&addr, Duration::from_millis(100)).is_ok() {
            Some(port)
        } else {
            None
        }
    } else {
        None
    }
}

async fn proxy(req: HttpRequest, body: web::Bytes, client: web::Data<Client>) -> HttpResponse {
    let backend_url = format!("http://localhost:8080{}", req.uri());

    let method = match req.method().as_str() {
        "GET" => reqwest::Method::GET,
        "POST" => reqwest::Method::POST,
        "PUT" => reqwest::Method::PUT,
        "DELETE" => reqwest::Method::DELETE,
        "HEAD" => reqwest::Method::HEAD,
        "OPTIONS" => reqwest::Method::OPTIONS,
        "PATCH" => reqwest::Method::PATCH,
        _ => reqwest::Method::GET,
    };
    let mut request_builder = client.request(method, &backend_url);

    for (key, value) in req.headers().iter() {
        if let Ok(value_str) = value.to_str() {
            request_builder = request_builder.header(key.as_str(), value_str);
        }
    }

    let request_builder = request_builder.body(body.to_vec());

    match request_builder.send().await {
        Ok(response) => {
            let status = response.status();
            let headers = response.headers().clone();
            let bytes = response.bytes().await.unwrap_or_default();

            let mut client_response = HttpResponse::build(
                actix_web::http::StatusCode::from_u16(status.as_u16())
                    .unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR),
            );

            // Copy response headers manually
            for (key, value) in headers.iter() {
                if let Ok(header_value) =
                    actix_web::http::header::HeaderValue::from_bytes(value.as_bytes())
                {
                    client_response.append_header((key.as_str(), header_value));
                }
            }

            client_response.body(bytes.to_vec())
        }
        Err(e) => {
            eprintln!("Failed to forward request: {}", e);
            HttpResponse::InternalServerError().body("Failed to forward request")
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let ip = "127.0.0.1";
    let mut handles = vec![];

    for port in 1..5000 {
        let ip = ip.to_string();
        handles.push(std::thread::spawn(move || scan_port(&ip, port)));
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
            "Found open port {}. Proxy will run on http://{}:8081",
            first_port,
            get_local_ip().unwrap_or_else(|| "localhost".to_string())
        );
    } else {
        println!("No open ports found in range 1-999.");
    }

    let client = Client::new();

    println!("Starting proxy server on 0.0.0.0:8081");
    println!("Forwarding requests to http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .default_service(web::route().to(proxy))
    })
    .bind("0.0.0.0:8081")?
    .run()
    .await
}
