use crate::http::HttpResponse;
use colored::*;

pub fn print_request_info(url: &str) {
    println!("\n🌐 Requesting: {}\n", url.cyan());
    println!("⏳ Loading...\n");
}

pub fn print_response_status(response: &HttpResponse) {
    let colored_status = if response.status >= 200 && response.status < 300 {
        format!("✅ {}", response.status_text).green()
    } else if response.status >= 300 && response.status < 400 {
        format!("➡️  {}", response.status_text).yellow()
    } else if response.status >= 400 && response.status < 500 {
        format!("❌ {}", response.status_text).red()
    } else if response.status >= 500 {
        format!("💥 {}", response.status_text).bright_red()
    } else {
        format!("ℹ️  {}", response.status_text).white()
    };

    println!("{}\n", colored_status);
}

pub fn print_cat_header() {
    println!("🐱 Here's your cat:\n");
}

pub fn print_response_time(duration_ms: u64) {
    println!("\n⏱️  Response time: {}ms\n", duration_ms.to_string().bright_blue());
}

pub fn print_headers(response: &HttpResponse) {
    println!("\n📋 Response Headers:");
    println!("{}", "=".repeat(50));
    for (name, value) in response.headers.iter() {
        println!("{}: {}", name.as_str().yellow(), value.to_str().unwrap_or(""));
    }
    println!("{}\n", "=".repeat(50));
}

pub fn print_body(response: &HttpResponse) {
    println!("\n📄 Response Body:");
    println!("{}", "=".repeat(50));
    println!("{}", response.body);
    println!("{}\n", "=".repeat(50));
}
