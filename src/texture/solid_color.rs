use crate::color::Color;
use crate::vector::Vec3;
use crate::texture::{Texture, Uv};

use std::rc::Rc;

pub struct SolidColor {
    color: Color,
}

impl SolidColor {
    pub fn new(color: Color) -> Rc<dyn Texture> {
        Rc::new(Self {
            color
        })
    }

    pub fn from_rgb(r: f64, g: f64, b: f64) -> Rc<dyn Texture> {
        Self::new(Color::new(r, g, b))
    }
}

impl Texture for SolidColor {
    fn value(&self, _uv: &Uv, _p: &Vec3) -> Color {
        self.color.clone()
    }
}