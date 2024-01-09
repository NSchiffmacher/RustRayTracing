use crate::vector::Vec3;
use crate::color::Color;

mod image_texture;
mod solid_color;
mod checker_texture;
mod noise_texture;

pub use image_texture::ImageTexture;
pub use solid_color::SolidColor;
pub use checker_texture::CheckerTexture;
pub use noise_texture::NoiseTexture;

#[derive(Copy, Clone, Debug)]
pub struct Uv {
    pub u: f64,
    pub v: f64,
}

pub trait Texture {
    fn value(&self, uv: &Uv, p: &Vec3) -> Color;
}