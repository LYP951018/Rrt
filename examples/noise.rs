extern crate image;
extern crate rand;
extern crate rrt;

use image::{GenericImage, ImageBuffer};

use rrt::*;
use rrt::noise;
use std::fs::File;

fn main() {
    let img = ImageBuffer::from_fn(500, 500, |x, y| {
        let n = noise::perlin_noise(&vec3(x as f32 / 32.0, y as f32 / 32.0, 0.0));
        image::Rgb::from(Rgb::white() * ((n + 1.0) / 2.0))
    });

    let mut out = File::create("noise.png").unwrap();
    image::ImageRgb8(img).save(&mut out, image::PNG).unwrap();
}
