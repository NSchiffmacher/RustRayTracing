use crate::vector::{Point, Vec3};
use crate::hittable::{Hittable, Quad};
use crate::material::Material;

use std::rc::Rc;

type Cuboid = Vec<Box<dyn Hittable>>;

pub fn cuboid(center_pos: Point, u: Vec3, v: Vec3, w: Vec3, material: Rc<dyn Material>) -> Cuboid {
    let mut objects: Vec<Box<dyn Hittable>> = Vec::new();

    let hu = u / 2.;
    let hv = v / 2.;
    let hw = w / 2.;

    objects.push(Quad::new(center_pos - hu - hv - hw, u, w, material.clone())); // front
    objects.push(Quad::new(center_pos - hu + hv - hw, u, w, material.clone())); // back
    objects.push(Quad::new(center_pos - hu - hv - hw, w, v, material.clone())); // left
    objects.push(Quad::new(center_pos + hu - hv - hw, w, v, material.clone())); // right
    objects.push(Quad::new(center_pos - hu - hv + hw, u, v, material.clone())); // top
    objects.push(Quad::new(center_pos - hu - hv - hw, u, v, material.clone())); // bottom

    objects
}

pub fn axis_aligned_cuboid(center_pos: Point, size: Vec3, material: Rc<dyn Material>) -> Cuboid {
    let u = Vec3::new(size.x(), 0., 0.);
    let v = Vec3::new(0., size.y(), 0.);
    let w = Vec3::new(0., 0., size.z());

    cuboid(center_pos, u, v, w, material)
}

pub fn yaw_rotated_cuboid(center_pos: Point, size: Vec3, yaw_rotation_deg: f64, material: Rc<dyn Material>) -> Cuboid {
    let yaw_rotation = yaw_rotation_deg.to_radians();

    let u = Vec3::new(size.x() * yaw_rotation.cos(), 0., -size.x() * yaw_rotation.sin());
    let v = Vec3::new(0., size.y(), 0.);
    let w = Vec3::new(size.z() * yaw_rotation.sin(), 0., size.z() * yaw_rotation.cos());

    cuboid(center_pos, u, v, w, material)
}