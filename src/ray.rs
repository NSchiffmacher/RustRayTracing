

use crate::vector::{Vec3, Point};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Ray {
    origin: Point,
    direction: Vec3,
    inv_direction: Vec3,
    time: f64,
}

impl Ray {
    pub fn new(origin: Point, direction: Vec3, time: f64) -> Self {
        Self {
            origin,
            inv_direction: Vec3::new(1. / direction.x(), 1. / direction.y(), 1. / direction.z()),
            direction,
            time,
        }
    }

    pub fn origin(&self) -> &Point {
        &self.origin
    }

    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }

    pub fn inv_direction(&self) -> &Vec3 {
        &self.inv_direction
    }

    pub fn time(&self) -> f64 {
        self.time
    }

    pub fn at(&self, t: f64) -> Point {
        self.origin + self.direction * t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let origin = Point::new(1., 2., 3.);
        let direction = Vec3::new(4., 5., 6.);
        let ray = Ray::new(origin.clone(), direction.clone(), 0.);

        assert_eq!(*ray.origin(), origin);
        assert_eq!(*ray.direction(), direction);
    }

    #[test]
    fn test_at() {
        let origin = Point::new(1., 2., 3.);
        let direction = Vec3::new(4., 5., 6.);
        let ray = Ray::new(origin.clone(), direction.clone(), 0.);

        assert_eq!(ray.at(0.), origin);
        assert_eq!(ray.at(1.), origin + direction);
        assert_eq!(ray.at(2.), origin + direction * 2.);
    }
}