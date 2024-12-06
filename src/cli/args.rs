use clap::Parser;

// https://docs.rs/clap/latest/clap/
// https://docs.rs/clap/latest/clap/_derive/_tutorial/chapter_0/index.html

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CLIArguments {
    /// Path of pipeline configuration file to operate with
    // #[arg(short, long, value_name="FILE.json")]
    pub file: String,
}
