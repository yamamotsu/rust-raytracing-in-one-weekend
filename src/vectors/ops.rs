use std::ops;
use num_traits::Float as Number;
use crate::vectors::vector3::*;

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
