use super::hittable::Hittable;

pub struct Object {
    pub mesh: Box<dyn Hittable + Send>,
}

impl Object {}

impl<T: Hittable + Send + 'static> From<T> for Object {
    fn from(value: T) -> Self {
        Object {
            mesh: Box::new(value),
        }
    }
}
