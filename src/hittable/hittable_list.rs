use crate::hittable::{Hittable, HitRecord, AABB, BvhTree};
use crate::ray::Ray;
use crate::interval::Interval;

use std::rc::Rc;
use std::cell::RefCell;
use std::ops::AddAssign;

pub struct HittableList {
    objects: Rc<RefCell<Vec<Box<dyn Hittable>>>>,
    bbox: AABB,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Rc::new(RefCell::new(Vec::new())),
            bbox: AABB::empty(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.bbox = self.bbox.surrounding_box(&object.bounding_box());
        self.objects.borrow_mut().push(object);
    }

    pub fn extends(&mut self, objects: Vec<Box<dyn Hittable>>) {
        for object in objects {
            self.add(object);
        }
    }

    pub fn objects(&self) -> Rc<RefCell<Vec<Box<dyn Hittable>>>> {
        self.objects.clone()
    }

    pub fn to_bvh(&self) -> Self {
        let tree = BvhTree::from_list(self);
        let bbox = tree.bounding_box();

        Self {
            objects: Rc::new(RefCell::new(vec![Box::new(tree)])),
            bbox,
        }
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let mut hit_record = None;
        let mut min_dist = ray_t.max;

        for object in self.objects.borrow().iter() {
            if let Some(hit_rec) = object.hit(ray, &Interval::new(ray_t.min, min_dist)) {
                min_dist = hit_rec.t;
                hit_record = Some(hit_rec);
            }
        }

        hit_record
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

impl AddAssign<Box<dyn Hittable>> for HittableList {
    fn add_assign(&mut self, object: Box<dyn Hittable>) {
        self.add(object);
    }
}

impl AddAssign<Vec<Box<dyn Hittable>>> for HittableList {
    fn add_assign(&mut self, objects: Vec<Box<dyn Hittable>>) {
        self.extends(objects);
    }
}