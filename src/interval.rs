use num_traits::{Float, Num, PrimInt};

trait Cmp: PartialOrd + Copy {}

pub struct Interval<T: PartialOrd + Copy> {
    pub min: T,
    pub max: T,
}

impl<T: PartialOrd + Copy> Interval<T> {
    pub fn contains(&self, x: T) -> bool {
        self.min <= x && x <= self.max
    }
    pub fn surrounds(&self, x: T) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, value: T) -> T {
        if value < self.min {
            return self.min;
        }
        if value > self.max {
            return self.max;
        }
        value
    }
}

impl<T: Float + PartialOrd> Interval<T> {
    pub fn empty() -> Self {
        Interval {
            min: T::infinity(),
            max: T::neg_infinity(),
        }
    }
    pub fn universe() -> Self {
        Interval {
            min: T::neg_infinity(),
            max: T::infinity(),
        }
    }
}

impl<T: PartialOrd + Copy> Copy for Interval<T> {}
impl<T: PartialOrd + Copy> Clone for Interval<T> {
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

impl<T: PartialOrd + Copy> From<(T, T)> for Interval<T> {
    fn from(value: (T, T)) -> Self {
        Interval {
            min: value.0,
            max: value.1,
        }
    }
}
