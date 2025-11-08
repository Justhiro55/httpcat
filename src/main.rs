mod ascii;
mod cat;
mod cli;
mod display;
mod http;

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let args = cli::Cli::parse();

    // Display request info
    display::print_request_info(&args.url);

    // Send HTTP request
    let response = http::send_request(&args.url, &args.method, &args.request_headers)?;

    // Display response status
    display::print_response_status(&response);

    // Fetch and display cat image (unless disabled)
    if !args.no_image {
        display::print_cat_header();

        match cat::fetch_cat_image(response.status) {
            Ok(img) => {
                if args.ascii {
                    let ascii_art = ascii::image_to_ascii(&img, args.size);
                    println!("{}", ascii_art);
                } else {
                    cat::display_image(&img, args.size)?;
                }
            }
            Err(e) => {
                eprintln!("⚠️  Could not fetch cat image: {}", e);
            }
        }
    }

    // Display response time
    display::print_response_time(response.duration.as_millis() as u64);

    // Display headers if requested
    if args.headers {
        display::print_headers(&response);
    }

    // Display body if requested
    if args.body {
        display::print_body(&response);
    }

    Ok(())
}
