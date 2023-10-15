use crate::vectors::vector3::{Point3, Vector3};

use super::hittable::Hittable;

pub struct ObjectContainer {
    pub object: Box<dyn Hittable + Send>,
}

impl ObjectContainer {}

impl<T: Hittable + 'static + Send> From<T> for ObjectContainer {
    fn from(value: T) -> Self {
        ObjectContainer {
            object: Box::new(value),
        }
    }
}
