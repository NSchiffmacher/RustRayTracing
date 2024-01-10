use crate::color::Color;
use crate::vector::Vec3;
use crate::texture::{Texture, Uv};
use crate::noise::Perlin;

use std::rc::Rc;

pub struct NoiseTexture {
    noise_generator: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Rc<dyn Texture> {
        Rc::new(Self {
            noise_generator: Perlin::new(),
            scale,
        })
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _uv: &Uv, p: &Vec3) -> Color {
        // let noise = self.noise_generator.turb(&(*p * self.scale), 7);
        let noise = 0.5 * (1. + (p.z() + 10. * self.noise_generator.turb(&(*p * self.scale), 7)).sin());
        Color::white() * noise
    }
}