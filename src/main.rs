extern crate image;

use std::env;
use std::env::join_paths;
use std::fs;
use std::fs::read_to_string;
use std::path::{Path};
use image::{DynamicImage, GenericImageView, imageops};

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

fn pipeline_img_path(directory: &Path, material_name: string, data: string, format: string) -> &Path {
    let img_name = concat!(material_name, "_", data, ".", format);
    let path_buf = directory.join(Path::new(&img_name));
    return path_buf.as_path();
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

fn pipeline(config_file: &Path, args: Vec<String>) {
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

    println!("Hello world! {0}", outdir.to_str().unwrap());

    let mats = config["materials"].entries();

    // Check if we're using DirectX normals--if true, flip green channels
    let flip_normals = config.has_key("flip_normals") && config["flip_normals"].as_bool().unwrap();

    // Iterate through all materials
    for (matname, mat) in mats {
        let channels = mat["channels"].clone();
        let res = mat["res_out"].as_u32().unwrap();

        if channels.contains("basecolor") {
            let basecolor_path = pipeline_img_path(indir, matname, "basecolor", "png");
            let basecolor = pipeline_resize_if_necessary(util::load_image(basecolor_path), res, res);
            
        }

        // Load textures for given material
        // scale textures according to configuration
        // then combine as needed (ORM pass)
        // Save textures in corresponding formats in output directory
    }
}
