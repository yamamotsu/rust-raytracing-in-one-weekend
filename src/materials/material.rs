use crate::{color::Color, objects::hittable::HitRecord, ray::Ray};

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord<'_>) -> (Color, Ray);
}
