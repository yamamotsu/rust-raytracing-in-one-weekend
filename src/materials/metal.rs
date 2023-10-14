use crate::{
    color::Color,
    optical::{ray::Ray, scatter::reflect},
    vectors::vector3::Vector3,
};

use super::material::{Material, Scatter};

pub struct Metal {
    pub albedo: Color,
    pub fuzzy: f32,
}

impl From<(Color, f32)> for Metal {
    fn from(value: (Color, f32)) -> Self {
        Metal {
            albedo: value.0,
            fuzzy: value.1,
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray: &Ray,
        hit_record: &crate::objects::hittable::HitRecord,
    ) -> Option<Scatter> {
        let reflected = reflect(&ray.direction.to_unit(), &hit_record.norm)
            + Vector3::<f32>::random_unit_vector() * self.fuzzy;
        let attenuation = self.albedo;
        let scattered = Ray {
            origin: hit_record.point,
            direction: reflected,
        };
        Some(Scatter {
            attenuation,
            ray: scattered,
        })
    }
}
