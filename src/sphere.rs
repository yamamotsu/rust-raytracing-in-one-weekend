use std::vec::Vec;
use crate::{vector3::{Point3, MatrixDot}, ray::{ RayCollider, Ray, RayCastIntersection }};

pub struct Sphere {
    pub r: f32,
    pub center: Point3,
}
impl Sphere {
    pub fn new() -> Self {
        Sphere {
            r: 1.0,
            center: Point3::from((0.0, 0.0, 0.0)),
        }
    }
}

impl From<(f32, Point3)> for Sphere {
    fn from(value: (f32, Point3)) -> Self {
        Sphere { r: value.0, center: value.1 }
    }
}

impl RayCollider for Sphere {
    fn cast_ray(&self, ray: &Ray) -> Option<Vec<RayCastIntersection>> {
        let direction = &ray.direction;
        let origin_to_sphere = ray.origin - self.center;
        let a = direction.norm_squared();
        let b = direction.dot(&origin_to_sphere);
        let c = origin_to_sphere.norm_squared() - self.r.powi(2);
        let discriminant = b.powi(2) - a*c;

        let intersects_t = if discriminant < 0.0 {
            vec![]
        } else if discriminant == 0.0 {
            vec![-b/a]
        } else {
            let _itrc0 = (-b - discriminant.sqrt()) / a;
            let _itrc1 = (-b + discriminant.sqrt()) / a;
            vec![_itrc0, _itrc1]
        };

        let mut collisions: Vec<RayCastIntersection> = vec![];
        for t in intersects_t {
            let intersection = ray.at(t);
            let norm = (intersection - self.center).to_unit();
            let reflect = ray.direction.dot(&norm);
            let intersec = RayCastIntersection {
                norm: norm,
                point: intersection,
                reflect: reflect
            };
            collisions.push(intersec);
        };

        if collisions.len() == 0 {
            None
        } else {
            Some(collisions)
        }
    }
}
