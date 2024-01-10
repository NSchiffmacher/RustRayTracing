use crate::material::Material;
use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::color::Color;
use crate::vector::Vec3;

use std::rc::Rc;

pub struct Metal {
    albedo: Color,
    fuzz_factor: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz_factor: f64) -> Rc<dyn Material> {
        Rc::new(Self {
            albedo,
            fuzz_factor: 1f64.min(fuzz_factor),
        })
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = ray_in.direction().reflect(&hit_record.normal) + self.fuzz_factor * Vec3::random_unit_vector();
        if reflected.dot(&hit_record.normal) <= 0. {
            return None;
        }

        let scattered = Ray::new(hit_record.point, reflected, ray_in.time());
        Some((self.albedo.clone(), scattered))
    }
}