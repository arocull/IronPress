use crate::util::Rgb16Image;
use image::{ImageBuffer, Rgb};

// Pack channels of an image
pub fn flip_green(texture: Rgb16Image) -> ImageBuffer<Rgb<u16>, Vec<u16>> {
    let (_width, _height) = texture.dimensions();

    // Prep new image
    let mut imgbuf = image::ImageBuffer::from(texture);
    // Fill out pixels of new image with data from inputs
    for (_x, _y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = Rgb([pixel[0], u16::MAX - pixel[1], pixel[2]]);
    }

    return imgbuf;
}
