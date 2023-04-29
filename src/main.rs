extern crate image;

use std::env;
use std::fs;
use std::fs::read_to_string;
use std::path::{Path};

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

    println!("Hello world! {0}", outdir.to_str().unwrap());

    // Iterate through all materials
        // Load textures for given material
        // scale textures according to configuration
        // then combine as needed (ORM pass)
    // Save textures in corresponding formats in output directory
}
