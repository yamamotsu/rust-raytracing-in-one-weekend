use uuid::Uuid;

use crate::{interval::Interval, optical::ray::Ray, Point3, Vector3};

pub struct HitRecord {
    pub point: Point3,
    pub norm: Vector3,
    pub front_face: bool,
    pub t: f32,
    pub material_id: Uuid,
}

pub trait Hittable: Sync {
    fn hit(&self, ray: &Ray, interval: Interval<f32>) -> Option<HitRecord>;
}
