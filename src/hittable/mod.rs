use crate::interval::Interval;
use crate::vector::{Point, Vec3};
use crate::ray::Ray;

mod sphere;
mod hittable_list;

pub use sphere::Sphere;
pub use hittable_list::HittableList;

pub struct HitRecord {
    pub point: Point,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord>;
}

impl HitRecord {
    pub fn new(point: Point, normal: Vec3, t: f64, ray: &Ray) -> Self {
        Self {
            point,
            front_face: ray.direction().dot(&normal) < 0.,
            normal: if ray.direction().dot(&normal) < 0. { normal } else { -normal },
            t,
        }
    }
}