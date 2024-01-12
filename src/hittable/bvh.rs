use crate::interval::Interval;
use crate::ray::Ray;
use crate::hittable::{Hittable, HittableList, HitRecord, AABB};

use rand::Rng;

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
pub enum BvhNode {
    Leaf(usize),
    Node(Box<BvhTree>, Box<BvhTree>),
}

#[derive(Clone)]
pub struct BvhTree {
    value: BvhNode,
    bbox: AABB,

    objects: Rc<RefCell<Vec<Box<dyn Hittable>>>>,
}

impl BvhTree {
    pub fn from_list(list: &HittableList) -> Self {
        let indices: Vec<usize> = (0..list.objects().borrow().len()).collect();
        Self::from_objects(list.objects().clone(), indices)
    }

    pub fn from_objects(objects: Rc<RefCell<Vec<Box<dyn Hittable>>>>, indices: Vec<usize>) -> Self {
        let mut rng = rand::thread_rng();
        let axis = rng.gen_range(0..3);
        
        match indices.len() {
            0 => panic!("Cannot create BVH tree from empty list"),
            1 => {
                let index = indices[0];
                let bbox = objects.borrow()[index].bounding_box();
                Self {
                    value: BvhNode::Leaf(index),
                    bbox,
                    objects,
                }
            },
            _ => {
                let mut indices = indices;
                indices.sort_by(|a, b| Self::box_compare(&objects.borrow()[*a], &objects.borrow()[*b], axis));

                let mid = indices.len() / 2;
                let left = Self::from_objects(objects.clone(), indices[..mid].to_vec());
                let right = Self::from_objects(objects.clone(), indices[mid..].to_vec());
                let bbox = left.bounding_box().surrounding_box(&right.bounding_box());
                Self {
                    value: BvhNode::Node(Box::new(left), Box::new(right)),
                    bbox,
                    objects,
                }
            },
        }
    }

    fn box_compare(box1: &Box<dyn Hittable>, box2: &Box<dyn Hittable>, axis: usize) -> std::cmp::Ordering {
        let box1 = box1.bounding_box();
        let box2 = box2.bounding_box();

        box1.axis(axis).min.partial_cmp(&box2.axis(axis).min).unwrap()
    }
}

impl Hittable for BvhTree {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        if !self.bbox.hit(ray, ray_t) {
            return None;
        }
        
        match &self.value {
            BvhNode::Leaf(index) => {
                self.objects.borrow()[*index].hit(ray, ray_t)
            },
            BvhNode::Node(left, right) => {
                if let Some(hit_left) = left.hit(ray, ray_t) {
                    right.hit(ray, &Interval::new(ray_t.min, hit_left.t)).or(Some(hit_left))
                } else {
                    right.hit(ray, ray_t)
                }
            }
        }
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }

    fn box_clone(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }
}