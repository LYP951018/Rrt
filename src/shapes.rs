use vectors::Vector3;
use rgb::Rgb;

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
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32, translation: &Vector3) -> Option<HitRecord>;
}

pub struct Triangle {
    pub p0: Vector3,
    pub p1: Vector3,
    pub p2: Vector3,
    pub color: Rgb,
}

impl Triangle {
    pub fn new(p0: Vector3, p1: Vector3, p2: Vector3, color: Rgb) -> Self {
        Triangle {
            p0: p0.clone(),
            p1: p1.clone(),
            p2: p2.clone(),
            color,
        }
    }

    pub fn with_pos(p0: Vector3, p1: Vector3, p2: Vector3) -> Self {
        Triangle::new(p0, p1, p2, Rgb::black())
    }
}

impl Shape for Triangle {
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32, translation: &Vector3) -> Option<HitRecord> {
        let a = self.p0.x - self.p1.x;
        let b = self.p0.y - self.p1.y;
        let c = self.p0.z - self.p1.z;

        let d = self.p0.x - self.p2.x;
        let e = self.p0.y - self.p2.y;
        let f = self.p0.z - self.p2.z;

        let g = ray.direction.x;
        let h = ray.direction.y;
        let i = ray.direction.z;

        let j = self.p0.x - ray.origin.x;
        let k = self.p0.y - ray.origin.y;
        let l = self.p0.z - ray.origin.z;

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
                        color: self.color.clone(),
                    })
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct Sphere {
    pub center: Vector3,
    pub radius: f32,
    pub color: Rgb,
}

impl Sphere {
    pub fn new(center: Vector3, radius: f32, color: Rgb) -> Self {
        Sphere {
            center,
            radius,
            color,
        }
    }
}

impl Shape for Sphere {
    // (x - c_x)^2 + (y - c_y)^2 + (z - c_z)^2 - R^2 = 0
    // (p - c) . (p - c) - R^2 = 0 (dot product)
    // (o + td - c) . (o + td - c) - R^2 = 0
    // solve the equation.
    // normal:
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32, translation: &Vector3) -> Option<HitRecord> {
        let new_center = self.center + translation;
        let temp = &ray.origin - &new_center;

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
                let dir = t * &ray.direction;
                let point = &ray.origin + &dir;
                let normal = &point - &new_center;
                let normal = normal.unit();
                Some(HitRecord {
                    t,
                    normal,
                    color: self.color.clone(),
                })
            }
        } else {
            None
        }
    }
}
