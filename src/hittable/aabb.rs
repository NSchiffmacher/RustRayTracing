use crate::interval::Interval;
use crate::vector::Point;
use crate::ray::Ray;

#[derive(Debug, Clone, Copy)]
pub struct AABB {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl AABB {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    pub fn empty() -> Self {
        Self {
            x: Interval::new(0., 0.), // ! MIGHT HAVE TO CHANGE THAT (+inf, -inf) ? 
            y: Interval::new(0., 0.), // ! MIGHT HAVE TO CHANGE THAT (+inf, -inf) ? 
            z: Interval::new(0., 0.), // ! MIGHT HAVE TO CHANGE THAT (+inf, -inf) ? 
        }
    }

    pub fn from_points(a: Point, b: Point) -> Self {
        let x = Interval::new(a.x().min(b.x()), a.x().max(b.x()));
        let y = Interval::new(a.y().min(b.y()), a.y().max(b.y()));
        let z = Interval::new(a.z().min(b.z()), a.z().max(b.z()));
        Self::new(x, y, z)
    }

    pub fn surrounding_box(&self, other: &Self) -> Self {
        let x = Interval::surrounding(&self.x, &other.x);
        let y = Interval::surrounding(&self.y, &other.y);
        let z = Interval::surrounding(&self.z, &other.z);
        Self::new(x, y, z)
    }

    pub fn axis(&self, axis: usize) -> &Interval {
        match axis {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Invalid axis"),
        }
    }

    pub fn hit(&self, ray: &Ray, ray_t: &Interval) -> bool {
        let mut tmin = ray_t.min;
        let mut tmax = ray_t.max;

        let tx1 = (self.x.min - ray.origin().x()) * ray.inv_direction().x();
        let tx2 = (self.x.max - ray.origin().x()) * ray.inv_direction().x();

        tmin = f64::min(f64::max(tx1, tmin), f64::max(tx2, tmin));
        tmax = f64::max(f64::min(tx1, tmax), f64::min(tx2, tmax));

        let ty1 = (self.y.min - ray.origin().y()) * ray.inv_direction().y();
        let ty2 = (self.y.max - ray.origin().y()) * ray.inv_direction().y();

        tmin = f64::min(f64::max(ty1, tmin), f64::max(ty2, tmin));
        tmax = f64::max(f64::min(ty1, tmax), f64::min(ty2, tmax));

        let tz1 = (self.z.min - ray.origin().z()) * ray.inv_direction().z();
        let tz2 = (self.z.max - ray.origin().z()) * ray.inv_direction().z();

        tmin = f64::min(f64::max(tz1, tmin), f64::max(tz2, tmin));
        tmax = f64::max(f64::min(tz1, tmax), f64::min(tz2, tmax));

        tmin <= tmax
    }
    
    pub fn x(&self) -> &Interval {
        &self.x
    }

    pub fn y(&self) -> &Interval {
        &self.y
    }

    pub fn z(&self) -> &Interval {
        &self.z
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::ray::Ray;
    use crate::vector::Vec3;

    #[test]
    fn test_hit() {
        let bbox = AABB::new(
            Interval::new(0., 1.),
            Interval::new(0., 1.),
            Interval::new(0., 1.),
        );
        let ray = Ray::new(Vec3::new(0.5, 0.5, -0.5), Vec3::new(0., 0., 1.), 0.);
        let ray_t = Interval::positive();
        assert!(bbox.hit(&ray, &ray_t));
    }
}