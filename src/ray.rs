

use crate::vector::{Vec3, Point};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Ray {
    origin: Point,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point, direction: Vec3) -> Self {
        Self {
            origin,
            direction,
        }
    }

    pub fn origin(&self) -> &Point {
        &self.origin
    }

    pub fn direction(&self) -> &Point {
        &self.direction
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
        let ray = Ray::new(origin.clone(), direction.clone());

        assert_eq!(*ray.origin(), origin);
        assert_eq!(*ray.direction(), direction);
    }

    #[test]
    fn test_at() {
        let origin = Point::new(1., 2., 3.);
        let direction = Vec3::new(4., 5., 6.);
        let ray = Ray::new(origin.clone(), direction.clone());

        assert_eq!(ray.at(0.), origin);
        assert_eq!(ray.at(1.), origin + direction);
        assert_eq!(ray.at(2.), origin + direction * 2.);
    }
}