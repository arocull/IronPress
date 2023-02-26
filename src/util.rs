extern crate image;

use image::{DynamicImage, ImageBuffer, Luma, Rgb, Rgba};

pub(crate) type Gray16Image = ImageBuffer<Luma<u16>, Vec<u16>>;
pub(crate) type Rgba16Image = ImageBuffer<Rgba<u16>, Vec<u16>>;
pub(crate) type Rgb16Image = ImageBuffer<Rgb<u16>, Vec<u16>>;

// Load an image into RAM
pub(crate) fn load_image(path: &str) -> DynamicImage {
    return image::open(path).unwrap();
}

pub(crate) fn int16_to_float64(a: u16) -> f64 {
    return (a as f64) / (u16::MAX as f64);
}
