use crate::color::Color;
use crate::ray::Ray;
use crate::hittable::HitRecord;

mod lambertian;
mod metal;

pub use lambertian::Lambertian;
pub use metal::Metal;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)>;
}
