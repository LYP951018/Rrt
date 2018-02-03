use math::*;
use {HitRecord, Ray, Shape};

#[derive(Copy, Clone)]
pub struct Sphere {
    pub center: Vector3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Vector3, radius: f32) -> Self {
        Sphere { center, radius }
    }
}

impl Shape for Sphere {
    // (x - c_x)^2 + (y - c_y)^2 + (z - c_z)^2 - R^2 = 0
    // (p - c) . (p - c) - R^2 = 0 (dot product)
    // (o + td - c) . (o + td - c) - R^2 = 0
    // solve the equation.
    // normal:
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32, transform: &Matrix) -> Option<HitRecord> {
        let center = transform * make_pos(&self.center);
        let temp = &make_pos(&ray.origin) - &center;
        let ray_dir = make_dir(&ray.direction);
        let ray_origin = make_pos(&ray.origin);
        let a = ray_dir.magnitude2();
        let b = 2.0 * ray_dir.dot(temp);
        let c = temp.magnitude2() - self.radius * self.radius;

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
                let dir = t * &ray_dir;
                let point = &ray_origin + &dir;
                let normal = &point - &center;
                let normal = normal.normalize();
                let delta = &point - &center;
                // let theta = (delta.z / self.radius).acos();
                // let phi = f32::atan2(delta.y, delta.x);
                Some(HitRecord {
                    t,
                    normal: normal.truncate(),
                    pos: point.truncate()
                    // color: texture.get_value(&point, &Vector2::new(theta / 2.0 * f32::consts::PI, phi / f32::consts::PI)),
                })
            }
        } else {
            None
        }
    }
}