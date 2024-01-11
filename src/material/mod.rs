use crate::color::Color;
use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::vector::Vec3;
use crate::texture::Uv;

mod diffuse_light;
mod dielectric;
mod isotropic;
mod lambertian;
mod metal;

pub use diffuse_light::DiffuseLight;
pub use dielectric::Dielectric;
pub use isotropic::Isotropic;
pub use lambertian::Lambertian;
pub use metal::Metal;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)>;

    fn emitted(&self, _uv: &Uv, _p: &Vec3) -> Color {
        Color::black()
    }
}
