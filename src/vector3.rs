use num_traits::Float as Number;
use std::ops;

pub struct Vector3<T: Number = f32> {
    pub x: T,
    pub y: T,
    pub z: T,
}

pub type Point3 = Vector3;

impl<T: Number> Vector3<T> {
    pub fn norm(self) -> T {
        T::sqrt(self.norm_squared())
    }
    pub fn norm_squared(self) -> T {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn to_unit(self) -> Vector3::<T> {
        self / self.norm()
    }
}

impl<T: Number> Copy for Vector3::<T> {}
impl<T: Number> Clone for Vector3::<T> {
    fn clone(&self) -> Self {
        Vector3::<T>::from((self.x, self.y, self.z))
    }
    fn clone_from(&mut self, source: &Self) {
        self.x = source.x;
        self.y = source.y;
        self.z = source.z;
    }
}

// Factory
impl<T: Number> From<(T, T, T)> for Vector3::<T> {
    fn from(value: (T, T, T)) -> Self {
        Vector3 {
            x: value.0,
            y: value.1,
            z: value.2,
        }
    }
}

// Vector + Vector
impl<T: Number> ops::Add<Vector3<T>> for Vector3<T> {
    type Output = Vector3<T>;
    fn add(self, rhs: Vector3<T>) -> Vector3<T> {
        Vector3::<T> {
            x: (self.x + rhs.x),
            y: (self.y + rhs.y),
            z: (self.z + rhs.z),
        }
    }
}

// Vector + Number
impl<T: Number> ops::Add<T> for Vector3<T> {
    type Output = Vector3<T>;
    fn add(self, rhs: T) -> Vector3<T> {
        Vector3::<T> {
            x: (self.x + rhs),
            y: (self.y + rhs),
            z: (self.z + rhs),
        }
    }
}
impl<T: Number> ops::AddAssign<Vector3::<T>> for Vector3::<T> {
    fn add_assign(&mut self, rhs: Vector3::<T>) {
        let computed = *self + rhs;
        self.clone_from(&computed);
    }
}
impl<T: Number> ops::AddAssign<T> for Vector3::<T> {
    fn add_assign(&mut self, rhs: T) {
        let computed = *self + rhs;
        self.clone_from(&computed);
    }
}

// Vector - Vector
impl<T: Number> ops::Sub<Vector3<T>> for Vector3<T> {
    type Output = Vector3<T>;
    fn sub(self, rhs: Vector3<T>) -> Vector3<T> {
        Vector3::<T> {
            x: (self.x - rhs.x),
            y: (self.y - rhs.y),
            z: (self.z - rhs.z),
        }
    }
}
// Vector - Number
impl<T: Number> ops::Sub<T> for Vector3<T> {
    type Output = Vector3<T>;
    fn sub(self, rhs: T) -> Vector3<T> {
        Vector3::<T> {
            x: (self.x - rhs),
            y: (self.y - rhs),
            z: (self.z - rhs),
        }
    }
}
impl<T: Number> ops::SubAssign<Vector3::<T>> for Vector3::<T> {
    fn sub_assign(&mut self, rhs: Vector3::<T>) {
        let computed = *self - rhs;
        self.clone_from(&computed);
    }
}
impl<T: Number> ops::SubAssign<T> for Vector3::<T> {
    fn sub_assign(&mut self, rhs: T) {
        let computed = *self - rhs;
        self.clone_from(&computed);
    }
}

// Vector * Vector
impl<T: Number> ops::Mul<Vector3<T>> for Vector3<T> {
    type Output = Vector3<T>;
    fn mul(self, rhs: Vector3<T>) -> Vector3<T> {
        Vector3::<T> {
            x: (self.x * rhs.x),
            y: (self.y * rhs.y),
            z: (self.z * rhs.z),
        }
    }
}
// Vector * Number
impl<T: Number> ops::Mul<T> for Vector3<T> {
    type Output = Vector3<T>;
    fn mul(self, rhs: T) -> Vector3<T> {
        Vector3::<T> {
            x: (self.x * rhs),
            y: (self.y * rhs),
            z: (self.z * rhs),
        }
    }
}
impl<T: Number> ops::MulAssign<Vector3::<T>> for Vector3::<T> {
    fn mul_assign(&mut self, rhs: Vector3::<T>) {
        let computed = *self * rhs;
        self.clone_from(&computed);
    }
}
impl<T: Number> ops::MulAssign<T> for Vector3::<T> {
    fn mul_assign(&mut self, rhs: T) {
        let computed = *self * rhs;
        self.clone_from(&computed);
    }
}

// Vector / Vector
impl<T: Number> ops::Div<Vector3<T>> for Vector3<T> {
    type Output = Vector3<T>;
    fn div(self, rhs: Vector3<T>) -> Vector3<T> {
        Vector3::<T> {
            x: (self.x / rhs.x),
            y: (self.y / rhs.y),
            z: (self.z / rhs.z),
        }
    }
}
// Vector / Number
impl<T: Number> ops::Div<T> for Vector3<T> {
    type Output = Vector3<T>;
    fn div(self, rhs: T) -> Vector3<T> {
        Vector3::<T> {
            x: (self.x / rhs),
            y: (self.y / rhs),
            z: (self.z / rhs),
        }
    }
}
impl<T: Number> ops::DivAssign<Vector3::<T>> for Vector3::<T> {
    fn div_assign(&mut self, rhs: Vector3::<T>) {
        let computed = *self / rhs;
        self.clone_from(&computed);
    }
}
impl<T: Number> ops::DivAssign<T> for Vector3::<T> {
    fn div_assign(&mut self, rhs: T) {
        let computed = *self / rhs;
        self.clone_from(&computed);
    }
}

// [] operators
impl<T: Number> ops::Index<usize> for Vector3<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!(),
        }
    }
}
impl<T: Number> ops::IndexMut<usize> for Vector3<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!(),
        }
    }
}

// -Vector3
impl<T: Number> ops::Neg for Vector3<T> {
    type Output = Vector3<T>;
    fn neg(self) -> Self::Output {
        Vector3::<T> {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

// Dot operation
pub trait MatrixDot<Rhs, Output> {
    fn dot(&self, rhs: &Rhs) -> Output;
}
impl<T: Number> MatrixDot<Vector3::<T>, T> for Vector3::<T> {
    fn dot(&self, rhs: &Vector3::<T>) -> T {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

// Cross operation
pub trait MatrixCross<Rhs = Self, Output = Self> {
    fn cross(&self, rhs: &Rhs) -> Output;
}
impl<T: Number> MatrixCross for Vector3::<T> {
    fn cross(&self, rhs: &Vector3::<T>) -> Vector3::<T> {
        Vector3::<T> {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_calc_norm_correctly() {
        // simple
        let vec1 = Vector3 {
            x: 0.0,
            y: 3.0,
            z: 4.0,
        };
        assert_eq!(vec1.norm_squared(), 25.0);
        assert_eq!(vec1.norm(), 5.0);

        // negative property
        let vec2 = Vector3::from((5.0, -1.0, -2.0));
        assert_eq!(vec2.norm_squared(), 30.0);
        assert_eq!((vec2.norm() * 1000000.0).round(), 5477226.0);

        // zero
        let vec3 = Vector3::from((0.0, 0.0, 0.0));
        assert_eq!(vec3.norm(), 0.0);
        assert_eq!(vec3.norm_squared(), 0.0);
    }

    #[test]
    fn should_calc_unit_correctly() {
        // simple
        let vec1 = Vector3 {
            x: 0.0,
            y: -3.0,
            z: 4.0,
        };

        let expected = Vector3::from((0.0, -3.0/5.0, 4.0/5.0));
        let actual = vec1.to_unit();
        assert_eq!(actual.x, expected.x);
        assert_eq!(actual.y, expected.y);
        assert_eq!(actual.z, expected.z);
    }

    #[test]
    fn should_throw_error_when_to_unit_with_zero_vec() {
        let vec1 = Vector3::from((0.0, 0.0, 0.0));

        let actual = vec1.to_unit();
        assert!(actual.x.is_nan() && actual.y.is_nan() && actual.z.is_nan());
    }
}
