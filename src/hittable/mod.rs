use crate::interval::Interval;
use crate::ray::Ray;

mod aabb;
mod bvh;
mod sphere;
mod quad;
mod hittable_list;
mod hit_record;

pub use aabb::AABB;
pub use bvh::BvhTree;
pub use sphere::Sphere;
pub use quad::Quad;
pub use hittable_list::HittableList;
pub use hit_record::HitRecord;

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord>;
    fn bounding_box(&self) -> AABB;
}