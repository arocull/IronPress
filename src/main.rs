#![doc(html_favicon_url = "https://alanocull.com/favicon.ico")]
//! Texture optimization tool for games and animation.

/// Command line interface helpers.
mod cli {
    /// Launch argument helpers for IronPress.
    pub mod args;
    /// Configuration file formatting.
    pub mod config;
}
/// Common utilities.
mod util;
/// Operations for manipulating images.
mod op {
    /// Methods for flipping channels.
    pub mod flip;
    /// Methods for packing channels.
    pub mod pack;
}
/// Texture pipeline command.
mod pipeline;

use std::{path::Path, process::exit};
use clap::Parser;

fn main() {
    // Parse command-line arguments via clap.
    let args = cli::args::CLIArguments::parse();

    if args.default {
        let success = cli::config::write_default(Path::new(&args.file));
        exit(!success as i32);
    }

    // Perform pipeline.
    pipeline::from_file(Path::new(&args.file), args.dryrun);
}
