use image::{ImageFormat, Rgb, RgbImage};
use std::io::Cursor;
use wasm_bindgen::prelude::*;

// Export a `generate_image` function from Rust to JavaScript,
// that generates image.
#[wasm_bindgen]
pub fn generate_image() -> Vec<u8> {
    // Create a 100x100 pixel image with a solid color
    let mut img = RgbImage::new(100, 100);
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let color = if (x + y) % 2 == 0 {
            Rgb([255, 0, 0])
        } else {
            Rgb([0, 255, 0])
        };
        *pixel = color;
    }

    // Convert the image to PNG format
    let mut buffer = Cursor::new(Vec::new());
    img.write_to(&mut buffer, ImageFormat::Png).unwrap();

    // Return the PNG image as a byte array (Vec<u8>)
    buffer.into_inner()
}
