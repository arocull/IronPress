extern crate image;

use image::{ImageBuffer, Rgb, ImageFormat};
use image::buffer::ConvertBuffer;
use std::path::{Path};
use crate::util;
use crate::util::Rgb16Image;

pub(crate) fn execute(paths: Vec<String>) {
    let mut texture: Rgb16Image = image::DynamicImage::new_rgb16(1, 1).into_rgb16();
    let mut out_path: &Path = Path::new("./out/flipped.png");

    // arguments 0 is input RGB
    // argument 1 is output
    for i in 0..paths.len() {
        if i == 0 {
            if paths[i].eq("_") { // If we're given a placeholder, input a blank image instead
                println!("Placeholders not allowed for channel flipping!");
                return;
            }
            texture = util::load_image(paths[i].as_str()).into_rgb16();
        } else if i == 1 {
            out_path = Path::new(paths[i].as_str());
        }
    }

    // Perform manipulation and save out
    let output: ImageBuffer<Rgb<u16>, Vec<u16>> = execute_internal(texture).convert();
    output.save_with_format(out_path, ImageFormat::Png).unwrap();
}

// Pack channels of an image
fn execute_internal(texture: Rgb16Image) -> ImageBuffer<Rgb<u16>, Vec<u16>> {
    let (_width, _height) = texture.dimensions();

    // Prep new image
    let mut imgbuf = image::ImageBuffer::from(texture);
    // Fill out pixels of new image with data from inputs
    for (_x, _y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = Rgb([pixel[0], u16::MAX - pixel[1], pixel[2]]);
    }

    return imgbuf;
}
