extern crate rrt;
extern crate image;

use image::{GenericImage, ImageBuffer, Rgb};

use rrt::*;

fn main() {
    let mut shapes: Vec<Box<Shape>> = Vec::new();
    shapes.push(Box::new(Sphere::new(Vector3::from_xyz(250.0, 250.0, -1000.0), 150.0, Rgb::new(0.2, 0.2, 0.8))));
    shapes.push(Box::new(Triangle::new(
        Vector3::from_xyz(300.0, 600.0, -800.0),
        Vector3::from_xyz(0.0, 100.0, -1000.0),
        Vector3::from_xyz(450.0, 20.0, -1000.0),
        Rgb::new(0.8, 0.2, 0.2))
    ));

    let img = ImageBuffer::from_fn(500, 500, |x, y| {
        let ray = Ray {
            origin: Vector3::from_xyz(x as f32, y as f32, 0.0),
            direction: Vector3::from_xyz(0.0, 0.0, -1.0)
        };
        for shape in &shapes {
            let shape = &*shape;
            if let Some(hit) = shape.hit(&ray, 0.00001, 1000.0) {
                return image::RGB(hit.color.r, hit.color.g, hit.color.b)
            } else {
                return image::RGB(0)
            }
        }
    });

}