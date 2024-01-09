use crate::material::Material;
use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::color::Color;
use crate::vector::Vec3;
use crate::texture::{Texture, SolidColor};

use std::rc::Rc;

pub struct Lambertian {
    albedo: Rc<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self {
            albedo: Rc::new(SolidColor::new(albedo)),
        }
    }

    pub fn from_texture(albedo: Rc<dyn Texture>) -> Self {
        Self {
            albedo,
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let mut direction_out = hit_record.normal + Vec3::random_unit_vector();
        if direction_out.near_zero() {
            direction_out = hit_record.normal;
        }

        let ray_out = Ray::new(hit_record.point, direction_out, ray_in.time());
        let attenuation = self.albedo.value(&hit_record.uv, &hit_record.point);
        Some((attenuation, ray_out))
    }
}