use std::collections::HashMap;

use uuid::Uuid;

use crate::{interval::Interval, materials::material::Material, ray::Ray, Point3, Vector3};

pub struct HitRecord {
    pub point: Point3,
    pub norm: Vector3,
    pub front_face: bool,
    pub t: f32,
    pub material_id: Uuid,
}
pub trait Raycaster {
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitRecord>;
}

pub struct Hittables<I: Sized> {
    pub objects: HashMap<I, ObjectContainer>,
}

impl<I: Sized> Hittables<I> {
    pub fn new() -> Self {
        Hittables {
            objects: HashMap::new(),
        }
    }
}
impl Hittables<Uuid> {
    pub fn insert(&mut self, container: ObjectContainer) {
        self.objects.insert(Uuid::new_v4(), container);
    }
}

impl<I: Sized> Raycaster for Hittables<I> {
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitRecord> {
        let hit_records = self
            .objects
            .values()
            .clone()
            .into_iter()
            .filter_map(|obj| obj.object.hit(ray, interval));
        hit_records.reduce(|accumulator, hit| {
            if hit.t < accumulator.t {
                hit
            } else {
                accumulator
            }
        })
    }
}

pub struct ObjectContainer {
    pub object: Box<dyn Raycaster>,
}

impl ObjectContainer {}

impl<T: Raycaster + 'static> From<T> for ObjectContainer {
    fn from(value: T) -> Self {
        ObjectContainer {
            object: Box::new(value),
        }
    }
}
