# IronPress
Texture optimization tool for games and animation.

## Usage

1. Install the tool using cargo: `$ cargo install --git https://github.com/arocull/IronPress.git` (or download the repository and run `$ cargo install --path .` inside this directory)
2. `$ ironpress --help`

## Configuration

IronPress operates on texture pipeline `.json` files. These let you define materials, where to source textures, and where to output them.

See [the Clover Pipeline test configuration](test/clover/texture_pipeline.json) for an example.

Order of operations:

1. Images are taken from an input folder relative to the pipeline file.
2. Textures are scaled (if ncessary)
3. Enforced into color channel formats
3. Exported using maximum PNG compression into the specified output folder

### Notes
Certain texture maps have special features.
- `arm` - Use this to specify that you want ambient occlusion (`ao`), `roughness`, and `metallic` maps combined into RGB (since they're all single-channel).
