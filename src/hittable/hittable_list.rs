use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;
use crate::interval::Interval;

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let mut hit_record = None;
        let mut min_dist = ray_t.max;

        for object in self.objects.iter() {
            if let Some(hit_rec) = object.hit(ray, &Interval::new(ray_t.min, min_dist)) {
                min_dist = hit_rec.t;
                hit_record = Some(hit_rec);
            }
        }

        hit_record
    }
}