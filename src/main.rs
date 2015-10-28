extern crate image;

mod simplex_noise;

use std::fs::File;
use std::path::Path;
use simplex_noise::noise::noise2d;

fn main() {
    let height = 500;
    let width  = 500;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let scale = 0.009;
        let luma = (noise2d(x as f32 * scale, y as f32 * scale) + 1.0) / 2.0 * 255.0;
        *pixel = image::Luma([luma as u8]);
    }

    let ref mut output_image = File::create(&Path::new("simplex_noise.png")).unwrap();
    let _ = image::ImageLuma8(imgbuf).save(output_image, image::PNG);
}
