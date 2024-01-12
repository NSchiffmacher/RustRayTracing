use crate::interval::Interval;
use crate::ray::Ray;

mod aabb;
mod cuboid;
mod bvh;
mod constant_medium;
mod sphere;
mod quad;
mod hittable_list;
mod hit_record;

pub use aabb::AABB;
pub use cuboid::*;
pub use bvh::BvhTree;
pub use constant_medium::ConstantMedium;
pub use sphere::Sphere;
pub use quad::Quad;
pub use hittable_list::HittableList;
pub use hit_record::HitRecord;

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord>;
    fn bounding_box(&self) -> AABB;

    fn box_clone(&self) -> Box<dyn Hittable>;
}

impl Clone for Box<dyn Hittable> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}