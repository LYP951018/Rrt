extern crate image;

use std::ops::*;
use std::iter::*;

use self::image::Rgb;

#[derive(Debug, Clone)]
pub struct Vector3 {
    pub data: [f32; 3],
}

type Color = Rgb<f32>;

impl Vector3 {
    pub fn from_xyz(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { data: [x, y, z] }
    }

    pub fn zero() -> Vector3 {
        Self::from_xyz(0.0, 0.0, 0.0)
    }

    pub fn x(&self) -> f32 {
        self.data[0]
    }

    pub fn y(&self) -> f32 {
        self.data[1]
    }

    pub fn z(&self) -> f32 {
        self.data[2]
    }

    pub fn length(&self) -> f32 {
        self.squared_len().sqrt()
    }

    pub fn squared_len(&self) -> f32 {
        self.data.iter().map(|x| x.powi(2)).sum::<f32>()
    }

    pub fn dot(&self, rhs: &Self) -> f32 {
        self.data
            .iter()
            .zip(rhs.data.iter())
            .map(|(x, y)| x * y)
            .sum::<f32>()
    }

    pub fn unit(&self) -> Vector3 {
        let len = self.length();
        Vector3::from_xyz(self.x() / len, self.y() / len, self.z() / len)
    }

    pub fn cross(&self, rhs: &Vector3) -> Vector3 {
        Vector3 {
            data: [
                self.y() * rhs.z() - self.z() - rhs.y(),
                self.z() * rhs.x() - self.x() - rhs.z(),
                self.x() * rhs.y() - self.y() - rhs.x(),
            ],
        }
    }
}

impl<'a, 'b> Sub<&'b Vector3> for &'a Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: &'b Vector3) -> Self::Output {
        Vector3::from_xyz(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Self::Output {
        (&self).sub(&rhs)
    }
}

impl<'a, 'b> Add<&'b Vector3> for &'a Vector3 {
    type Output = Vector3;

    fn add(self, rhs: &'b Vector3) -> Self::Output {
        Vector3::from_xyz(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Self::Output {
        (&self).add(&rhs)
    }
}

impl Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, scale: f32) -> Self::Output {
        Vector3::from_xyz(self.x() * scale, self.y() * scale, self.z() * scale)
    }
}

impl Mul<Vector3> for f32 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        rhs * self
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
        let normalized_u = u.unit();
        let temp_v = normalized_u.cross(&n);
        let v = if temp_v.length() < Self::ONBEPSILON {
            normalized_u.cross(&m)
        } else {
            temp_v
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
            w: u.cross(&v),
        }
    }
}

pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

pub struct HitRecord {
    pub t: f32,
    pub normal: Vector3,
    pub color: Color,
}

pub trait Shape {
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32, time: f32) -> Option<HitRecord>;
}

pub struct Triangle {
    pub p0: Vector3,
    pub p1: Vector3,
    pub p2: Vector3,
    pub color: Color
}

impl Triangle {
    pub fn new(p0: Vector3, p1: Vector3, p2: Vector3, color: Color) -> Self {
        Triangle {
            p0: p0.clone(),
            p1: p1.clone(),
            p2: p2.clone(),
            color
        }
    }

    pub fn with_pos(p0: Vector3, p1: Vector3, p2: Vector3) -> Self {
        Triangle::new(p0, p1, p2, Color {data: [0.0, 0.0, 0.0]})
    }
}

impl Shape for Triangle {
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32, time: f32) -> Option<HitRecord> {
        let a = self.p0.x() - self.p1.x();
        let b = self.p0.y() - self.p1.y();
        let c = self.p0.z() - self.p1.z();

        let d = self.p0.x() - self.p2.x();
        let e = self.p0.y() - self.p2.y();
        let f = self.p0.z() - self.p2.z();

        let g = ray.direction.x();
        let h = ray.direction.y();
        let i = ray.direction.z();

        let j = self.p0.x() - ray.origin.x();
        let k = self.p0.y() - ray.origin.y();
        let l = self.p0.z() - ray.origin.z();

        let eihf = e * i - h * f;
        let gfdi = g * f - d * i;
        let dheg = d * h - e * g;

        let denom = a * eihf + b * gfdi + c * dheg;
        let beta = (j * eihf + k * gfdi + l * dheg) / denom;

        if beta <= 0.0 || beta >= 1.0 {
            None
        } else {
            let akjb = a * k - j * b;
            let jcal = j * c - a * l;
            let blkc = b * l - k * c;

            let gamma = (i * akjb + h * jcal + g * blkc) / denom;
            if gamma <= 0.0 || beta + gamma >= 1.0 {
                None
            } else {
                let tval = -(f * akjb + e * jcal + g * blkc) / denom;
                let vec = &self.p2 - &self.p0;
                if tval >= tmin && tval <= tmax {
                    Some(HitRecord {
                        t: tval,
                        normal: ((&self.p1 - &self.p0).cross(&vec)).unit(),
                        color: self.color.clone()
                    })
                } else {
                    None
                }
            }
        }
    }
}

pub struct Sphere {
    pub center: Vector3,
    pub radius: f32,
    pub color: Color
}

impl Sphere {
    pub fn new(center: Vector3, radius: f32, color: Color) -> Self {
        Sphere {
            center, radius, color
        }
    }
}

impl Shape for Sphere {
    // (x - c_x)^2 + (y - c_y)^2 + (z - c_z)^2 - R^2 = 0
    // (p - c) . (p - c) - R^2 = 0 (dot product)
    // (o + td - c) . (o + td - c) - R^2 = 0
    // solve the equation.
    // normal: 
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32, time: f32) -> Option<HitRecord> {
        let temp = &ray.origin - &self.center;
        
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&temp);
        let c = temp.dot(&temp) - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;
        if discriminant > 0.0 {
            let discriminant = discriminant.sqrt();
            let mut t = (-b - discriminant) / 2.0 * a;
            if t < tmin {
                t = (-b + discriminant) / (2.0 * a);
            }
            if t < tmin || t > tmax {
                None
            } else {
                let dir = t * ray.direction.clone();
                let point = &ray.origin + &dir;
                let normal = &point - &self.center;
                let normal = normal.unit();
                Some(HitRecord {
                    t,
                    normal,
                    color: self.color.clone()
                })
            }           
        } else {
            None
        }      
    }
}