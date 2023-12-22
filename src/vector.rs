use std::ops;

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            x,
            y,
            z,
        }
    }

    pub fn zero() -> Self {
        Self::new(0., 0., 0.)
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn normalize(&mut self) -> Self {
        let length = self.length();
        *self / length
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
}
