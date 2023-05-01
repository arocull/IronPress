extern crate image;

use std::env;
use std::path::{Path};

mod channel_pack;
mod channel_flip;
mod mask_sum;
mod util;
mod pipeline;

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
        pipeline::from_file(Path::new(&command), args_packed);
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
