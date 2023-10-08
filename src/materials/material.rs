use crate::{ray::Ray, color::Color, Raycaster, objects::hittable::HitRecord};

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord::<'_>) -> (Color, Ray);
}
