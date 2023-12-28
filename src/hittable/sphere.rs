use crate::vector::Point;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;

pub struct Sphere {
    center: Point,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point, radius: f64) -> Self {
        Self {
            center,
            radius,
        }
    }

    pub fn boxed(center: Point, radius: f64) -> Box<Self> {
        Box::new(Self::new(center, radius))
    }

    pub fn center(&self) -> &Point {
        &self.center
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let oc = *ray.origin() - self.center;
        let a = ray.direction().dot(&ray.direction());
        let b_half = ray.direction().dot(&oc);
        let c = oc.dot(&oc) - self.radius * self.radius;

        let delta_quarter = b_half * b_half -  a * c;
        if delta_quarter < 0. {
            return None;
        } 
        
        for t_root in [(-b_half - delta_quarter.sqrt()) / a, (-b_half + delta_quarter.sqrt()) / a].iter() {
            let t = *t_root;
            if !ray_t.surrounds(t) {
                continue;
            }

            let hit_point = ray.at(t);
            let normal = (hit_point - self.center).normalized();

            return Some(HitRecord::new(hit_point, normal, t, ray));
        }

        None
    }
}