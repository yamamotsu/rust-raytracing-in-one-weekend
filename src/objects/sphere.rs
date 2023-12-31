use uuid::Uuid;

use crate::objects::hittable::{HitRecord, Hittable};
use crate::optical::ray::Ray;
use crate::vectors::ops::MatrixDot;
use crate::{interval::Interval, vectors::vector3::Point3};

pub struct Sphere {
    pub r: f32,
    pub center: Point3,
    pub material_id: Uuid,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, interval: Interval<f32>) -> Option<HitRecord> {
        let direction = &ray.direction;
        let origin_to_sphere = ray.origin - self.center;
        let a = direction.norm_squared();
        let b = direction.dot(&origin_to_sphere);
        let c = origin_to_sphere.norm_squared() - self.r.powi(2);
        let discriminant = b.powi(2) - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let intersec_t0 = (-b - discriminant.sqrt()) / a;
        let intersec_t1 = (-b + discriminant.sqrt()) / a;
        let root = if interval.surrounds(intersec_t0) {
            intersec_t0
        } else {
            if interval.surrounds(intersec_t1) {
                intersec_t1
            } else {
                return None;
            }
        };

        let point = ray.at(root);
        let mut norm = (point - self.center) / self.r;
        let front_face = direction.dot(&norm) < 0.0;
        if front_face == false {
            // ensure that norm is always against ray
            norm *= -1.0;
        }
        Some(HitRecord {
            point,
            norm,
            t: root,
            front_face,
            material_id: self.material_id,
        })
    }
}
