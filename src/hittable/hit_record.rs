use crate::vector::{Point, Vec3};
use crate::ray::Ray;
use crate::material::Material;
use crate::texture::Uv;

use std::rc::Rc;

pub struct HitRecord {
    pub point: Point,
    pub normal: Vec3,
    pub t: f64,
    pub uv: Uv,
    pub front_face: bool,
    pub material: Rc<dyn Material>,
}

impl HitRecord {
    pub fn new(point: Point, normal: Vec3, t: f64, uv: Uv, ray: &Ray, material: Rc<dyn Material>) -> Self {
        let normal = normal.normalized();
        Self {
            point,
            front_face: ray.direction().dot(&normal) < 0.,
            normal: if ray.direction().dot(&normal) < 0. { normal } else { -normal },
            t,
            uv,
            material,
        }
    }
}