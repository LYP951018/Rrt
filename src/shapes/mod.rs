use math::{Matrix, Vector3, Transformation, Transform};
use super::texture::{PureColorTexture, Texture};
use rgb::Rgb;

pub mod triangle;
pub mod sphere;

#[derive(Debug)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
    pub dir_inv: Vector3,
    pub neg: [bool; 3],
}

pub struct RayBuilder {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl RayBuilder {
    pub fn build(&self) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.direction,
            dir_inv: 1.0 / self.direction,
            neg: [
                self.direction.x < 0.0,
                self.direction.y < 0.0,
                self.direction.z < 0.0,
            ],
        }
    }
}

//TODO: 是否应该让 HitRecord 直接包含 Vertex？
#[derive(Debug)]
pub struct HitRecord {
    pub t: f32,
    pub pos: Vector3,
    pub normal: Vector3,
}

pub trait Shape {
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32, transform: &Matrix) -> Option<HitRecord>;
}

//optimization: isDirty?
pub struct TexedShape {
    pub texture: Box<Texture>,
    pub shape: Box<Shape>,
    pub transform: Transformation
}

impl TexedShape {
    pub fn hit(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
        self.shape.hit(ray, tmin, tmax, &self.transform.into())
    }
}

pub fn pure_color_shape<T: Shape + 'static>(color: Rgb, shape: T) -> TexedShape {
    TexedShape {
        texture: Box::new(PureColorTexture { color }),
        shape: Box::new(shape),
        transform: Transformation::one(),
    }
}

pub use triangle::*;
pub use sphere::*;