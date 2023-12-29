use crate::material::Material;
use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::color::Color;

use rand::Rng;

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self {
            refraction_index,
        }
    }

    fn reflectance(cos: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0 * r0;

        r0 + (1.0 - r0) * (1.0 - cos).powf(5.0)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let attenuation = Color::white();
        let refraction_ratio = if hit_record.front_face { 1.0 / self.refraction_index } else { self.refraction_index };

        let unit_direction = ray_in.direction().normalized();
        let cos_theta = (-unit_direction).dot(&hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let mut rng = rand::thread_rng();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let ray_out = if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > rng.gen_range(0f64..1f64) {
            // Cannot refract, so reflect (or reflect with probability base on the reflectance)
            unit_direction.reflect(&hit_record.normal)
        } else {
            unit_direction.refract(&hit_record.normal, refraction_ratio)
        };

        let scattered = Ray::new(hit_record.point, ray_out);
        Some((attenuation, scattered))
    }
}