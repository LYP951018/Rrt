use math::{make_pos, InnerSpace, Matrix, Vector3};
use {HitRecord, Ray, Shape};

pub struct Triangle {
    pub p0: Vector3,
    pub p1: Vector3,
    pub p2: Vector3,
}

impl Triangle {
    pub fn new(p0: Vector3, p1: Vector3, p2: Vector3) -> Self {
        Triangle { p0, p1, p2 }
    }
}

impl Shape for Triangle {
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32, transform: &Matrix) -> Option<HitRecord> {
        let p0 = transform * make_pos(&self.p0);
        let p1 = transform * make_pos(&self.p1);
        let p2 = transform * make_pos(&self.p2);

        let a = p0.x - p1.x;
        let b = p0.y - p1.y;
        let c = p0.z - p1.z;

        let d = p0.x - p2.x;
        let e = p0.y - p2.y;
        let f = p0.z - p2.z;

        let g = ray.direction.x;
        let h = ray.direction.y;
        let i = ray.direction.z;

        let j = p0.x - ray.origin.x;
        let k = p0.y - ray.origin.y;
        let l = p0.z - ray.origin.z;

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
                let vec = &p2 - &p0;
                if tval >= tmin && tval <= tmax {
                    Some(HitRecord {
                        t: tval,
                        normal: ((&p1 - &p0).truncate().cross(vec.truncate())).normalize(),
                        pos: ray.origin + ray.direction * tval,
                    })
                } else {
                    None
                }
            }
        }
    }
}