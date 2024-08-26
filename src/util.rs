extern crate image;

use image::codecs::png;
use image::{
    imageops, ColorType, DynamicImage, GenericImageView, ImageBuffer, ImageEncoder, Luma, Rgb,
};
use std::cmp::max;
use std::fs::File;
use std::io::BufWriter;
use std::path::{Path, PathBuf};

pub(crate) type Gray16Image = ImageBuffer<Luma<u16>, Vec<u16>>;
// pub(crate) type Rgba16Image = ImageBuffer<Rgba<u16>, Vec<u16>>;
pub(crate) type Rgb16Image = ImageBuffer<Rgb<u16>, Vec<u16>>;

/// Load an image into RAM.
/// TODO: Provide an empty, default image if opening fails.
pub(crate) fn load_image(path: &Path) -> DynamicImage {
    return image::open(path).unwrap();
}

/// Loads an image from the given filepath, converting it to the specified color format.
pub(crate) fn load_image_adv(
    path: &Path,
    res: u32,
    convert_to: ColorType,
) -> (DynamicImage, u32, u32) {
    let img_result = image::open(path);
    if img_result.as_ref().is_err() {
        eprintln!(
            "Failed to open image at path {0}, got error {1}",
            path.to_str().unwrap(),
            img_result.as_ref().unwrap_err().to_string()
        );
    }
    let img = img_result.unwrap();

    let map = match convert_to {
        ColorType::Rgb8 => DynamicImage::from(img.into_rgb8()),
        ColorType::Rgb16 => DynamicImage::from(img.into_rgb16()),
        ColorType::L8 => DynamicImage::from(img.into_luma8()),
        ColorType::L16 => DynamicImage::from(img.into_luma16()),
        ColorType::Rgba8 => DynamicImage::from(img.into_rgba8()),
        ColorType::Rgba16 => DynamicImage::from(img.into_rgba16()),
        ColorType::Rgb32F => DynamicImage::from(img.into_rgb32f()),
        ColorType::Rgba32F => DynamicImage::from(img.into_rgba32f()),
        _ => DynamicImage::from(img.into_rgb8()),
    };

    return auto_resize(map, res, res);
}

/// Returns the color format for the given map name.
pub(crate) fn map_to_color(map_name: &str) -> ColorType {
    return match map_name {
        "basecolor" => ColorType::Rgb8,
        "diffuse" => ColorType::Rgb8,

        "basecoloralpha" => ColorType::Rgba8, // If we're including alpha in our basecolor, forcibly include it

        "normal" => ColorType::Rgb16,

        "ao" => ColorType::L8,
        "occlusion" => ColorType::L8,
        "roughness" => ColorType::L8,
        "metallic" => ColorType::L8,
        "metalness" => ColorType::L8,

        "mask" => ColorType::L8,
        "opacity" => ColorType::L8,
        "alpha" => ColorType::L8,

        _ => ColorType::Rgb8,
    };
}

pub(crate) fn int16_to_float64(a: u16) -> f64 {
    return (a as f64) / (u16::MAX as f64);
}

/// Automatically resizes an image, preserving aspect ratio, so the maximum dimension of the image matches the max specified dimension.
/// If the image already matches the specified dimension, then no operation is performed.
pub(crate) fn auto_resize(
    img: DynamicImage,
    mut width: u32,
    mut height: u32,
) -> (DynamicImage, u32, u32) {
    let (dim_x, dim_y) = img.dimensions();

    // Pick resizing filter based off of what we're doing, up-scaling versus downscaling
    // https://stackoverflow.com/questions/384991/what-is-the-best-image-downscaling-algorithm-quality-wise

    // If our image dimensions are not equal, find the ratio between the max values, and scale accordingly
    if dim_x != dim_y {
        let scale_factor = max(width, height) as f64 / max(dim_x, dim_y) as f64;
        width = (dim_x as f64 * scale_factor) as u32;
        height = (dim_y as f64 * scale_factor) as u32;
    }

    if dim_x > width || dim_y > height {
        // If we're up-scaling an image, use Catmull Rom
        return (
            img.resize(width, height, imageops::FilterType::CatmullRom),
            width,
            height,
        );
    } else if dim_x != width || dim_y != height {
        // If we're downscaling an image, use Lanczos
        return (
            img.resize(width, height, imageops::FilterType::Lanczos3),
            width,
            height,
        );
    }

    return (img, width, height);
}

/// Saves an image buffer to the given path, using a specific color format, at the best compression
pub(crate) fn compressed_save(
    path: &Path,
    buffer: &[u8],
    width: u32,
    height: u32,
    format: ColorType,
) {
    let f = File::create(path).unwrap(); // Create a file at the given path
    let writer = BufWriter::new(f); // Create a writer buffer to it

    // Set up a PNG encoder on top of the buffer, and attempt to maximize compression (for space efficiency)
    let encoder = png::PngEncoder::new_with_quality(
        writer,
        png::CompressionType::Best,
        png::FilterType::Adaptive,
    );

    encoder.write_image(buffer, width, height, format).unwrap(); // Finally, write out the image
}

/// Creates a texture filepath for the given parameters.
/// Can be absolute or relative, depending on `directory` input.
pub(crate) fn path_material_map(
    directory: &Path,
    material_name: &str,
    data: &str,
    format: &str,
) -> PathBuf {
    // Concatenate strings together
    let mut owned_str: String = material_name.to_string();
    owned_str.push_str("_");
    owned_str.push_str(data);
    owned_str.push_str(".");
    owned_str.push_str(format);

    return directory.join(Path::new(owned_str.as_str()));
}
