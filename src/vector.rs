use std::ops;
use rand::Rng;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

pub type Point = Vec3;

impl Vec3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            x,
            y,
            z,
        }
    }

    pub fn axis(&self, axis: usize) -> f64 {
        match axis {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("Invalid axis"),
        }
    }

    pub fn set_axis(&mut self, axis: usize, value: f64) {
        match axis {
            0 => self.x = value,
            1 => self.y = value,
            2 => self.z = value,
            _ => panic!("Invalid axis"),
        }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub const fn zero() -> Self {
        Self { x: 0., y: 0., z: 0. }
    }

    pub fn length_squared(&self) -> f64 {
        self.dot(self)
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn normalized(&self) -> Self {
        let length = self.length();
        *self / length
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Self::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    pub fn random_in_sphere() -> Self {
        let mut rng = rand::thread_rng();
        loop {
            let vec = Self::new(
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
            );
            if vec.length_squared() < 1. {
                return vec;
            }
        }
    }

    pub fn random(min: f64, max: f64) -> Self {
        let mut rng = rand::thread_rng();
        Self::new(
            rng.gen_range(min..=max),
            rng.gen_range(min..=max),
            rng.gen_range(min..=max),
        )
    }

    pub fn random_in_disk() -> Self {
        let mut rng = rand::thread_rng();
        loop {
            let vec = Self::new(
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
                0.,
            );
            if vec.length_squared() < 1. {
                return vec;
            }
        }
    }

    pub fn random_unit_vector() -> Self {
        Self::random_in_sphere().normalized()
    }

    pub fn random_vector_in_hemisphere(normal: &Vec3) -> Vec3 {
        let mut random_vec = Vec3::random_unit_vector();
        if random_vec.dot(normal) < 0.0 {
            random_vec = -random_vec;
        }
        random_vec
    }

    pub fn reflect(&self, normal: &Vec3) -> Vec3 {
        *self - 2. * self.dot(normal) * (*normal)
    }

    pub fn refract(&self, normal: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = 1.0f64.min(-self.dot(normal));
        let r_out_perp = etai_over_etat * (*self + cos_theta * (*normal));
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * (*normal);
        r_out_perp + r_out_parallel
    }

    pub fn near_zero(&self) -> bool {
        let epsilon = 1e-8;
        self.x.abs() < epsilon && self.y.abs() < epsilon && self.z.abs() < epsilon
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(
            -self.x,
            -self.y,
            -self.z,
        )
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
        )
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
        )
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1. / rhs)
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_squared() {
        let vec = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(vec.length_squared(), 14.0);
    }

    #[test]
    fn test_length() {
        let vec = Vec3::new(1.0, 0.0, 0.0);
        assert_eq!(vec.length(), 1.);

        let vec = Vec3::new(1.0, 2.0, 2.0);
        assert_eq!(vec.length(), 3.);
    }

    #[test]
    fn test_add() {
        let mut vec1 = Vec3::new(1.0, 2.0, 3.0);
        let vec2 = Vec3::new(4.0, 5.0, 6.0);

        let result = vec1.clone() + vec2.clone();
        vec1 += vec2;
        assert_eq!(result.x, 5.0);
        assert_eq!(result.y, 7.0);
        assert_eq!(result.z, 9.0);
        assert_eq!(vec1.x, 5.0);
        assert_eq!(vec1.y, 7.0);
        assert_eq!(vec1.z, 9.0);
    }

    #[test]
    fn test_sub() {
        let mut vec1 = Vec3::new(4.0, 5.0, 6.0);
        let vec2 = Vec3::new(1.0, 2.0, 3.0);

        let result = vec1.clone() - vec2.clone();
        vec1 -= vec2;
        assert_eq!(result.x, 3.0);
        assert_eq!(result.y, 3.0);
        assert_eq!(result.z, 3.0);
        assert_eq!(vec1.x, 3.0);
        assert_eq!(vec1.y, 3.0);
        assert_eq!(vec1.z, 3.0);
    }

    #[test]
    fn test_mul() {
        let mut vec1 = Vec3::new(1.0, 2.0, 3.0);
        let scalar = 0.5;

        let result = vec1.clone() / scalar;
        vec1 /= scalar;
        assert_eq!(result.x, 2.0);
        assert_eq!(result.y, 4.0);
        assert_eq!(result.z, 6.0);
        assert_eq!(vec1.x, 2.0);
        assert_eq!(vec1.y, 4.0);
        assert_eq!(vec1.z, 6.0);
    }

    #[test]
    fn test_neg() {
        let vec = Vec3::new(1.0, 2.0, 3.0);
        let result = -vec;
        assert_eq!(result.x, -1.0);
        assert_eq!(result.y, -2.0);
        assert_eq!(result.z, -3.0);
    }

    #[test]
    fn test_dot() {
        let vec1 = Vec3::new(1.0, 2.0, 3.0);
        let vec2 = Vec3::new(4.0, 5.0, 6.0);
        let result = vec1.dot(&vec2);
        assert_eq!(result, 32.0);
    }

    #[test]
    fn test_cross() {
        let vec1 = Vec3::new(1.0, 0.0, 0.0);
        let vec2 = Vec3::new(0.0, 1.0, 0.0);
        let result = vec1.cross(&vec2);
        assert_eq!(result.x, 0.0);
        assert_eq!(result.y, 0.0);
        assert_eq!(result.z, 1.0);
    }
}
