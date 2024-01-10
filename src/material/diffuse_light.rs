use crate::material::Material;
use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::color::Color;
use crate::vector::Vec3;
use crate::texture::{Texture, SolidColor, Uv};

use std::rc::Rc;

pub struct DiffuseLight {
    emit: Rc<dyn Texture>
}

impl DiffuseLight {
    pub fn new(emit: Rc<dyn Texture>) -> Rc<dyn Material> {
        Rc::new(Self { emit })
    }

    pub fn from_color(color: Color) -> Rc<dyn Material> {
        Rc::new(Self { emit: SolidColor::new(color) })
    }

    pub fn white(intensity: f64) -> Rc<dyn Material> {
        Self::from_color(Color::white() * intensity)
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _ray_in: &Ray, _hit_record: &HitRecord) -> Option<(Color, Ray)> {
        None
    }

    fn emitted(&self, uv: &Uv, p: &Vec3) -> Color {
        self.emit.value(uv, p)
    }
}