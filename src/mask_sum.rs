extern crate image;

use image::{Luma, ImageFormat};
use image::buffer::ConvertBuffer;
use std::path::{Path};
use std::process::exit;

use crate::util;
use util::{Gray16Image, int16_to_float64};

pub(crate) fn execute(paths: Vec<String>, width: u32, height: u32) {
    let mut images: Vec<Gray16Image> = Vec::new();
    let mut out_path: &Path = Path::new("./out/mask.png");

    // arguments 0 and 1 are masks
    // argument 2 is output
    for i in 0..paths.len() {
        if i < 2 {
            if paths[i].eq("_") { // If we're given a placeholder, input a blank image instead
                println!("Placeholder images not permitted for masking!");
                exit(1);
            }
            println!("...loading {}", paths[i]);
            images.push(image::open(paths[i].as_str()).unwrap().into_luma16());
        } else if i == 2 {
            out_path = Path::new(paths[i].as_str());
        }
    }

    let mut weight: f64 = 0 as f64;
    let mut sum: f64 = 0 as f64;

    let mut imgbuf = image::ImageBuffer::new(width, height);
    // Fill out pixels of new image with data from inputs
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let mask = int16_to_float64(images[0].get_pixel(x, y).0[0]);
        let overlay = int16_to_float64(images[1].get_pixel(x, y).0[0]);

        weight += mask;
        sum += overlay * mask;

        // println!("mask {0} | {1}\toverlay {2} | {3}\ttotal {4}", images[0].get_pixel(x, y).0[0], mask, images[1].get_pixel(x, y).0[0], overlay, mask * overlay);
        *pixel = Luma([((overlay * mask) * (u16::MAX as f64)) as u16]);
    }

    println!("Resulting power is {}", sum / weight);
    let output: Gray16Image = imgbuf.convert();
    output.save_with_format(out_path, ImageFormat::Png).unwrap();
}