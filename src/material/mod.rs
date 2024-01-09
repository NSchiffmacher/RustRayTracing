use crate::color::Color;
use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::vector::Vec3;
use crate::texture::Uv;

mod dielectric;
mod lambertian;
mod metal;
mod diffuse_light;

pub use dielectric::Dielectric;
pub use lambertian::Lambertian;
pub use metal::Metal;
pub use diffuse_light::DiffuseLight;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)>;

    fn emitted(&self, _uv: &Uv, _p: &Vec3) -> Color {
        Color::black()
    }
}
