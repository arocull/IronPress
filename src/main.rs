extern crate image;

mod channel_flip;
mod channel_pack;
mod mask_sum;
mod pipeline;
/// Common utilities.
mod util;
/// Command line interface helpers.
mod cli {
    /// Launch argument helpers for IronPress.
    pub mod args;
}

use std::path::Path;
use clap::Parser;

fn main() {
    let args = cli::args::CLIArguments::parse();

    // Perform pipeline
    pipeline::from_file(Path::new(&args.file));
}
