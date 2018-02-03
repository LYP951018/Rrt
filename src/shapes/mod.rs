use math::{Vector3, Matrix};
use super::texture::Texture;

#[derive(Debug)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

//TODO: 是否应该让 HitRecord 直接包含 Vertex？
#[derive(Debug)]
pub struct HitRecord {
    pub t: f32,
    pub pos: Vector3,   
    pub normal: Vector3,
}

pub trait Shape {
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32, 
        transform: &Matrix) -> Option<HitRecord>;
}

//optimization: isDirty?
pub struct TexedShape {
    pub texture: Box<Texture>,
    pub shape: Box<Shape>,
    pub transform: Matrix
}

impl TexedShape {
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
        self.shape.hit(ray, tmin, tmax, &self.transform)
    }
}

mod triangle;
mod sphere;