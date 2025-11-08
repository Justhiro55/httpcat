use anyhow::{Context, Result};
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::time::{Duration, Instant};

pub struct HttpResponse {
    pub status: u16,
    pub status_text: String,
    pub headers: HeaderMap,
    pub body: String,
    pub duration: Duration,
}

pub fn send_request(url: &str, method: &str, request_headers: &[String]) -> Result<HttpResponse> {
    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .context("Failed to create HTTP client")?;

    let mut headers = HeaderMap::new();
    for header in request_headers {
        let parts: Vec<&str> = header.splitn(2, ':').collect();
        if parts.len() == 2 {
            let name = HeaderName::from_bytes(parts[0].trim().as_bytes())
                .context("Invalid header name")?;
            let value = HeaderValue::from_str(parts[1].trim()).context("Invalid header value")?;
            headers.insert(name, value);
        }
    }

    let start = Instant::now();

    let response = match method.to_uppercase().as_str() {
        "GET" => client.get(url),
        "POST" => client.post(url),
        "PUT" => client.put(url),
        "DELETE" => client.delete(url),
        "HEAD" => client.head(url),
        "PATCH" => client.patch(url),
        _ => anyhow::bail!("Unsupported HTTP method: {}", method),
    }
    .headers(headers)
    .send()
    .context("Failed to send HTTP request")?;

    let duration = start.elapsed();
    let status = response.status().as_u16();
    let status_text = response.status().to_string();
    let response_headers = response.headers().clone();
    let body = response.text().unwrap_or_default();

    Ok(HttpResponse {
        status,
        status_text,
        headers: response_headers,
        body,
        duration,
    })
}
