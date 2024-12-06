use image::{ImageBuffer, Rgba, Rgba32FImage};

/// Provided a set of images, packs the Red channel of each image into the corresponding RGBA channels of a new one.
pub fn channel_pack(
    channel_data: Vec<Rgba32FImage>,
    use_alpha: bool,
    width: u32,
    height: u32,
) -> ImageBuffer<Rgba<f32>, Vec<f32>> {
    // Prep new image
    let mut imgbuf = image::ImageBuffer::new(width, height);
    // Fill out pixels of new image with data from inputs
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = channel_data[0].get_pixel(x, y).0[0] as f32;
        let g = channel_data[1].get_pixel(x, y).0[1] as f32;
        let b = channel_data[2].get_pixel(x, y).0[2] as f32;
        let mut a = channel_data[3].get_pixel(x, y).0[3] as u8;
        if !use_alpha {
            // If we're not using alpha, maximize the alpha channel
            a = 255 as u8;
        }

        *pixel = Rgba([r, g, b, a.into()]);
    }

    return imgbuf;
}
