extern crate image;

use rgb::Rgb;
use math::*;
use self::image::ImageBuffer;

pub trait Texture {
    fn get_value(&self, pos: &Vector3, uv: &Vector2) -> Rgb;
}

type ImgBuf<P: image::Pixel> = ImageBuffer<P, Vec<P::Subpixel>>;

pub struct ImageTexture {
    image: ImgBuf<image::Rgb<u8>>,
}

impl Texture for ImageTexture {
    fn get_value(&self, _pos: &Vector3, uv: &Vector2) -> Rgb {
        let image = &self.image;
        let width = image.width();
        let height = image.height();
        let Vector2 { x: u, y: v } = uv.mul_element_wise(vec2(width as f32, height as f32));
        let ud = u - u.floor();
        let vd = v - v.floor();
        let ui = u.floor() as u32;
        let vi = v.floor() as u32;

        let a = Rgb::from(image.get_pixel(ui, vi).clone());
        let b = Rgb::from(image.get_pixel(ui + 1, vi).clone());
        let c = Rgb::from(image.get_pixel(ui, vi + 1).clone());
        let d = Rgb::from(image.get_pixel(ui + 1, vi + 1).clone());

        Rgb::from(lerp(vd, lerp(ud, a, b), lerp(ud, c, d)))
    }
}

pub struct PureColorTexture {
    pub color: Rgb,
}

impl Texture for PureColorTexture {
    fn get_value(&self, _pos: &Vector3, _uv: &Vector2) -> Rgb {
        self.color
    }
}
