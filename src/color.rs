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
            r: COLOR_RANGE.clamp(r), 
            g: COLOR_RANGE.clamp(g), 
            b: COLOR_RANGE.clamp(b)
        }
    }

    pub fn from_vec(v: &Vec3) -> Self {        
        Self::new(v.x(), v.y(), v.z())
    }

    pub fn to_vec(&self) -> Vec3 {
        Vec3::new(self.r, self.g, self.b)
    }

    pub fn to_ppm_string(&self) -> String {
        let ir = (self.r * 255.).floor() as u8;
        let ig = (self.g * 255.).floor() as u8;
        let ib = (self.b * 255.).floor() as u8;

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