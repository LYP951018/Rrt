
use std::ops::*;
use std::iter::*;

#[derive(Debug)]
pub struct Rgb {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

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
    fn black() -> Rgb {
        Rgb {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }
}

#[derive(Debug)]
pub struct Vector3 {
    pub data: [f32; 3],
}

impl Vector3 {
    fn from_xyz(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { data: [x, y, z] }
    }

    fn zero() -> Vector3 {
        Self::from_xyz(0.0, 0.0, 0.0)
    }

    fn x(&self) -> f32 {
        self.data[0]
    }

    fn y(&self) -> f32 {
        self.data[1]
    }

    fn z(&self) -> f32 {
        self.data[2]
    }

    fn length(&self) -> f32 {
        self.squared_len().sqrt()
    }

    fn squared_len(&self) -> f32 {
        self.data.iter().map(|x| x.powi(2)).sum::<f32>()
    }

    fn dot(&self, rhs: &Self) -> f32 {
        self.data
            .iter()
            .zip(rhs.data.iter())
            .map(|(x, y)| x * y)
            .sum::<f32>()
    }

    fn unit(&self) -> Vector3 {
        let len = self.length();
        Vector3::from_xyz(self.x() / len, self.y() / len, self.z() / len)
    }

    fn cross(&self, rhs: &Self) -> Vector3 {
        Vector3 {
            data: [
                self.y() * rhs.z() - self.z() - rhs.y(),
                self.z() * rhs.x() - self.x() - rhs.z(),
                self.x() * rhs.y() - self.y() - rhs.x(),
            ],
        }
    }
}

struct Onb {
    u: Vector3,
    v: Vector3,
    w: Vector3,
}

impl Onb {
    const ONBEPSILON: f32 = 0.01;

    fn from_u(u: Vector3) -> Onb {
        let n = Vector3::from_xyz(1.0, 0.0, 0.0);
        let m = Vector3::from_xyz(0.0, 1.0, 0.0);
        let normalizedU = u.unit();
        let tempV = normalizedU.cross(&n);
        let v = if tempV.length() < Self::ONBEPSILON {
            normalizedU.cross(&m)
        } else {
            tempV
        };
        let w = v.cross(&u);
        Onb { u, v, w }
    }

    fn from_uv(u: Vector3, v: Vector3) -> Onb {
        let fu = u.unit();
        let fv = v.unit();
        let normal = fu.cross(&fv);
        Onb {
            u: fu,
            v: normal,
            w: u.cross(&v)
        }
    }
}

pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3
}

pub struct HitRecord {
    pub t: f32,
    pub normal: Vector3,
    pub rgb: Rgb
}

pub trait Shape {
    fn hit(ray: &Ray, tmin: f32, tmax: f32, time: f32) -> Option<HitRecord>;
    fn shadow_hit(ray: &Ray, tmin: f32, tmax: f32, time: f32) -> bool;
}

pub struct Triangle {
    pub a: Vector3,
    pub b: Vector3,
    pub c: Vector3
}

