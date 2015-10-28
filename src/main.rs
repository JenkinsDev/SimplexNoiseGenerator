extern crate image;

mod simplex_noise;

use std::fs::File;
use std::path::Path;
use simplex_noise::noise::noise2d;

fn sum_ocatave(num_iterations: i32, x: f32, y: f32, persistence: f32, scale: f32, low: f32, high: f32) -> f32 {
    let mut max_amp = 0.0;
    let mut freq = scale;
    let mut amp = 1.0;
    let mut noise = 0.0;
    let mut i = 0;

    while i < num_iterations {
        noise += noise2d(x * freq, y * freq) * amp;
        max_amp += amp;
        amp *= persistence;
        freq *= 2.0;

        i += 1;
    }

    noise /= max_amp;

    noise * (high - low) / 2.0 + (high + low) / 2.0
}

fn main() {
    let height = 1080;
    let width  = 1920;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let scale = 0.009;
        let luma = sum_ocatave(16, x as f32, y as f32, 0.5, scale, 0.0, 255.0);
        *pixel = image::Luma([luma as u8]);
    }

    let ref mut output_image = File::create(&Path::new("simplex_noise.png")).unwrap();
    let _ = image::ImageLuma8(imgbuf).save(output_image, image::PNG);
}
