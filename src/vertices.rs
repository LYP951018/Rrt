use math::{Vector2, Vector3};

pub trait Vertex {
    fn get_pos(&self) -> &Vector3;
}

macro_rules! impl_vertex {
    ($type: ty) => {
        impl Vertex for $type {
            fn get_pos(&self) -> &Vector3 {
                &self.pos
            }
        }
    };
}

pub struct VertexUV {
    pos: Vector3,
    uv: Vector2,
}

impl_vertex!(VertexUV);

pub struct VertexNormal {
    pos: Vector3,
    uv: Vector2,
}

impl_vertex!(VertexNormal);

pub struct VertexUvn {
    pos: Vector3,
    normal: Vector3,
    uv: Vector2,
}

impl_vertex!(VertexUvn);
