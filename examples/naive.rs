extern crate image;
extern crate rrt;

use image::{GenericImage, ImageBuffer};
use rrt::*;
use std::fs::File;

fn main() {
    let mut shapes: Vec<TexedShape> = Vec::new();
    shapes.push(pure_color_shape(
        Rgb::new(0.2, 0.2, 0.8),
        Sphere::new(vec3(250.0, 250.0, -1000.0), 150.0),
    ));

    shapes.push(pure_color_shape(
        Rgb::new(0.8, 0.2, 0.2),
        Triangle::new(
            vec3(300.0, 600.0, -800.0),
            vec3(0.0, 100.0, -1000.0),
            vec3(450.0, 20.0, -1000.0),
        ),
    ));

    let img = ImageBuffer::from_fn(500, 500, |x, y| {
        let ray = RayBuilder {
            origin: vec3(x as f32, y as f32, 0.0),
            direction: vec3(0.0, 0.0, -1.0),
        }.build();
        for shape in &shapes {
            let shape = &*shape;
            if let Some(hit) = shape.hit(&ray, 0.00001, 1000.0) {
                return image::Rgb::from(Rgb::black()); //image::Rgb::from(hit.color);
            } else {
                continue;
            }
        }
        image::Rgb::from(Rgb::black())
    });
    let mut out = File::create("test.png").unwrap();
    image::ImageRgb8(img).save(&mut out, image::PNG).unwrap();
}
