use crate::vectors::vector3::{Point3, Vector3};

use super::hittable::Hittable;

pub struct Triangle {
    pub points: [Point3; 3],
    pub normal: Vector3,
}

impl Triangle {
    pub fn gravity_center(&self) -> Point3 {
        (self.points[0] + self.points[1] + self.points[2]) / 3.0
    }
}

// impl Hittable for Triangle {
//     fn hit(
//         &self,
//         ray: &crate::optical::ray::Ray,
//         interval: crate::interval::Interval<f32>,
//     ) -> Option<super::hittable::HitRecord> {
//     }
// }
