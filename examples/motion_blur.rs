#![feature(nll)]

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

pub fn to_center(vec: Vector2) -> Vector2 {
    let transform = |x| (x * 2.0 - 1.0) / 2.0;
    vec2(transform(vec.x), transform(vec.y))
}

fn color(camera: &Camera, shapes: &[TexedShape], pixel: Vector2, lens: Vector2) -> Rgb {
    let ray = camera.gen_ray(&pixel, &lens);
    for shape in shapes {
        if let Some(hit) = shape.hit(&ray, 0.00001, 1000.0) {
            // println!("{:?}", hit);
            let hit_color = shape.texture.get_value(&hit.pos, &vec2(0.0, 0.0));
            return hit_color;
        }
    }
    Rgb::black()
}

const SAMPLE_COUNT: u32 = 64;

fn sampling<T: Rng>(rng: &mut T, camera: &Camera, shapes: &[TexedShape], x: u32, y: u32) -> Rgb {
    let mut pixels = Vec::new();
    let mut lens = Vec::new();

    sample::jitter(rng, &mut pixels, SAMPLE_COUNT);
    sample::jitter(rng, &mut lens, SAMPLE_COUNT);

    rng.shuffle(&mut pixels);
    rng.shuffle(&mut lens);

    let pixel_trans = vec2(x as f32, y as f32);
    pixels
        .into_iter()
        .map(|x| (x + pixel_trans) / 250.0)
        .zip(lens.into_iter().map(to_center))
        .map(|(p, l)| color(camera, shapes, p, l))
        .fold(Rgb::black(), |l, r| l + r) / (SAMPLE_COUNT as f32) / (MOVE_TIMES as f32)
}

fn main() {
    let camera = CameraBuilder {
        lens: ThinLens {
            radius: 1.0,
            center: Vector3::zero(),
            focal_length: 1.0,
        },
        at: Vector3::zero(),
        target: -Vector3::unit_z(),
        up: Vector3::unit_y(),
        aspect_ratio: 1.0,
        fov: f32::consts::PI / 4.0,
    }.build();

    let mut shapes = vec![pure_color_shape(
        Rgb::new(0.2, 0.2, 0.8),
        Sphere::new(vec3(0.0, 0.0, -1.01), 0.2),
    )];
    let mut rng = rand::thread_rng();
    let mut pixels = vec![Rgb::default(); 500 * 500];

    let dir = vec3(0.005, 0.0, 0.0);
    for i in 1..MOVE_TIMES {
        let ball = &mut shapes[0];         
        ball.transform.disp = dir * (i as f32);
        for x in 0..500 {
            for y in 0..500 {
                pixels[x * 500 + y] += sampling(&mut rng, &camera, &shapes, x as u32, y as u32);
            }
        }
    }

    let img = ImageBuffer::from_fn(500, 500, |x, y| {
        pixels[(x as usize) * 500 + (y as usize)].into()
    });
    let mut out = File::create("motion_blur.png").unwrap();
    image::ImageRgb8(img).save(&mut out, image::PNG).unwrap();
}
