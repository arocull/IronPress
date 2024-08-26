# IronPress
Texture optimization tool for games and animation. Developed in Rust.

## Use
If a `.json` file is fed as the first argument, the program runs a pipeline based off the given configuration.
See [the Clover Pipeline test configuration](test/clover/texture_pipeline.json) for an example.

Images are taken from an input folder relative to the pipeline file.
They are then scaled accordingly, forced into proper color channel formats, and exported using maximal PNG compression
into the specified pipeline's output folder (can be relative or absolute).
The tool is multithreaded, so these will all run asynchronously.

- `$ cargo build --release` - Builds the release executable (note: Debug build is *really* slow)
- `$ cargo run --release test\clover\texture_pipeline.json` - Run an IronPress pipeline on the provided Clover textures

There is an example project inside `test/clover` with input textures and a texture pipeline file.
Run `$ cargo test --release` (`release` because it's fast) in the top-level directory to see the output.

### Notes
Certain texture maps have special features.
- `arm` - Use this to specify that you want ambient occlusion (`ao`), `roughness`, and `metallic` maps combined into RGB (since they're all single-channel).

## Other Tools
Additional features support image manipulation/analysis via commandline. These are subject to change in the future.

Current list of commands. Each [brackets] represent a filepath to an image (preferably PNG).
Use underscores (`_`) as placeholder textures. 
- `mask [mask_a] [mask_b] [output]`
  - Returns a weighted sum of two masks, as well as an image result (masks are multiplied)
  - Mask A is used for weighting, mask B is the mask to sum
  - sum = (Σ weight * overlay) / (Σ weight)
  - Stores as **Luma16**.
- `pack [channel_r] [channel_g] [channel_b] [channel_a] [output]`
  - Packs multiple texture channels into one. Useful for PBR maps.
  - Only the Red, Green, Blue, and Alpha channel are sampled from their respective texture inputs.
  - Stores as **RGBA8** with alpha, or **RGB8** if no alpha is provided.
- `flipnorm [texture] [output]`
  - Flips the green channel of a given texture.
  - Used to convert between DirectX and OpenGL normal maps.
  - Stores as **RGB16**.
