
#[derive(Debug, Clone, Copy)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self {
            min,
            max,
        }
    }

    pub fn surrounding(&self, other: &Self) -> Self {
        Self::new(self.min.min(other.min), self.max.max(other.max))
    }

    pub fn positive() -> Self {
        Self::new(0.00001, f64::INFINITY)
    }

    pub fn universe() -> Self {
        Self::new(-f64::INFINITY, f64::INFINITY)
    }

    pub fn contains(&self, value: f64) -> bool {
        self.min <= value && value <= self.max
    }

    pub fn surrounds(&self, value: f64) -> bool {
        self.min < value && value < self.max
    }

    pub fn expand(&self, value: f64) -> Self {
        Self::new(self.min - value, self.max + value)
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn clamp(&self, value: f64) -> f64 {
        if value < self.min {
            self.min
        } else if value > self.max {
            self.max
        } else {
            value
        }
    }
}