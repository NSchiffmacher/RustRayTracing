use crate::color::Color;
use crate::ray::Ray;
use crate::hittable::HitRecord;

mod lambertian;
pub use lambertian::Lambertian;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)>;
}
