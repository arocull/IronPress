extern crate image;

use std::cmp::min;
use std::env;
use std::ptr::null;
use image::{GenericImageView, ImageBuffer, RgbImage, imageops, DynamicImage, Pixels, RgbaImage, Rgba};
use image::buffer::ConvertBuffer;

fn main() {
    let args: Vec<_> = env::args().collect();

    let mut channels = [true, true, true, true];
    let mut images: Vec<DynamicImage> = Vec::new();
    // let dim: Vec<(u32, u32)> = Vec::new();
    let mut out_path = "";

    // let img = load_image(PATH);

    // First argument is where the executable is
    // arguments 1-4 are channels RGBA
    // argument 5 is output
    for i in 1..args.len() {
        println!("arg {0}, {1}", i, args[i]);

        if i > 0 && i < 5 {
            if args[i].eq("_") {
                channels[i - 1] = false;
            } else {
                images.push(load_image(args[i].as_str()));
            }
        } else if i == 6 {
            out_path = args[i].as_str();
        }
    }

    let output = pack_channels(channels, images.clone());
    let output: RgbaImage = output.convert();
    output.save(out_path).unwrap();
}

fn load_image(path: &str) -> DynamicImage {
    return image::open(path).unwrap();
}

fn pack_channels(channels: [bool;4], mut channelData: Vec<DynamicImage>) -> ImageBuffer<Rgba<f32>, Vec<f32>> {
    let mut width= 1024;
    let mut height = 1024;
    for i in 0..channelData.len() {
        let (x1, y1) = channelData[i].dimensions();
        if i == 0 { // Define image bounds
            width = x1;
            height = y1;
        } else { // Make sure all image dimensions match up
            let (x2, y2) = channelData[i - 1].dimensions();
            assert_eq!(x1, x2, "Input image widths did not match");
            assert_eq!(y2, y2, "Input image heights did not match");
        }
    }
    // Lock width and height
    let width = width;
    let height = height;

    // Prep new image
    let mut imgbuf = image::ImageBuffer::new(width, height);

    // Edit channels
    let mut channel_idx = 0;

    // Declare and fill out red channel
    let mut r_chan: DynamicImage;
    if channels[0] { // If we have an image for it, load it
        r_chan = channelData[channel_idx];
        channel_idx += 1;
    } else { // Otherwise, fill it out with a blank image
        r_chan = image::DynamicImage::new_rgba32f(width, height);
    }

    // Declare and fill out red channel
    let mut g_chan: DynamicImage;
    if channels[1] { // If we have an image for it, load it
        g_chan = channelData[channel_idx];
        channel_idx += 1;
    } else { // Otherwise, fill it out with a blank image
        g_chan = image::DynamicImage::new_rgba32f(width, height);
    }

    // Declare and fill out red channel
    let mut b_chan: DynamicImage;
    if channels[2] { // If we have an image for it, load it
        b_chan = channelData[channel_idx];
        channel_idx += 1;
    } else { // Otherwise, fill it out with something
        b_chan = image::DynamicImage::new_rgba32f(width, height);
    }

    // Declare and fill out alpha channel
    let mut a_chan: DynamicImage;
    if channels[3] { // If we have an image for it, load it
        a_chan = channelData[channel_idx];
        channel_idx += 1;
    } else { // Otherwise, fill it out with something
        a_chan = image::DynamicImage::new_rgba32f(width, height);
    }

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = r_chan.get_pixel(x, y).0[0] as f32;
        let g = g_chan.get_pixel(x, y).0[1] as f32;
        let b = b_chan.get_pixel(x, y).0[2] as f32;
        let mut a = 1 as u8;
        if channels[3] {
            a = a_chan.get_pixel(x, y).0[3];
        }

        *pixel = image::Rgba([r, g, b, a.into()]);
    }

    return imgbuf;
}
