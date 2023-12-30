use crate::vector::Vec3;
use crate::interval::Interval;

const COLOR_RANGE: Interval = Interval { min: 0., max: 1. };

#[derive(Clone)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { 
            r,
            g,
            b,
        }
    }

    pub fn from_vec(v: Vec3) -> Self {        
        Self::new(v.x(), v.y(), v.z())
    }

    pub fn to_vec(&self) -> Vec3 {
        Vec3::new(self.r, self.g, self.b)
    }

    pub fn to_ppm_string(&self) -> String {
        let r = COLOR_RANGE.clamp(self.r);
        let g = COLOR_RANGE.clamp(self.g);
        let b = COLOR_RANGE.clamp(self.b);

        let ir = (r * 255.).floor() as u8;
        let ig = (g * 255.).floor() as u8;
        let ib = (b * 255.).floor() as u8;

        format!("{} {} {}", ir, ig, ib)
    }

    pub fn lerp(&self, rhs: &Self, t: f64) -> Self {
        let r = (1. - t) * self.r + t * rhs.r;
        let g = (1. - t) * self.g + t * rhs.g;
        let b = (1. - t) * self.b + t * rhs.b;

        Self::new(r, g, b)
    }

    pub fn white() -> Self {
        Self::new(1., 1., 1.)
    }

    pub fn black() -> Self {
        Self::new(0., 0., 0.)
    }

    pub fn red() -> Self {
        Self::new(1., 0., 0.)
    }

    pub fn green() -> Self {
        Self::new(0., 1., 0.)
    }

    pub fn blue() -> Self {
        Self::new(0., 0., 1.)
    }
}

impl std::ops::AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b);
    }
}

impl std::ops::Add for Color {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl std::iter::Sum for Color {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Color::black(), |a, b| a + b)
    }
}

impl std::ops::Mul for Color {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
    }
}

impl std::ops::MulAssign for Color {
    fn mul_assign(&mut self, rhs: Self) {
        *self = Self::new(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b);
    }

}

impl std::ops::Mul<f64> for Color {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.r * rhs, self.g * rhs, self.b * rhs)
    }
}

impl std::ops::MulAssign<f64> for Color {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Self::new(self.r * rhs, self.g * rhs, self.b * rhs);
    }
}