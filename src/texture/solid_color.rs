use crate::color::Color;
use crate::vector::Vec3;
use crate::texture::{Texture, Uv};

pub struct SolidColor {
    color: Color,
}

impl SolidColor {
    pub fn new(color: Color) -> Self {
        Self {
            color
        }
    }

    pub fn from_rgb(r: f64, g: f64, b: f64) -> Self {
        Self {
            color: Color::new(r, g, b)
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, _uv: &Uv, _p: &Vec3) -> Color {
        self.color.clone()
    }
}