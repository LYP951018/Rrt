extern crate cgmath;
extern crate rand;

pub use self::cgmath::*;
use std::ops::{Add, Mul, Sub};

pub type Vector4 = cgmath::Vector4<f32>;
pub type Vector3 = cgmath::Vector3<f32>;
pub type Vector2 = cgmath::Vector2<f32>;
pub type Quaternion = cgmath::Quaternion<f32>;

pub type Matrix = cgmath::Matrix4<f32>;

pub type Transformation = cgmath::Decomposed<cgmath::Vector3<f32>, cgmath::Quaternion<f32>>;

pub fn lerp<T: Mul<f32, Output = T> + Sub<Output = T> + Add<Output = T> + Clone>(
    t: f32,
    a: T,
    b: T,
) -> T {
    a.clone() + (b - a) * t
}

pub fn make_pos(vec3: &Vector3) -> Vector4 {
    vec3.extend(1.0)
}

pub fn make_dir(vec3: &Vector3) -> Vector4 {
    vec3.extend(0.0)
}
