use crate::vector::Vec3;
use crate::color::Color;
use crate::texture::{Texture, Uv};

use std::rc::Rc;

pub struct CheckerTexture {
    odd: Rc<dyn Texture>,
    even: Rc<dyn Texture>,
    inv_scale: f64,
}

impl CheckerTexture {
    pub fn new(odd: Rc<dyn Texture>, even: Rc<dyn Texture>, scale: f64) -> Self {
        Self {
            odd,
            even,
            inv_scale: 1. / scale,
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, uv: &Uv, p: &Vec3) -> Color {
        let x_int = (self.inv_scale * p.x()).floor() as i32;
        let y_int = (self.inv_scale * p.y()).floor() as i32;
        let z_int = (self.inv_scale * p.z()).floor() as i32;

        if (x_int + y_int + z_int) % 2 == 0 {
            self.even.value(uv, p)
        } else {
            self.odd.value(uv, p)
        }
    }
}