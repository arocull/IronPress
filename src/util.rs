extern crate image;

use std::path::{Path, PathBuf};
use image::{ColorType, DynamicImage, GenericImageView, ImageBuffer, ImageEncoder, imageops, Luma, Rgb};
use std::fs::{File};
use std::io::BufWriter;
use image::codecs::png;

pub(crate) type Gray16Image = ImageBuffer<Luma<u16>, Vec<u16>>;
// pub(crate) type Rgba16Image = ImageBuffer<Rgba<u16>, Vec<u16>>;
pub(crate) type Rgb16Image = ImageBuffer<Rgb<u16>, Vec<u16>>;

// Load an image into RAM
pub(crate) fn load_image(path: &Path) -> DynamicImage {
    return image::open(path).unwrap();
}

pub(crate) fn int16_to_float64(a: u16) -> f64 {
    return (a as f64) / (u16::MAX as f64);
}

pub(crate) fn auto_resize(img: DynamicImage, width: u32, height: u32) -> DynamicImage {
    let (dim_x, dim_y) = img.dimensions();

    // Pick resizing filter based off of what we're doing, upscaling versus downscaling
    // https://stackoverflow.com/questions/384991/what-is-the-best-image-downscaling-algorithm-quality-wise

    if dim_x > width || dim_y > height { // If we're upscaling an image, use Catmull Rom
        return img.resize(width, height, imageops::FilterType::CatmullRom);
    } else if dim_x != width || dim_y != height { // If we're downscaling an image, use Lanczos
        return img.resize(width, height, imageops::FilterType::Lanczos3);
    }

    return  img;
}

// Saves an image buffer to the given path, using a specific color format, at the best compression
pub(crate) fn compressed_save(path: &Path, buffer: &[u8], width: u32, height: u32, format: ColorType) {
    let f = File::create(path).unwrap(); // Create a file at the given path
    let writer = BufWriter::new(f); // Create a writer buffer to it
    // Set up a PNG encoder on top of the buffer, and attempt to maximize compression (for space efficiency)
    let encoder = png::PngEncoder::new_with_quality(writer, png::CompressionType::Best, png::FilterType::Adaptive);
    encoder.write_image(buffer, width, height, format).unwrap(); // Finally, write out the image
}

pub(crate) fn path_material_map(directory: &Path, material_name: &str, data: &str, format: &str) -> PathBuf {
    // Concatenate strings together
    let mut owned_str: String = material_name.clone().to_string();
    owned_str.push_str("_");
    owned_str.push_str(data);
    owned_str.push_str(".");
    owned_str.push_str(format);

    return directory.join(Path::new(owned_str.as_str()));
}