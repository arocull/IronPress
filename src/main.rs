extern crate image;

use std::env;
use std::fs;
use std::fs::read_to_string;
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use image::{DynamicImage, GenericImageView, ImageBuffer, ImageFormat, imageops, Rgb, Rgba32FImage};
use image::buffer::ConvertBuffer;
use crate::channel_flip::flip_green;
use crate::channel_pack::channel_pack_images;

mod channel_pack;
mod channel_flip;
mod mask_sum;
mod util;

fn main() {
    let args: Vec<_> = env::args().collect();

    // First argument is where executable is
    // Second argument is mode to use
    let command = args[1].clone();

    // Pack remaining arguments into a list for future processing
    let mut args_packed: Vec<String> = Vec::new();
    for i in 2..args.len() {
        args_packed.push(args[i].clone());
    }

    if command.ends_with(".json") {
        // Perform pipeline
        pipeline(Path::new(&command), args_packed);
        return;
    }

    // Perform command based off argument
    if command.eq("mask") {
        mask_sum::execute(args_packed, 2048, 2048);
    } else if command.eq("pack") {
        channel_pack::execute(args_packed);
    } else if command.eq("flipnorm") {
        channel_flip::execute(args_packed);
    }
}

fn pipeline_img_path(directory: &Path, material_name: &str, data: &str, format: &str) -> PathBuf {
    // Concatenate strings together
    let mut owned_str: String = material_name.clone().to_string();
    owned_str.push_str("_");
    owned_str.push_str(data);
    owned_str.push_str(".");
    owned_str.push_str(format);

    return directory.join(Path::new(owned_str.as_str()));
}

fn pipeline_resize_if_necessary(img: DynamicImage, width: u32, height: u32) -> DynamicImage {
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

fn pipeline(config_file: &Path, _args: Vec<String>) {
    let dir = config_file.parent().unwrap(); // Get working directory

    // Load and parse configuration
    let config_contents = read_to_string(config_file).unwrap();
    let config = json::parse(config_contents.as_str()).unwrap();

    // Get output directory, relative to parent (or replacing it, if path is absolute)
    let outdir_buf = dir.join(Path::new(&(config["out"].as_str().unwrap())));
    let outdir = outdir_buf.as_path();
    if !outdir.exists() { // If path does not exist, create all folders so it does
        fs::create_dir_all(outdir).unwrap();
    }

    // Get input directory, relative to parent (or replacing it, if path is absolute)
    let indir_buf = dir.join(Path::new(&(config["in"].as_str().unwrap())));
    let indir = indir_buf.as_path();
    if !indir.exists() { // If path does not exist, create all folders so it does
        fs::create_dir_all(indir).unwrap();
    }

    println!("Successfully loaded config at {0}", config_file.to_str().unwrap());

    let mats = config["materials"].entries();

    // Check if we're using DirectX normals--if true, flip green channels
    let flip_normals = config.has_key("flip_normals") && config["flip_normals"].as_bool().unwrap();

    // let codec = image::codecs::png::PngEncoder::new_with_quality(, image::codecs::png::CompressionType::Best, image::codecs::png::FilterType::Adaptive);

    // Iterate through all materials
    for (matname, mat) in mats {
        let channels = mat["channels"].clone();
        let res = mat["res_out"].as_u32().unwrap();

        // Basecolor map
        if channels.contains("basecolor") {
            let img_path = pipeline_img_path(indir, matname, "basecolor", "png");
            println!("Loading basecolor from {0}", img_path.to_str().unwrap());
            let basecolor = pipeline_resize_if_necessary(util::load_image(img_path.as_path()), res, res).into_rgb8();
            let img_path = pipeline_img_path(outdir, matname, "basecolor", "png");
            println!("\tExporting basecolor to {0}", img_path.to_str().unwrap());
            basecolor.save_with_format(img_path, ImageFormat::Png).unwrap();

        }

        // Normal map
        if channels.contains("normal") {
            let img_path = pipeline_img_path(indir, matname, "normal", "png");
            println!("Loading normal map from {0}", img_path.to_str().unwrap());

            let mut normal = pipeline_resize_if_necessary(util::load_image(img_path.as_path()), res, res).into_rgb16();
            if flip_normals {
                println!("\tFlipping normal map");
                normal = flip_green(normal);
            }
            let normal = normal; // Lock normal so it can no longer be edited

            let img_path = pipeline_img_path(outdir, matname, "normal", "png");
            println!("\tExporting normal map to {0}", img_path.to_str().unwrap());
            normal.save_with_format(img_path, ImageFormat::Png).unwrap();
        }

        // ORM (Occlusion, Roughness, Metallic)
        if channels.contains("ao") || channels.contains("roughness") || channels.contains("metallic") {
            println!("Preparing ORM map...");
            let mut images: Vec<Rgba32FImage> = Vec::new();
            let out_path = pipeline_img_path(outdir, matname, "orm", "png");

            // Find all corresponding images, or default to a blank one
            // TODO: Remove duplicate code by packing into functions

            // Ambient Occlusion
            let ao_path = pipeline_img_path(indir, matname, "ao", "png");
            if channels.contains("ao") && ao_path.exists() {
                println!("\tLoading Ambient Occlusion");
                let map = pipeline_resize_if_necessary(util::load_image(ao_path.as_path()), res, res).into_rgba32f();
                images.push(map);
            } else {
                println!("\tUsing default Ambient Occlusion map");
                // TODO: Fill this image with white instead of black
                images.push(image::DynamicImage::new_rgba32f(res, res).into_rgba32f());
            }

            // Roughness
            let roughness_path = pipeline_img_path(indir, matname, "roughness", "png");
            if channels.contains("roughness") && roughness_path.exists() {
                println!("\tLoading Roughness");
                let map = pipeline_resize_if_necessary(util::load_image(roughness_path.as_path()), res, res).into_rgba32f();
                images.push(map);
            } else {
                println!("\tUsing default Roughness map");
                // TODO: Fill this image with grey instead of black
                images.push(image::DynamicImage::new_rgba32f(res, res).into_rgba32f());
            }

            // Metallic
            let metallic_path = pipeline_img_path(indir, matname, "metallic", "png");
            if channels.contains("metallic") && metallic_path.exists() {
                println!("\tLoading Metallic");
                let map = pipeline_resize_if_necessary(util::load_image(metallic_path.as_path()), res, res).into_rgba32f();
                images.push(map);
            } else {
                println!("\tUsing default Metallic map");
                images.push(image::DynamicImage::new_rgba32f(res, res).into_rgba32f());
            }

            // Push blank image for alpha
            images.push(image::DynamicImage::new_rgba32f(res, res).into_rgba32f());

            // Pack channels
            println!("Processing ORM map...");
            let orm_raw = channel_pack_images(images, false, res, res);
            let orm: ImageBuffer<Rgb<u8>, Vec<u8>> = orm_raw.convert(); // Convert to RGB8
            println!("\tExporting ORM map to {0}", out_path.to_str().unwrap());
            orm.save_with_format(out_path.as_path(), ImageFormat::Png).unwrap(); // Save out file
        }
    }
}
