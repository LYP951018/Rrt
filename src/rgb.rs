extern crate image;

use std::ops::{Add, Div, Mul};

#[derive(Debug, Copy, Clone)]
pub struct Rgb {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl<'a, 'b> Add<&'b Rgb> for &'a Rgb {
    type Output = Rgb;

    fn add(self, rhs: &'b Rgb) -> Self::Output {
        Rgb::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl_binop!(impl Add add for Rgb);

impl Mul<f32> for Rgb {
    type Output = Rgb;

    fn mul(self, rhs: f32) -> Self::Output {
        Rgb {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl Div<f32> for Rgb {
    type Output = Rgb;

    fn div(self, rhs: f32) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl Rgb {
    pub fn black() -> Rgb {
        Rgb {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }

    pub fn white() -> Rgb {
        Rgb {
            r: 1.0,
            g: 1.0,
            b: 1.0
        }
    }

    pub fn new(r: f32, g: f32, b: f32) -> Rgb {
        Rgb { r, g, b }
    }
}

impl From<Rgb> for image::Rgb<u8> {
    fn from(original: Rgb) -> image::Rgb<u8> {
        let r2u8 = |r: f32| -> u8 { (r * 255.0) as u8 };
        image::Rgb {
            data: [r2u8(original.r), r2u8(original.g), r2u8(original.b)],
        }
    }
}
