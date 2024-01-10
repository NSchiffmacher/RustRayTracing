use crate::vector::{Point, Vec3};
use crate::hittable::{HitRecord, Hittable, AABB};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::material::Material;
use crate::texture::Uv;

use std::rc::Rc;

pub struct Quad {
    q: Point,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    normal: Vec3,
    d: f64,
    material: Rc<dyn Material>,
    bbox: AABB,
}

impl Quad {
    pub fn new(q: Point, u: Vec3, v: Vec3, material: Rc<dyn Material>) -> Self {
        let bbox = AABB::from_points(q, q + u + v);

        let normal = u.cross(&v);
        let w = normal / (normal.dot(&normal));
        
        let normal = normal.normalized();
        let d = normal.dot(&q);

        Self { q, u, v, w, material, bbox, normal, d }
    }

    pub fn boxed(q: Point, u: Vec3, v: Vec3, material: Rc<dyn Material>) -> Box<dyn Hittable> {
        Box::new(Self::new(q, u, v, material))
    }
}

impl Hittable for Quad {
    fn bounding_box(&self) -> AABB {
        self.bbox
    }

    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let denom = self.normal.dot(&ray.direction());
        if denom == 0. {
            return None;
        }

        let t = (self.d - self.normal.dot(&ray.origin())) / denom;
        if !ray_t.contains(t) {
            return None;
        }
        
        // Determine if the intersection is in the plane
        let hit_point = ray.at(t);
        let p = hit_point - self.q;
        let alpha = self.w.dot(&p.cross(&self.v));
        let beta = self.w.dot(&self.u.cross(&p));

        if alpha < 0. || beta < 0. || alpha > 1. || beta > 1. {
            return None;
        }
        
        // Build the hit record
        let uv = Uv{ u: alpha, v: beta };  

        Some(HitRecord::new(hit_point, self.normal, t, uv, ray, self.material.clone()))
    }
} 