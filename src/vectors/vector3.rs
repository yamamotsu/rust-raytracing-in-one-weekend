use num_traits::Float as Number;
use rand::random as rnd;

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

    pub fn sqrt(self) -> Vector3<T> {
        Vector3::<T> {
            x: self.x.sqrt(),
            y: self.y.sqrt(),
            z: self.z.sqrt(),
        }
    }

    pub fn to_unit(self) -> Vector3<T> {
        self / self.norm()
    }

    pub fn random() -> Vector3 {
        Vector3 {
            x: rnd(),
            y: rnd(),
            z: rnd(),
        }
    }
    pub fn random_range(min: f32, max: f32) -> Vector3 {
        Self::random() * (max - min) + min
    }
    pub fn random_unit_vector() -> Vector3 {
        Self::random_range(-1.0, 1.0).to_unit()
    }
}

impl<T: Number> Copy for Vector3<T> {}
impl<T: Number> Clone for Vector3<T> {
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
impl<T: Number> From<(T, T, T)> for Vector3<T> {
    fn from(value: (T, T, T)) -> Self {
        Vector3 {
            x: value.0,
            y: value.1,
            z: value.2,
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

        let expected = Vector3::from((0.0, -3.0 / 5.0, 4.0 / 5.0));
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
