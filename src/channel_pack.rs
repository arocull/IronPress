extern crate image;

use std::env;
use image::{ImageBuffer, DynamicImage, Rgb, Rgba, ImageFormat, Rgba32FImage};
use image::buffer::ConvertBuffer;
use std::path::{Path};

pub(crate) fn execute(paths: Vec<String>, width: u32, height: u32) {
    let mut images: Vec<Rgba32FImage> = Vec::new();
    let mut out_path: &Path = Path::new("./out/pack.png");

    let mut use_alpha = false;

    // arguments 0-3 are channels RGBA
    // argument 4 is output
    for i in 0..paths.len() {
        if i < 4 {
            if paths[i].eq("_") { // If we're given a placeholder, input a blank image instead
                images.push(image::DynamicImage::new_rgba32f(width, height).into_rgba32f());
            } else {
                println!("...loading {}", paths[i]);
                images.push(image::open(paths[i].as_str()).unwrap().into_rgba32f());
                if i == 3 {
                    use_alpha = true;
                }
            }
        } else if i == 4 {
            out_path = Path::new(paths[i].as_str());
        }
    }

    // Perform channel packing
    let output = channel_pack_internal(images, use_alpha);

    // When outputting image, we compress to 16-bit channels, as we need a high range of value
    // ...but most programs don't work in 32-bit, so we don't need that extreme either
    if use_alpha { // Output image with an alpha channel, if one was used
        let output: ImageBuffer<Rgba<u16>, Vec<u16>> = output.convert();
        output.save_with_format(out_path, ImageFormat::Png).unwrap();
    } else { // Otherwise, output without alpha
        let output: ImageBuffer<Rgb<u16>, Vec<u16>> = output.convert();
        output.save_with_format(out_path, ImageFormat::Png).unwrap();
    }
}

// Pack channels of an image
fn channel_pack_internal(channel_data: Vec<Rgba32FImage>, use_alpha: bool) -> ImageBuffer<Rgba<f32>, Vec<f32>> {
    let mut width= 1024;
    let mut height = 1024;
    for i in 0..channel_data.len() {
        let (x1, y1) = channel_data[i].dimensions();
        if i == 0 { // Define image bounds
            width = x1;
            height = y1;
        } else { // Make sure all image dimensions match up
            let (x2, y2) = channel_data[i - 1].dimensions();
            assert_eq!(x1, x2, "Input image widths did not match");
            assert_eq!(y2, y2, "Input image heights did not match");
        }
    }
    // Lock width and height
    let width = width;
    let height = height;

    // Prep new image
    let mut imgbuf = image::ImageBuffer::new(width, height);
    // Fill out pixels of new image with data from inputs
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = channel_data[0].get_pixel(x, y).0[0] as f32;
        let g = channel_data[1].get_pixel(x, y).0[1] as f32;
        let b = channel_data[2].get_pixel(x, y).0[2] as f32;
        let mut a = channel_data[3].get_pixel(x, y).0[3] as u8;
        if !use_alpha { // If we're not using alpha, maximize the alpha channel
            a = 255 as u8;
        }

        *pixel = image::Rgba([r, g, b, a.into()]);
    }

    return imgbuf;
}
