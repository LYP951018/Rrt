extern crate image;
extern crate rand;
extern crate rrt;

use image::ImageBuffer;
use rand::Rng;
use rrt::*;
use std::f32;
use std::fs::File;
use std::iter::*;

const MOVE_TIMES: u32 = 50;

fn color(camera: &Camera, shapes: &[Box<Shape>], pixel: Vector2, lens: Vector2) -> Rgb {
    let dir = Vector3::new(0.005, 0.0, 0.0);
    let ray = camera.gen_ray(&pixel, &lens);
    for shape in shapes {
        let mut total = Rgb::black();
        let mut hitted = false;
        for t in 0..MOVE_TIMES {
            if let Some(hit) = shape.hit(&ray, 0.00001, 1000.0, &(dir * (t as f32))) {
                // println!("{:?}", hit);
                total = &total + hit.color;
                hitted = true;
            }
        }
        if hitted {
            return total / (MOVE_TIMES as f32);
        }
    }
    Rgb::black()
}

const SAMPLE_COUNT: u32 = 64;

fn sampling<T: Rng>(
    rng: &mut T,
    camera: &Camera,
    shapes: &[Box<Shape>],
    x: u32,
    y: u32,
) -> image::Rgb<u8> {
    let mut pixels = Vec::new();
    let mut lens = Vec::new();

    sample::jitter(rng, &mut pixels, SAMPLE_COUNT);
    sample::jitter(rng, &mut lens, SAMPLE_COUNT);

    rng.shuffle(&mut pixels);
    rng.shuffle(&mut lens);

    let pixel_trans = Vector2::new(x as f32, y as f32);
    image::Rgb::from(
        pixels
            .into_iter()
            .map(|x| (x + pixel_trans) / 250.0)
            .zip(lens.into_iter().map(|y| y.to_center()))
            .map(|(p, l)| color(camera, shapes, p, l))
            .fold(Rgb::black(), |l, r| l + r) / (SAMPLE_COUNT as f32),
    )
}

fn main() {
    let camera = CameraBuilder {
        lens: ThinLens {
            radius: 1.0,
            center: Vector3::zero(),
            focal_length: 1.0,
        },
        at: Vector3::zero(),
        target: Vector3::back(),
        up: Vector3::up(),
        aspect_ratio: 1.0,
        fov: f32::consts::PI / 4.0,
    }.build();

    let mut shapes: Vec<Box<Shape>> = Vec::new();
    shapes.push(Box::new(Sphere::new(
        Vector3::new(0.0, 0.0, -1.01),
        0.2,
        Rgb::new(0.2, 0.2, 0.8),
    )));

    let mut rng = rand::thread_rng();

    let img = ImageBuffer::from_fn(500, 500, |x, y| sampling(&mut rng, &camera, &shapes, x, y));
    let mut out = File::create("motion_blur.png").unwrap();
    image::ImageRgb8(img).save(&mut out, image::PNG).unwrap();
}
