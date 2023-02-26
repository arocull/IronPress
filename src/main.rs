extern crate image;

use std::env;
use image::{ImageBuffer, DynamicImage, Rgb, Rgba, ImageFormat, Rgba32FImage};
use image::buffer::ConvertBuffer;
use std::path::{Path};

mod channel_pack;

fn main() {
    let args: Vec<_> = env::args().collect();

    let mut args_packed: Vec<String> = Vec::new();
    for i in 1..args.len() {
        args_packed.push(args[i].clone());
    }

    channel_pack::execute(args_packed, 2048, 2048);
}

// Load an image into RAM
fn load_image(path: &str) -> DynamicImage {
    return image::open(path).unwrap();
}
