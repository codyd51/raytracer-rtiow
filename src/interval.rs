
#[derive(Debug, Copy, Clone)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max, }
    }

    pub fn empty() -> Self {
        // Default interval is empty
        Self::new(
            f64::MAX,
            f64::MIN,
        )
    }

    pub fn maximal() -> Self {
        Self::new(
            f64::MIN,
            f64::MAX,
        )
    }

    /// Inclusive contains
    pub fn contains(&self, val: f64) -> bool {
        self.min <= val && val <= self.max
    }

    /// Exclusive contains
    pub fn surrounds(&self, val: f64) -> bool {
        self.min < val && val < self.max
    }
}