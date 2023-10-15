use uuid::Uuid;

use crate::{
    geometry::coordinate::CoordinateSystem,
    interval::Interval,
    optical::ray::Ray,
    vectors::{
        ops::MatrixDot,
        utils::near_zero,
        vector3::{Point3, Vector3},
    },
};

use super::hittable::{HitRecord, Hittable};

pub struct Plane {
    pub coordinate: CoordinateSystem,
    pub material_id: Uuid,
    pub width: f32,
    pub height: f32,
}

impl Plane {
    fn get_plane00_loc(&self) -> Point3 {
        self.coordinate.origin
            - self.coordinate.axes.u * (self.width / 2.0)
            - self.coordinate.axes.w * (self.height / 2.0)
    }

    fn inside_area(&self, point: Point3) -> bool {
        let point_from_00 = point - self.get_plane00_loc();
        0.0 <= point_from_00.x
            && point_from_00.x <= self.width
            && 0.0 <= point_from_00.z
            && point_from_00.z <= self.height
    }
}

impl Hittable for Plane {
    fn hit(&self, ray: &Ray, interval: Interval<f32>) -> Option<HitRecord> {
        let normal = self.coordinate.axes.v;
        let ray_dot_face = ray.direction.dot(&normal);
        let t = (self.coordinate.origin - ray.origin).dot(&normal) / ray_dot_face;
        let point = ray.at(t);

        if !self.inside_area(point) || !interval.surrounds(t) {
            None
        } else {
            let front_face = ray_dot_face < 0.0;
            let norm = if front_face { normal } else { -normal };
            Some(HitRecord {
                point,
                norm,
                front_face,
                t,
                material_id: self.material_id,
            })
        }
    }
}

pub struct InfinitePlane {
    pub origin: Point3,
    pub normal: Vector3,
    pub material_id: Uuid,
}

impl Hittable for InfinitePlane {
    fn hit(&self, ray: &Ray, interval: Interval<f32>) -> Option<HitRecord> {
        let ray_dot_face = ray.direction.dot(&self.normal);
        let t = (self.origin - ray.origin).dot(&self.normal) / ray_dot_face;
        if !interval.surrounds(t) {
            return None;
        }

        let point = ray.direction * t + ray.origin;

        let front_face = ray_dot_face < 0.0;
        let norm = if front_face {
            self.normal
        } else {
            -self.normal
        };
        Some(HitRecord {
            point,
            norm,
            front_face,
            t,
            material_id: self.material_id,
        })
    }
}
