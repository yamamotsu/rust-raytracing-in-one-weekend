use super::hittable::Hittable;

pub struct ObjectContainer {
    pub object: Box<dyn Hittable>,
}

impl ObjectContainer {}

impl<T: Hittable + 'static> From<T> for ObjectContainer {
    fn from(value: T) -> Self {
        ObjectContainer {
            object: Box::new(value),
        }
    }
}
