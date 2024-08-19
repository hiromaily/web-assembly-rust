use image::{ImageFormat, Rgb, RgbImage, Rgba, RgbaImage};
use std::io::Cursor;
use wasm_bindgen::prelude::*;

// Export a `generate_image` function from Rust to JavaScript,
// that generates image.
#[wasm_bindgen]
pub fn generate_image(r: u8, g: u8, b: u8, a: u8) -> Vec<u8> {
    let block_size = 16; // this size changes generated image size
    let obj_width = block_size * 21;
    let obj_height = block_size * 21;

    // Define the RGBA color value
    let col = Rgba([r, g, b, a]);

    // Create a 100x100 pixel image with a solid color
    //let mut img = RgbImage::new(100, 100);
    //fixed_color(&mut img);
    //rgb_color(&mut img, r, g, b);

    let mut img = RgbaImage::new(obj_width, obj_height);
    Config::new(block_size, obj_width, obj_height).rgba_hy(&mut img, col);

    // Convert the image to PNG format
    let mut buffer = Cursor::new(Vec::new());
    img.write_to(&mut buffer, ImageFormat::Png).unwrap();

    // Return the PNG image as a byte array (Vec<u8>)
    buffer.into_inner()
}

#[allow(dead_code)]
fn fixed_color(img: &mut RgbImage) {
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let color = if (x + y) % 2 == 0 {
            Rgb([255, 0, 0])
        } else {
            Rgb([0, 255, 0])
        };
        *pixel = color;
    }
}

#[allow(dead_code)]
fn rgb_color(img: &mut RgbImage, r: u8, g: u8, b: u8) {
    for (_, _, pixel) in img.enumerate_pixels_mut() {
        *pixel = Rgb([r, g, b]);
    }
}

struct Config {
    block_size: u32,
    width: u32,
    height: u32,
}
impl Config {
    fn new(block_size: u32, width: u32, height: u32) -> Self {
        Self {
            block_size,
            width,
            height,
        }
    }

    fn rgba_hy(&self, img: &mut RgbaImage, col: Rgba<u8>) {
        // for (_, _, pixel) in img.enumerate_pixels_mut() {
        //     *pixel = Rgba([r, g, b, a]);
        // }

        for y in 0..self.height {
            for x in 0..self.width {
                let draw_flg = match () {
                    // Outline conditions
                    _ if y < self.block_size => true,
                    _ if y >= (self.block_size * 20) && y < (self.block_size * 21) => true,
                    _ if x < self.block_size => true,
                    _ if x >= (self.block_size * 20) && x < (self.block_size * 21) => true,

                    // H shape conditions
                    _ if x >= (self.block_size * 2)
                        && x < (self.block_size * 6)
                        && y >= (self.block_size * 2)
                        && y < (self.block_size * 19) =>
                    {
                        true
                    }
                    _ if x >= (self.block_size * 6)
                        && x < (self.block_size * 10)
                        && y >= (self.block_size * 11)
                        && y < (self.block_size * 14) =>
                    {
                        true
                    }
                    _ if x >= (self.block_size * 10)
                        && x < (self.block_size * 14)
                        && y >= (self.block_size * 11)
                        && y < (self.block_size * 19) =>
                    {
                        true
                    }

                    // Y shape conditions
                    _ if x >= (self.block_size * 7)
                        && x < (self.block_size * 11)
                        && y >= (self.block_size * 2)
                        && y < (self.block_size * 10) =>
                    {
                        true
                    }
                    _ if x >= (self.block_size * 11)
                        && x < (self.block_size * 15)
                        && y >= (self.block_size * 7)
                        && y < (self.block_size * 10) =>
                    {
                        true
                    }
                    _ if x >= (self.block_size * 15)
                        && x < (self.block_size * 19)
                        && y >= (self.block_size * 2)
                        && y < (self.block_size * 19) =>
                    {
                        true
                    }

                    _ => false,
                };

                if draw_flg {
                    img.put_pixel(x, y, col);
                }
            }
        }
    }
}
