use crate::material::Material;
use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::color::Color;
use crate::vector::Vec3;

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self {
            albedo,
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let mut direction_out = hit_record.normal + Vec3::random_unit_vector();
        if direction_out.near_zero() {
            direction_out = hit_record.normal;
        }

        let ray_out = Ray::new(hit_record.point, direction_out);
        Some((self.albedo.clone(), ray_out))
    }
}