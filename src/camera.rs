use vectors::{Vector2, Vector3};
use shapes::Ray;

#[derive(Debug, Clone)]
pub struct ThinLens {
    pub radius: f32,
    pub center: Vector3,
    pub focal_length: f32,
}

impl ThinLens {
    pub fn new(radius: f32, center: Vector3, focal_length: f32) -> ThinLens {
        ThinLens {
            radius,
            center,
            focal_length,
        }
    }

    pub fn refract(&self, ray: &Ray, hit_pos: Vector3, s: f32) -> Ray {
        let i = s * self.focal_length / (s - self.focal_length);
        let dir = &self.center - &ray.origin;
        let distance = dir.length() * s / i;
        let dest = &dir.unit() * distance + &self.center;
        Ray {
            origin: hit_pos.clone(),
            direction: (&dest - &hit_pos).unit(),
        }
    }
}

#[derive(Debug)]
pub struct Camera {
    pub lens: ThinLens,
    u: Vector3,
    v: Vector3,
    n: Vector3,
    origin: Vector3,
    left_bottom: Vector3,
}

pub struct CameraBuilder {
    pub lens: ThinLens,
    pub at: Vector3,
    pub target: Vector3,
    pub up: Vector3,
    pub aspect_ratio: f32,
    pub fov: f32,
    pub dist: f32,
}

impl CameraBuilder {
    pub fn build(&self) -> Camera {
        let up = self.up.unit();
        let n = (&self.target - &self.at).unit();
        let u = n.cross(&up);
        let v = u.cross(&n);
        let origin = &self.at - &(&n * self.dist);
        let half_width = self.fov.tan();
        let half_height = half_width / self.aspect_ratio;
        let left_bottom = &(&origin - &(&u * half_width)) - &(&v * half_height);
        Camera {
            lens: self.lens.clone(),
            u,
            v,
            n,
            origin: self.at.clone(),
            left_bottom,
        }
    }
}

impl Camera {
    ///`x`, `y`: pixel coord.
    pub fn gen_ray(&self, pixel: &Vector2, lens_pos: &Vector2, s: f32) -> Ray {
        //1. transform pixel coord to world.
        let pos = &self.left_bottom + &self.u * pixel.x + &self.v * pixel.y;
        let lens = &self.lens;
        let ux = lens_pos.x * 2.0 * lens.radius;
        let uy = lens_pos.y * 2.0 * lens.radius;
        let lens_pos = &self.u * ux + &self.v * uy + &lens.center;
        let new_dir = (&lens_pos - &pos).unit();
        lens.refract(
            &Ray {
                origin: pos,
                direction: new_dir,
            },
            lens_pos,
            s,
        )
    }
}



#[cfg(test)]
mod tests {
    use basic::*;
    #[test]
    fn refract() {
        let thinLens = ThinLens::new(20.0, Vector3::zero(), 5.0);
        let mut ray = Ray {
            origin: Vector3::new(0.0, 0.0, 2.0),
            direction: Vector3::new(-1.0, 0.0, 0.0),
        };
        let ray = thinLens.refract(&ray, Vector3::zero(), 10.0);
        assert_eq!(ray.origin, Vector3::zero());
        assert_eq!(ray.direction, Vector3::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn refract_normal() {
        let thinLens = ThinLens::new(20.0, Vector3::zero(), 2.5);
        let mut ray = Ray {
            origin: Vector3::new(5.0, 5.0, 0.0),
            direction: Vector3::new(-1.0, 0.0, 0.0),
        };
        let ray = thinLens.refract(&ray, Vector3::new(0.0, 5.0, 0.0), 5.0);
        assert_eq!(ray.origin, Vector3::new(0.0, 5.0, 0.0));
        assert_eq!(ray.direction, Vector3::new(-5.0, -10.0, 0.0).unit());
    }
}
