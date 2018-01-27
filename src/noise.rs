use vectors::{Vector2, Vector3};
use util::lerp;
use rgb::Rgb;
use texture::Texture;

static PERMUTATIONS: [usize; 512] = [
    151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7, 225, 140, 36, 103, 30, 69,
    142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234, 75, 0, 26, 197, 62, 94, 252, 219,
    203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174, 20, 125, 136, 171, 168, 68, 175,
    74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83, 111, 229, 122, 60, 211, 133, 230,
    220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25, 63, 161, 1, 216, 80, 73, 209, 76,
    132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188, 159, 86, 164, 100, 109, 198, 173,
    186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147, 118, 126, 255, 82, 85, 212, 207, 206,
    59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170, 213, 119, 248, 152, 2, 44, 154, 163,
    70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39, 253, 19, 98, 108, 110, 79, 113, 224, 232,
    178, 185, 112, 104, 218, 246, 97, 228, 251, 34, 242, 193, 238, 210, 144, 12, 191, 179, 162,
    241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31, 181, 199, 106, 157, 184, 84, 204,
    176, 115, 121, 50, 45, 127, 4, 150, 254, 138, 236, 205, 93, 222, 114, 67, 29, 24, 72, 243, 141,
    128, 195, 78, 66, 215, 61, 156, 180, 151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194,
    233, 7, 225, 140, 36, 103, 30, 69, 142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234,
    75, 0, 26, 197, 62, 94, 252, 219, 203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174,
    20, 125, 136, 171, 168, 68, 175, 74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83,
    111, 229, 122, 60, 211, 133, 230, 220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25,
    63, 161, 1, 216, 80, 73, 209, 76, 132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188,
    159, 86, 164, 100, 109, 198, 173, 186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147,
    118, 126, 255, 82, 85, 212, 207, 206, 59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170,
    213, 119, 248, 152, 2, 44, 154, 163, 70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39, 253,
    19, 98, 108, 110, 79, 113, 224, 232, 178, 185, 112, 104, 218, 246, 97, 228, 251, 34, 242, 193,
    238, 210, 144, 12, 191, 179, 162, 241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31,
    181, 199, 106, 157, 184, 84, 204, 176, 115, 121, 50, 45, 127, 4, 150, 254, 138, 236, 205, 93,
    222, 114, 67, 29, 24, 72, 243, 141, 128, 195, 78, 66, 215, 61, 156, 180,
];

fn constrain(n: u32) -> usize {
    (n & 255) as usize
}

fn fade(n: f32) -> f32 {
    n * n * n * (n * (n * 6.0 - 15.0) + 10.0)
}

fn hash(x: u32) -> u32 {
    PERMUTATIONS[constrain(x)] as u32
}

fn fake_dot(h: u32, x: f32, y: f32, z: f32) -> f32 {
    match h & 15 {
        0 => x + y,
        1 => -x + y,
        2 => x - y,
        3 => -x - y,
        4 => x + z,
        5 => -x + z,
        6 => x - z,
        7 => -x - z,
        8 => y + z,
        9 => -y + z,
        10 => y - z,
        11 => -y - z,
        12 => y + x,
        13 => -y + z,
        14 => y - x,
        15 => -y - z,
        _ => panic!(),
    }
}

pub fn noise(point: &Vector3) -> f32 {
    let Vector3 { x, y, z } = *point;
    let (floor_x, floor_y, floor_z) = (x.floor(), y.floor(), z.floor());
    let (xi, yi, zi) = (floor_x as u32, floor_y as u32, floor_z as u32);
    let (xf, yf, zf) = (x - floor_x, y - floor_y, z - floor_z);
    let (u, v, w) = (fade(xf), fade(yf), fade(zf));
    let a = hash(xi);
    let b = hash(xi + 1);
    let c = hash(a + yi);
    let d = hash(a + yi + 1);
    let e = hash(b + yi);
    let f = hash(b + yi + 1);
    let dot = |n: u32, x1: f32, y1: f32, z1: f32| fake_dot(hash(n + zi), x1, y1, z1);
    let lerp_x1 = lerp(u, dot(c, xf, yf, zf), dot(e, xf - 1.0, yf, zf));
    let lerp_x2 = lerp(u, dot(d, xf, yf - 1.0, zf), dot(f, xf - 1.0, yf - 1.0, zf));
    let lerp_x3 = lerp(
        u,
        dot(c + 1, xf, yf, zf - 1.0),
        dot(e + 1, xf - 1.0, yf, zf - 1.0),
    );
    let lerp_x4 = lerp(
        u,
        dot(d + 1, xf, yf - 1.0, zf - 1.0),
        dot(f + 1, xf - 1.0, yf - 1.0, zf - 1.0),
    );
    (lerp(w, lerp(v, lerp_x1, lerp_x2), lerp(v, lerp_x3, lerp_x4)) + 1.0) / 2.0
}

struct NoiseTexture {
    start: Rgb,
    end: Rgb,
    scale: f32,
}

impl Texture for NoiseTexture {
    fn get_value(&self, pos: &Vector3, _uv: &Vector2) -> Rgb {
        let noise = noise(&(pos * self.scale));
        lerp(noise, self.start, self.end)
    }
}
