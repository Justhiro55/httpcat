use anyhow::{Context, Result};
use image::DynamicImage;
use reqwest::blocking::Client;

pub fn fetch_cat_image(status_code: u16) -> Result<DynamicImage> {
    let url = format!("https://http.cat/{}", status_code);

    let client = Client::new();
    let response = client
        .get(&url)
        .send()
        .context("Failed to fetch cat image")?;

    let bytes = response
        .bytes()
        .context("Failed to read cat image bytes")?;

    let img = image::load_from_memory(&bytes)
        .context("Failed to decode cat image")?;

    Ok(img)
}

pub fn display_image(img: &DynamicImage, width: u32) -> Result<()> {
    let config = viuer::Config {
        width: Some(width),
        absolute_offset: false,
        ..Default::default()
    };

    viuer::print(img, &config)
        .context("Failed to display image in terminal")?;

    Ok(())
}
