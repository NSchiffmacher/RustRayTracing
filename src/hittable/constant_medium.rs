use crate::vector::Vec3;
use crate::hittable::{HitRecord, Hittable, AABB};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::color::Color;
use crate::material::{Material, Isotropic};
use crate::texture::{Uv, Texture, SolidColor};

use std::rc::Rc;
use rand::Rng;

pub struct ConstantMedium {
    boundary: Rc<dyn Hittable>,
    inv_neg_density: f64,
    phase_function: Rc<dyn Material>,
}

impl ConstantMedium {
    pub fn new(boundary: Rc<dyn Hittable>, density: f64, a: Rc<dyn Texture>) -> Box<dyn Hittable> {
        Box::new(Self {
            boundary,
            inv_neg_density: -1.0 / density,
            phase_function: Isotropic::new(a),
        })
    }

    pub fn from_color(boundary: Rc<dyn Hittable>, density: f64, color: Color) -> Box<dyn Hittable> {
        Self::new(boundary, density, SolidColor::new(color))
    }
}

impl Hittable for ConstantMedium {
    fn bounding_box(&self) -> AABB {
        self.boundary.bounding_box()
    }

    fn hit(&self, ray: &Ray, _ray_t: &Interval) -> Option<HitRecord> {
        let mut hit_record1 = if let Some(hr) = self.boundary.hit(ray, &Interval::universe()) {
            hr
        } else {
            return None;
        };

        let hit_record2 = if let Some(hr) = self.boundary.hit(ray, &Interval::new(hit_record1.t + 0.0001, f64::INFINITY)) {
            hr
        } else {
            return None;
        };

        if hit_record1.t >= hit_record2.t {
            return None;
        }

        if hit_record1.t < 0. {
            hit_record1.t = 0.;
        }

        let mut rng = rand::thread_rng();

        let ray_length = ray.direction().length();
        let distance_inside_boundary = (hit_record2.t - hit_record1.t) * ray_length;
        let hit_distance = self.inv_neg_density * f64::ln(rng.gen_range(0.0..1.0));

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let t = hit_record1.t + hit_distance / ray_length;
        Some(HitRecord::new(
            ray.at(t),
            Vec3::new(1., 0., 0.), // Arbitrary
            t,
            Uv { u: 0., v: 0. }, // Arbitrary UV
            ray,
            self.phase_function.clone(),
        ))
    }
}