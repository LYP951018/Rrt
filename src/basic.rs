extern crate image;

use std::ops::*;
use std::iter::*;
use std::u8;
use std::f32;

#[derive(Debug, Clone, PartialEq)]
pub struct Vector3 {
    pub data: [f32; 3],
}

pub type Vector2 = (f32, f32);

#[derive(Debug, Copy, Clone)]
pub struct Rgb {
    pub r: f32,
    pub g: f32,
    pub b: f32
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
    pub fn black() -> Rgb {
        Rgb {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }

    pub fn new(r: f32, g: f32, b: f32) -> Rgb {
        Rgb {
            r, g, b
        }
    }
}

impl From<Rgb> for image::Rgb<u8> {
    fn from(original: Rgb) -> image::Rgb<u8> {
        let r2u8 = |r: f32| -> u8 {
            (r * 255.0) as u8
        };
        image::Rgb { 
            data: [r2u8(original.r), r2u8(original.g), r2u8(original.b) ]
        }
    }
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { data: [x, y, z] }
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
        Vector3::new(self.x() / len, self.y() / len, self.z() / len)
    }

    pub fn cross(&self, rhs: &Vector3) -> Vector3 {
        Vector3 {
            data: [
                self.y() * rhs.z() - self.z() * rhs.y(),
                self.z() * rhs.x() - self.x() * rhs.z(),
                self.x() * rhs.y() - self.y() * rhs.x(),
            ],
        }
    }
}

//& - &
impl<'a, 'b> Sub<&'b Vector3> for &'a Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: &'b Vector3) -> Self::Output {
        Vector3::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
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
        Vector3::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Self::Output {
        (&self).add(&rhs)
    }
}

impl<'a> Add<&'a Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: &'a Vector3) -> Self::Output {
        &self + rhs
    }
}

//& * f32
impl<'a> Mul<f32> for &'a Vector3 {
    type Output = Vector3;

    fn mul(self, scale: f32) -> Self::Output {
        Vector3::new(self.x() * scale, self.y() * scale, self.z() * scale)
    }
}

impl Mul<Vector3> for f32 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        &rhs * self
    }
}

struct Onb {
    u: Vector3,
    v: Vector3,
    w: Vector3,
}

#[derive(Debug)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

#[derive(Debug)]
pub struct HitRecord {
    pub t: f32,
    pub normal: Vector3,
    pub color: Rgb,
}

pub trait Shape {
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32, time: f32) -> Option<HitRecord>;
}

pub struct Triangle {
    pub p0: Vector3,
    pub p1: Vector3,
    pub p2: Vector3,
    pub color: Rgb
}

impl Triangle {
    pub fn new(p0: Vector3, p1: Vector3, p2: Vector3, color: Rgb) -> Self {
        Triangle {
            p0: p0.clone(),
            p1: p1.clone(),
            p2: p2.clone(),
            color
        }
    }

    pub fn with_pos(p0: Vector3, p1: Vector3, p2: Vector3) -> Self {
        Triangle::new(p0, p1, p2, Rgb::black())
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
    pub color: Rgb
}

impl Sphere {
    pub fn new(center: Vector3, radius: f32, color: Rgb) -> Self {
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

#[derive(Debug)]
pub struct ThinLens {
    radius: f32,
    center: Vector3,
    focal_length: f32
}

impl ThinLens {
    pub fn new(radius: f32, center: Vector3, focal_length: f32) -> ThinLens {
        ThinLens {
            radius, center, focal_length
        }
    }

    pub fn refract(&self, ray: &Ray, hit_pos: Vector3, s: f32) -> Ray {
        let i = s * self.focal_length / (s - self.focal_length);
        let dir = &self.center - &ray.origin;
        let distance = dir.length() * s / i;
        let dest = &dir.unit() * distance + &self.center;
        Ray {
            origin: hit_pos.clone(),
            direction: (&dest - &hit_pos).unit()
        }
    }
}

#[derive(Debug)]
pub struct Camera {
    lens: ThinLens,
    u: Vector3,
    v: Vector3,
    n: Vector3,
    origin: Vector3,
    left_bottom: Vector3
}

impl Camera {
    pub fn new(lens: ThinLens, at: Vector3, target: Vector3, up: Vector3, aspect_ratio: f32, fov: f32, dist: f32) -> Camera {
        let up = up.unit();
        let n = (&target - &at).unit();
        let u = n.cross(&up);
        let v = u.cross(&n);    
        let origin = &at - &(&n * dist);
        let half_width = fov.tan();
        let half_height = half_width / aspect_ratio;
        let left_bottom = &(&origin - &(&u * half_width)) - &(&v * half_height);
        println!("{:?}", left_bottom);
        Camera {
            lens,
            u, v, n,
            origin: at,
            left_bottom
        }
    }

    ///`x`, `y`: pixel coord.
    pub fn gen_ray(&self, x: f32, y: f32, lens_x: f32, lens_y: f32, s: f32) -> Ray {
        //1. transform pixel coord to world.
        //println!("v: {:?}", self.v);     
        let pos = &self.left_bottom + &(&self.u * x) + &(&self.v * y);
        let lens = &self.lens;
        let ux = lens_x * lens.radius;
        let uy = lens_y * lens.radius;
        let lens_pos = &self.u * ux + &self.v * uy + &lens.center;
        let new_dir = (&lens_pos - &pos).unit();
        lens.refract(
            &Ray {
           origin: pos,
           direction: new_dir
        }, lens_pos, s)
    }
}

#[cfg(test)]
mod tests {
    use basic::*;
    #[test]
    fn refract_tests() {
        let thinLens = ThinLens::new(20.0, Vector3::zero(), 5.0);
        let mut ray = Ray {
            origin: Vector3::new(0.0, 0.0, 2.0),
            direction: Vector3::new(-1.0, 0.0, 0.0)
        };
        let ray = thinLens.refract(&ray, Vector3::zero(), 10.0);
        assert_eq!(ray.origin, Vector3::zero());
        assert_eq!(ray.direction, Vector3::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn refract_tests_normal() {
        let thinLens = ThinLens::new(20.0, Vector3::zero(), 2.5);
        let mut ray = Ray {
            origin: Vector3::new(5.0, 5.0, 0.0),
            direction: Vector3::new(-1.0, 0.0, 0.0),
        };
        let ray = thinLens.refract(&ray, Vector3::new(0.0, 5.0, 0.0), 5.0);
        assert_eq!(ray.origin, Vector3::new(0.0, 5.0, 0.0));
        assert_eq!(ray.direction, Vector3::new(-5.0, -10.0, 0.0).unit());
    }

    #[test]
    fn vector_tests() {
        assert_eq!(Vector3::new(0.0, 0.0, -1.0).cross(&Vector3::new(0.0, 1.0, 0.0)), Vector3::new(1.0, 0.0, 0.0));
    }
}

