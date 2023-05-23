use std::fs;
use std::fs::{read_to_string};
use std::path::{Path};
use image::{ColorType, EncodableLayout, ImageBuffer, ImageFormat, Rgb, Rgba32FImage};
use image::buffer::ConvertBuffer;
use crate::channel_flip::flip_green;
use crate::channel_pack::channel_pack_images;
use crate::util;

pub(crate) fn from_file(config_file: &Path, _args: Vec<String>) {
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

    // Iterate through all materials
    for (matname, mat) in mats {
        let channels = mat["channels"].clone();
        let res = mat["res_out"].as_u32().unwrap();

        // Basecolor map
        if channels.contains("basecolor") {
            let img_path = util::path_material_map(indir, matname, "basecolor", "png");
            println!("Loading basecolor from {0}", img_path.to_str().unwrap());
            let basecolor = util::auto_resize(util::load_image(img_path.as_path()), res, res).into_rgb8();
            let img_path = util::path_material_map(outdir, matname, "basecolor", "png");
            println!("\tExporting basecolor to {0}", img_path.to_str().unwrap());
            util::compressed_save(img_path.as_path(), basecolor.as_bytes(), res, res, ColorType::Rgb8);
        }

        // Normal map
        if channels.contains("normal") {
            let img_path = util::path_material_map(indir, matname, "normal", "png");
            println!("Loading normal map from {0}", img_path.to_str().unwrap());

            let mut normal = util::auto_resize(util::load_image(img_path.as_path()), res, res).into_rgb16();
            if flip_normals {
                println!("\tFlipping normal map");
                normal = flip_green(normal);
            }
            let normal = normal; // Lock normal so it can no longer be edited

            let img_path = util::path_material_map(outdir, matname, "normal", "png");
            println!("\tExporting normal map to {0}", img_path.to_str().unwrap());
            util::compressed_save(img_path.as_path(), normal.as_bytes(), res, res, ColorType::Rgb16);
        }

        // ORM (Occlusion, Roughness, Metallic)
        if channels.contains("ao") || channels.contains("roughness") || channels.contains("metallic") {
            println!("Preparing ORM map...");
            let mut images: Vec<Rgba32FImage> = Vec::new();
            let out_path = util::path_material_map(outdir, matname, "orm", "png");

            // Find all corresponding images, or default to a blank one
            // TODO: Remove duplicate code by packing into functions

            // Ambient Occlusion
            let ao_path = util::path_material_map(indir, matname, "ao", "png");
            if channels.contains("ao") && ao_path.exists() {
                println!("\tLoading Ambient Occlusion");
                let map = util::auto_resize(util::load_image(ao_path.as_path()), res, res).into_rgba32f();
                images.push(map);
            } else {
                println!("\tUsing default Ambient Occlusion map");
                // TODO: Fill this image with white instead of black
                images.push(image::DynamicImage::new_rgba32f(res, res).into_rgba32f());
            }

            // Roughness
            let roughness_path = util::path_material_map(indir, matname, "roughness", "png");
            if channels.contains("roughness") && roughness_path.exists() {
                println!("\tLoading Roughness");
                let map = util::auto_resize(util::load_image(roughness_path.as_path()), res, res).into_rgba32f();
                images.push(map);
            } else {
                println!("\tUsing default Roughness map");
                // TODO: Fill this image with grey instead of black
                images.push(image::DynamicImage::new_rgba32f(res, res).into_rgba32f());
            }

            // Metallic
            let metallic_path = util::path_material_map(indir, matname, "metallic", "png");
            if channels.contains("metallic") && metallic_path.exists() {
                println!("\tLoading Metallic");
                let map = util::auto_resize(util::load_image(metallic_path.as_path()), res, res).into_rgba32f();
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
            
            util::compressed_save(out_path.as_path(), orm.as_bytes(), res, res, ColorType::Rgb8);
        }

        for member in channels.members() {
            let mem = member.as_str().unwrap();
            if mem.starts_with("mask") || mem.starts_with("opacity") { // Export masks
                println!("Found mask {0}, loading...", mem);
                let img_path = util::path_material_map(indir, matname, mem, "png");
                let map = util::auto_resize(util::load_image(img_path.as_path()), res, res).into_luma8();
                let img_path = util::path_material_map(outdir, matname, mem, "png");
                println!("\tExporting mask {0}...", mem);
                util::compressed_save(img_path.as_path(), map.as_bytes(), res, res, ColorType::L8);
            }
        }
    }
}
