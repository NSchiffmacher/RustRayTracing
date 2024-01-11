use crate::material::Material;
use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::color::Color;
use crate::vector::Vec3;
use crate::texture::{Texture, SolidColor};

use std::rc::Rc;

pub struct Isotropic {
    albedo: Rc<dyn Texture>,
}

impl Isotropic {
    pub fn new(albedo: Rc<dyn Texture>) -> Rc<dyn Material> {
        Rc::new(Self {
            albedo,
        })
    }

    pub fn from_color(color: Color) -> Rc<dyn Material> {
        Self::new(SolidColor::new(color))
    }
}

impl Material for Isotropic {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let scattered = Ray::new(hit_record.point, Vec3::random_unit_vector(), ray_in.time());
        let attenuation = self.albedo.value(&hit_record.uv, &hit_record.point);
        Some((attenuation, scattered))
    }
}