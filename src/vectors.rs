extern crate rand;

use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x, y, z }
    }

    pub fn zero() -> Vector3 {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn back() -> Vector3 {
        Self::new(0.0, 0.0, -1.0)
    }

    pub fn up() -> Vector3 {
        Self::new(0.0, 1.0, 0.0)
    }

    pub fn down() -> Vector3 {
        Self::new(0.0, -1.0, 0.0)
    }

    pub fn forward() -> Vector3 {
        Self::new(0.0, 0.0, 1.0)
    }

    pub fn left() -> Vector3 {
        Self::new(-1.0, 0.0, 0.0)
    }

    pub fn one() -> Vector3 {
        Self::new(1.0, 1.0, 1.0)
    }

    pub fn length(&self) -> f32 {
        self.squared_len().sqrt()
    }

    pub fn squared_len(&self) -> f32 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn dot(&self, rhs: &Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn unit(&self) -> Vector3 {
        let len = self.length();
        Vector3::new(self.x / len, self.y / len, self.z / len)
    }

    pub fn cross(&self, rhs: &Vector3) -> Vector3 {
        Vector3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

impl<'a, 'b> Add<&'b Vector3> for &'a Vector3 {
    type Output = Vector3;

    fn add(self, rhs: &'b Vector3) -> Self::Output {
        Vector3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl_binop!(impl Add add for Vector3);

impl<'a, 'b> Sub<&'b Vector3> for &'a Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: &'b Vector3) -> Self::Output {
        Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl_binop!(impl Sub sub for Vector3);

impl<'a> Mul<f32> for &'a Vector3 {
    type Output = Vector3;

    fn mul(self, scale: f32) -> Self::Output {
        Vector3::new(self.x * scale, self.y * scale, self.z * scale)
    }
}

impl_scalar!(impl Mul mul for Vector3);

impl<'a> Div<f32> for &'a Vector3 {
    type Output = Vector3;

    fn div(self, scale: f32) -> Self::Output {
        self * (1.0 / scale)
    }
}

impl_scalar!(impl Div div for Vector3);

#[derive(Copy, Clone, Debug, PartialEq, Rand)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Vector2 {
        Vector2 { x, y }
    }

    pub fn zero() -> Vector2 {
        Self::new(0.0, 0.0)
    }

    ///(0, 1) -> (-0.5, 0.5)
    pub fn to_center(&self) -> Vector2 {
        let transform = |x| (x * 2.0 - 1.0) / 2.0;
        Self::new(transform(self.x), transform(self.y))
    }
}

impl<'a, 'b> Add<&'b Vector2> for &'a Vector2 {
    type Output = Vector2;

    fn add(self, rhs: &'b Vector2) -> Self::Output {
        Vector2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl_binop!(impl Add add for Vector2);

impl<'a, 'b> Sub<&'b Vector2> for &'a Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: &'b Vector2) -> Self::Output {
        Vector2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl_binop!(impl Sub sub for Vector2);

impl<'a> Mul<f32> for &'a Vector2 {
    type Output = Vector2;

    fn mul(self, scale: f32) -> Self::Output {
        Vector2::new(self.x * scale, self.y * scale)
    }
}

impl_scalar!(impl Mul mul for Vector2);

impl<'a> Div<f32> for &'a Vector2 {
    type Output = Vector2;

    fn div(self, scale: f32) -> Self::Output {
        self * (1.0 / scale)
    }
}

impl_scalar!(impl Div div for Vector2);

#[cfg(test)]
mod tests {
    use super::vectors;

    #[test]
    fn cross() {
        assert_eq!(Vector3::new(0.0, 0.0, -1.0).cross(&Vector3::new(0.0, 1.0, 0.0)),);
    }
}
