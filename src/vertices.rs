use vectors::{Vector2, Vector3};

struct VertexUV {
    vertex: Vector3,
    uv: Vector2
}

struct VertexNormal {
    vertex: Vector3,
    uv: Vector2
}

struct VertexUvn {
    vertex: Vector3,
    normal: Vector3,
    uv: Vector2
}