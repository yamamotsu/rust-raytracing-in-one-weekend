use std::collections::HashMap;

use uuid::Uuid;

use crate::{interval::Interval, ray::Ray};

use super::{
    container::ObjectContainer,
    hittable::{HitRecord, Hittable},
};

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

impl<I: Sized> Hittable for Hittables<I> {
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
