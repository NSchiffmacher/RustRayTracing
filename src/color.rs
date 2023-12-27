
#[derive(Clone)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b, }
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

    pub fn to_ppm_string(&self) -> String {
        let ir = (self.r * 255.).floor() as u8;
        let ig = (self.g * 255.).floor() as u8;
        let ib = (self.b * 255.).floor() as u8;

        format!("{} {} {}", ir, ig, ib)
    }
}