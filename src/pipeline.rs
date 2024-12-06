use crate::{op, util};
use image::{ColorType, DynamicImage, Rgba32FImage};
use std::cmp::min;
use std::fs::read_to_string;
use std::path::Path;
use std::thread;
use std::thread::JoinHandle;
use std::time;
use std::{fs, path};

/// Loads, (optionally) packs, converts, and compresses a single image from the given parameters.
fn threaded_convert(
    input_dir: String,
    outdir: String,
    material: String,
    channel: String,
    resolution: u32,
    flip_green: bool,
    has_alpha: bool,
) -> JoinHandle<()> {
    return thread::spawn(move || {
        // Generate file paths from strings (since we can't copy them from threads)
        let input_dir = path::PathBuf::from(input_dir);
        let output_dir = path::PathBuf::from(outdir);

        // Load defaults
        let mut ct = util::map_to_color(channel.as_str());
        let mut out_img: DynamicImage; //= image::DynamicImage::new_luma8(resolution, resolution); // Create blank template image
        let out_path = util::path_material_map(
            output_dir.as_path(),
            material.as_str(),
            channel.as_str(),
            "png",
        );
        let mut width: u32 = resolution;
        let mut height: u32 = resolution;

        // Forcibly include alpha in basecolor pass if we were told to
        if has_alpha && channel.eq("basecolor") {
            ct = ColorType::Rgba8;
        }

        let base_path = util::path_material_map(
            input_dir.as_path(),
            material.as_str(),
            channel.as_str(),
            "png",
        );
        if channel.eq("arm") && !base_path.exists() {
            // Only load basemaps for ARM if there isn't an existing ARM texture
            let base_path_ao =
                util::path_material_map(input_dir.as_path(), material.as_str(), "ao", "png");
            let base_path_rough =
                util::path_material_map(input_dir.as_path(), material.as_str(), "roughness", "png");
            let base_path_metal =
                util::path_material_map(input_dir.as_path(), material.as_str(), "metallic", "png");

            // TODO: I don't like storing all these as 32F images. Large and hard to work with.
            let map_ao: Rgba32FImage;
            let map_rough: Rgba32FImage;
            let map_metal: Rgba32FImage;

            // TODO: remove duplication here if possible?
            if base_path_ao.exists() {
                let (m, w, h) =
                    util::load_image_adv(base_path_ao.as_path(), resolution, ColorType::Rgba32F);
                width = min(width, w);
                height = min(height, h);
                map_ao = m.into_rgba32f();
            } else {
                map_ao = DynamicImage::new_rgba32f(width, height).into_rgba32f();
            }
            if base_path_rough.exists() {
                let (m, w, h) =
                    util::load_image_adv(base_path_rough.as_path(), resolution, ColorType::Rgba32F);
                width = min(width, w);
                height = min(height, h);
                map_rough = m.into_rgba32f();
            } else {
                map_rough = DynamicImage::new_rgba32f(width, height).into_rgba32f();
            }
            if base_path_metal.exists() {
                let (m, w, h) =
                    util::load_image_adv(base_path_metal.as_path(), resolution, ColorType::Rgba32F);
                width = min(width, w);
                height = min(height, h);
                map_metal = m.into_rgba32f();
            } else {
                map_metal = DynamicImage::new_rgba32f(width, height).into_rgba32f();
            }

            // TODO: simplify conversion process if possible
            let maps = vec![
                map_ao,
                map_rough,
                map_metal,
                image::DynamicImage::new_rgba32f(width, height).into_rgba32f(),
            ];
            let arm = op::pack::channel_pack(maps, false, width, height);
            out_img = DynamicImage::from(DynamicImage::from(arm).into_rgb8());
            ct = ColorType::Rgb8; // Override color space
        } else {
            // Otherwise, use default process
            if !base_path.exists() {
                // Exit thread if path isn't found
                eprintln!("\tFILE NOT FOUND at {0}", base_path.to_str().unwrap());
                return;
            }
            let (m, w, h) = util::load_image_adv(base_path.as_path(), resolution, ct);
            out_img = m;
            width = w;
            height = h;
        }

        // If requested and this is a normal map, invert green channel
        if flip_green && channel.eq("normal") {
            out_img = DynamicImage::from(op::flip::flip_green(out_img.into_rgb16()));
        }

        // Save out image
        util::compressed_save(out_path.as_path(), out_img.as_bytes(), width, height, ct.into());
        println!("\tExported {0}", out_path.to_str().unwrap());
    });
}

/// Loads an IronPress Pipeline JSON file and converts spawns a thread for converting each material, awaiting until all threads are completed.  
pub fn from_file(config_file: &Path) {
    let dir = config_file.parent().unwrap(); // Get working directory

    // Load and parse configuration
    let config_contents = read_to_string(config_file).unwrap();
    let config_result = json::parse(config_contents.as_str());
    if config_result.is_err() {
        // Pull error from unwrap and safely exit
        println!(
            "While parsing JSON config, got error:\t{0}",
            config_result.unwrap_err()
        );
        return;
    }
    let config = config_result.unwrap();

    // Get output directory, relative to parent (or replacing it, if path is absolute)
    let outdir_buf = dir.join(Path::new(&(config["output"].as_str().unwrap())));
    let outdir = outdir_buf.as_path();
    if !outdir.exists() {
        // If path does not exist, create all folders so it does
        fs::create_dir_all(outdir).unwrap();
    }

    // Get input directory, relative to parent (or replacing it, if path is absolute)
    let indir_buf = dir.join(Path::new(&(config["input"].as_str().unwrap())));
    let indir = indir_buf.as_path();
    if !indir.exists() {
        // If path does not exist, create all folders so it does
        fs::create_dir_all(indir).unwrap();
    }

    println!(
        "Successfully loaded config at {0}",
        config_file.to_str().unwrap()
    );

    let mats = config["materials"].entries();

    // Check if we're using DirectX normals--if true, flip green channels
    let flip_normals = config.has_key("flip_normals") && config["flip_normals"].as_bool().unwrap();

    let time_start = time::Instant::now();
    let mut num_materials: u32 = 0;
    let mut num_maps: u32 = 0;

    let mut threads: Vec<JoinHandle<()>> = Vec::new();

    // Iterate through all materials
    for (material_name, mat) in mats {
        let channels = mat["channels"].clone();
        let res = mat["max_dimension"].as_u32().unwrap();
        let has_alpha = mat.has_key("alpha");
        num_materials += 1;

        for member in channels.members() {
            let mem = member.as_str().unwrap();
            num_maps += 1;

            // Spawn thread with basic map conversion information
            threads.push(threaded_convert(
                indir_buf.to_str().unwrap().to_string(),
                outdir_buf.to_str().unwrap().to_string(),
                material_name.to_string(),
                mem.to_string(),
                res,
                flip_normals,
                has_alpha,
            ));
        }
    }

    // Wait on all threads
    for t in threads {
        t.join().expect("Await thread to rejoin main thread");
    }

    let time_end = time::Instant::now();
    println!(
        "Completed {0} materials with {1} exported maps, in {2} ms",
        num_materials,
        num_maps,
        (time_end - time_start).as_millis()
    );
}

#[cfg(test)]
mod tests {
    use std::{env, fs, path::PathBuf};

    use super::from_file;

    #[test]
    fn validate_pipeline() {
        // First, find our test pipeline
        let path_directory = env::current_dir().expect("Get current working directory");
        let path_pipeline = path_directory.join(PathBuf::from("test/clover/texture_pipeline.json"));

        // Find output directory
        let path_output = path_directory.join(PathBuf::from("test/clover/out"));
        // If we had an existing output directory, destroy it to ensure we can create them on the fly
        if path_output.exists() {
            fs::remove_dir_all(path_output.clone())
                .expect("Failed to delete output directory for test");
            assert!(
                !path_output.exists(),
                "Output directory still exists, cannot perform valid test"
            );
        }

        assert!(path_pipeline.exists(), "Test pipeline file didn't exist"); // Ensure that the filepath exists

        // Perform our pipeline output
        from_file(&path_pipeline);

        // Ensure that our output directory exists
        assert!(path_output.exists(), "Output directory didn't exist");

        // For each output file...
        let textures = vec![
            path_directory.join(PathBuf::from("test/clover/out/mat_fur_mask.png")),
            path_directory.join(PathBuf::from("test/clover/out/mat_daisy_basecolor.png")),
            path_directory.join(PathBuf::from("test/clover/out/mat_daisy_arm.png")),
            path_directory.join(PathBuf::from("test/clover/out/mat_daisy_normal.png")),
            path_directory.join(PathBuf::from("test/clover/out/mat_body_basecolor.png")),
            path_directory.join(PathBuf::from("test/clover/out/mat_body_arm.png")),
            path_directory.join(PathBuf::from("test/clover/out/mat_body_normal.png")),
            path_directory.join(PathBuf::from("test/clover/out/mat_sunhat_basecolor.png")),
            path_directory.join(PathBuf::from("test/clover/out/mat_sunhat_arm.png")),
            path_directory.join(PathBuf::from("test/clover/out/mat_sunhat_normal.png")),
            path_directory.join(PathBuf::from("test/clover/out/mat_sunhat_mask.png")),
            path_directory.join(PathBuf::from("test/clover/out/mat_shears_basecolor.png")),
            path_directory.join(PathBuf::from("test/clover/out/mat_shears_arm.png")),
            path_directory.join(PathBuf::from("test/clover/out/mat_shears_normal.png")),
            path_directory.join(PathBuf::from("test/clover/out/mat_eye_basecolor.png")),
            path_directory.join(PathBuf::from("test/clover/out/mat_eye_normal.png")),
        ];

        // ...verify that it outputted...
        for path_texture in textures.iter() {
            assert!(
                path_texture.exists(),
                "Failed to find output texture {0}",
                path_texture
                    .to_str()
                    .expect("Failed to cast output path to string")
            );
        }

        // TODO: verify bit depth, resolution, differing channel values for ARM textures
    }
}
