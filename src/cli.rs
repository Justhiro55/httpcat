use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "httpcat")]
#[command(about = "HTTP request tool with cat images based on status codes", long_about = None)]
pub struct Cli {
    /// Target URL to send HTTP request
    pub url: String,

    /// HTTP method to use
    #[arg(short = 'X', long, default_value = "GET")]
    pub method: String,

    /// Image width in terminal
    #[arg(long, default_value_t = 60)]
    pub size: u32,

    /// Display image as ASCII art
    #[arg(long)]
    pub ascii: bool,

    /// Don't display cat image
    #[arg(long)]
    pub no_image: bool,

    /// Show response headers
    #[arg(long)]
    pub headers: bool,

    /// Show response body
    #[arg(long)]
    pub body: bool,

    /// Add request header (can be used multiple times)
    #[arg(short = 'H', long = "header")]
    pub request_headers: Vec<String>,
}
