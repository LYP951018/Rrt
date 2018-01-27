
use std::ops::{Mul, Sub, Add};

pub fn lerp<T: Mul<f32, Output = T> + Sub<Output = T> + Add<Output = T> + Clone>(t: f32, a: T, b: T) -> T
{
    a.clone() + (b - a) * t
}
