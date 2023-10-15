use std::collections::HashMap;

use uuid::Uuid;

use crate::{interval::Interval, optical::ray::Ray};

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

impl<I: Sized + Sync> Hittable for Hittables<I> {
    fn hit(&self, ray: &Ray, interval: Interval<f32>) -> Option<HitRecord> {
        let mut current_interval = Interval {
            min: interval.min,
            max: interval.max,
        };
        let mut current_record: Option<HitRecord> = None;
        for (_, object) in &self.objects {
            match object.object.hit(ray, current_interval) {
                Some(record) => {
                    current_interval.max = record.t;
                    current_record = Some(record);
                }
                None => {}
            }
        }
        current_record
    }
}
