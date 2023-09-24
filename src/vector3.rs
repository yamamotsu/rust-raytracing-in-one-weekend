use std::{ops, fmt};
use num_traits::{Float as Number, PrimInt, Num};

pub struct Vector3<T: Number = f32>{
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Number> ops::Add<Vector3::<T>> for Vector3::<T>
{
    type Output = Vector3::<T>;
    fn add(self, rhs: Vector3::<T>) -> Vector3::<T> {
        Vector3::<T> {
            x: (self.x + rhs.x),
            y: (self.y + rhs.y),
            z: (self.z + rhs.z),
        }
    }
}

impl<T: Number> ops::Add<T> for Vector3::<T>
{
    type Output = Vector3::<T>;
    fn add(self, rhs: T) -> Vector3::<T> {
        Vector3::<T> {
            x: (self.x + rhs),
            y: (self.y + rhs),
            z: (self.z + rhs),
        }
    }
}

impl<T: Number> fmt::Display for Vector3::<T>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.x.to_f64(), self.y.to_string(), self.z.to_string())
    }
}
