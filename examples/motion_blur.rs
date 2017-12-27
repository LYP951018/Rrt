extern crate rrt;
extern crate image;
extern crate rand;

use image::{GenericImage, ImageBuffer};
use rand::Rng;
use std::f32;

use rrt::*;
use std::fs::File;

fn main() {
    let camera = Camera::new(ThinLens::new(1.0, Vector3::zero(), 1.0), 
        Vector3::zero(), 
        Vector3::back(), Vector3::up(), 1.0, f32::consts::PI / 4.0, 2.0);
    let mut shapes: Vec<Box<Shape>> = Vec::new();
    shapes.push(Box::new(Sphere::new(Vector3::new(0.0, 0.0, -20.0), 100.0, Rgb::new(0.2, 0.2, 0.8))));
    let mut rng = rand::thread_rng();

    let img = ImageBuffer::from_fn(500, 500, |x, y| {
        let transform = |x: f32| x * 2.0 - 1.0;        
        let len_pos_x = transform(rng.gen_range(0.0, 1.0));
        let len_pos_y = transform(rng.gen_range(0.0, 1.0));

        for shape in &shapes {
            let shape = &*shape;
            if let Some(hit) = shape.hit(&camera.gen_ray(x as f32 / 500.0, y as f32 / 500.0, len_pos_x, len_pos_y, 2.0), 0.00001, 1000.0, 0.0) {
                return image::Rgb::from(hit.color);
            } else {
                continue;
            }
        }
        image::Rgb::from(Rgb::black())
    });
    let mut out = File::create("motion_blur.png").unwrap();
    image::ImageRgb8(img).save(&mut out, image::PNG);
}