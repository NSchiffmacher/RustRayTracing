use crate::interval::Interval;
use crate::ray::Ray;

mod sphere;
mod hittable_list;
mod hit_record;

pub use sphere::Sphere;
pub use hittable_list::HittableList;
pub use hit_record::HitRecord;

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord>;
}