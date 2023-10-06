pub struct Interval {
    min: f32,
    max: f32,
}

impl Interval {
    pub fn contains(&self, x: f32) -> bool {
        self.min <= x && x <= self.max
    }
    pub fn surrounds(&self, x: f32) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, value: f32) -> f32 {
        if value < self.min {
            return self.min;
        }
        if value > self.max {
            return self.max;
        }
        value
    }

    pub fn empty() -> Self {
        Interval { min: f32::INFINITY, max: f32::NEG_INFINITY }
    }
    pub fn universe() -> Self {
        Interval { min: f32::NEG_INFINITY, max: f32::INFINITY }
    }
}

impl Copy for Interval{}
impl Clone for Interval {
    fn clone(&self) -> Self {
        Interval {
            min: self.min,
            max: self.max,
        }
    }
    fn clone_from(&mut self, source: &Self) {
        self.min = source.min;
        self.max = source.max;
    }
}

impl From<(f32, f32)> for Interval {
    fn from(value: (f32, f32)) -> Self {
        Interval { min: value.0, max: value.1 }
    }
}
