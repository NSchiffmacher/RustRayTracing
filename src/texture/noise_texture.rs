use crate::color::Color;
use crate::vector::Vec3;
use crate::texture::{Texture, Uv};
use crate::noise::Perlin;

pub struct NoiseTexture {
    noise_generator: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        Self {
            noise_generator: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _uv: &Uv, p: &Vec3) -> Color {
        Color::white() * 0.5 * (1. + self.noise_generator.noise(&(*p * self.scale)))
    }
}