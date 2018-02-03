extern crate rand;
extern crate cgmath;

pub use self::cgmath::{prelude, InnerSpace};
use std::ops::{Mul, Sub, Add};

pub type Vector4 = cgmath::Vector4<f32>;
pub type Vector3 = cgmath::Vector3<f32>;
pub type Vector2 = cgmath::Vector2<f32>;

pub type Matrix = cgmath::Matrix4<f32>;

pub fn lerp<T: Mul<f32, Output = T> + Sub<Output = T> + Add<Output = T> + Clone>(t: f32, a: T, b: T) -> T
{
    a.clone() + (b - a) * t
}

pub fn make_pos(vec3: &Vector3) -> Vector4 {
    Vector4::new(vec3.x, vec3.y, vec3.z, 1.0)
}

pub fn make_dir(vec3: &Vector3) -> Vector4 {
    Vector4::new(vec3.x, vec3.y, vec3.z, 0.0)
}