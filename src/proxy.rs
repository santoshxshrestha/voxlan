use std::sync::Arc;
use std::sync::atomic::{AtomicU16, Ordering};

use crate::Client;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::web;

pub async fn proxy(
    req: HttpRequest,
    body: web::Bytes,
    client: web::Data<Client>,
    target_port_atomic: web::Data<Arc<AtomicU16>>,
) -> HttpResponse {
    let port = target_port_atomic.load(Ordering::Relaxed);
    let backend_url = format!("http://localhost:{}{}", port, req.uri());
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

    // Copying headers from original request
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

            // Copying response headers
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
