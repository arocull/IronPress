use std::{fs, path::Path};
use json;

/// Returns a default IronPress configuration.
pub fn default() -> json::JsonValue {
    json::object! {
        input: "./input/",
        output: "./output/",
        flip_normals: false,
        materials: {
            mat_example: {
                max_dimension: 512, // Max dimension resolution of the file output.
                alpha: false,
                channels: [
                    "basecolor",
                    "arm",
                    "normal",
                    "mask",
                ]
            }
        }
    }
}

/// Writes the default IronPress configuration to a file.
/// Returns TRUE on successful write, FALSE otherwise.
pub fn write_default(output: &Path) -> bool {
    let cfg = default();

    let res = fs::write(output, json::stringify_pretty(cfg, 4));
    match res.err() {
        Some(err) => {
            println!("failed to output default config, {0}", err);
            return false;
        },
        None => {
            match output.to_str() {
                Some(str) => println!("output default config to {0}", str),
                None => println!("output default config")
            }
        }
    }
    return true;
}
