use image::DynamicImage;

const ASCII_CHARS: &[char] = &[' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];

pub fn image_to_ascii(img: &DynamicImage, width: u32) -> String {
    let aspect_ratio = img.height() as f32 / img.width() as f32;
    let height = (width as f32 * aspect_ratio * 0.5) as u32;

    let resized = img.resize_exact(
        width,
        height,
        image::imageops::FilterType::Lanczos3,
    );

    let gray = resized.to_luma8();
    let mut ascii_art = String::new();

    for y in 0..height {
        for x in 0..width {
            let pixel = gray.get_pixel(x, y)[0];
            let char_index = (pixel as usize * (ASCII_CHARS.len() - 1)) / 255;
            ascii_art.push(ASCII_CHARS[char_index]);
        }
        ascii_art.push('\n');
    }

    ascii_art
}
