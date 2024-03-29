use crate::vector::{Point, Vec3};
use crate::hittable::{HitRecord, Hittable, AABB};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::material::Material;
use crate::texture::Uv;

use std::rc::Rc;

#[derive(Clone)]
pub struct Sphere {
    initial_center: Point,
    center_vec: Vec3,
    is_moving: bool,
    radius: f64,
    material: Rc<dyn Material>,
    bbox: AABB,
}

impl Sphere {
    pub fn new(center: Point, radius: f64, material: Rc<dyn Material>) -> Box<dyn Hittable> {
        let r = Vec3::new(radius, radius, radius);
        let bbox = AABB::from_points(center - r, center + r);
        Box::new(Self {
            initial_center: center,
            center_vec: Vec3::zero(),
            is_moving: false,
            radius,
            material,
            bbox,
        })
    }

    pub fn new_moving(initial_center: Point, final_center: Point, radius: f64, material: Rc<dyn Material>) -> Box<dyn Hittable> {
        let r = Vec3::new(radius, radius, radius);
        let box1 = AABB::from_points(initial_center - r, initial_center + r);
        let box2 = AABB::from_points(final_center - r, final_center + r);
        let bbox = AABB::surrounding_box(&box1, &box2);

        Box::new(Self {
            initial_center,
            center_vec: final_center - initial_center,
            is_moving: true,
            radius,
            material,
            bbox,
        })
    }

    pub fn center(&self, time: f64) -> Point {
        self.initial_center + self.center_vec * time
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    fn compute_uv(p: &Vec3) -> Uv {
        // p must be a unit vector

        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + std::f64::consts::PI;

        Uv {
            u: phi / (2. * std::f64::consts::PI),
            v: theta / std::f64::consts::PI,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let center = if self.is_moving {
            self.center(ray.time())
        } else {
            self.initial_center
        };

        let oc = *ray.origin() - center;
        let a = ray.direction().dot(&ray.direction());
        let b_half = ray.direction().dot(&oc);
        let c = oc.dot(&oc) - self.radius * self.radius;

        let delta_quarter = b_half * b_half -  a * c;
        if delta_quarter < 0. {
            return None;
        } 
        
        for t_root in [(-b_half - delta_quarter.sqrt()) / a, (-b_half + delta_quarter.sqrt()) / a].iter() {
            let t = *t_root;
            if !ray_t.surrounds(t) {
                continue;
            }

            let hit_point = ray.at(t);
            let normal = ((hit_point - center) / self.radius).normalized();
            let uv = Self::compute_uv(&normal);

            return Some(HitRecord::new(hit_point, normal, t, uv, ray, self.material.clone()));
        }

        None
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }

    fn box_clone(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }
}