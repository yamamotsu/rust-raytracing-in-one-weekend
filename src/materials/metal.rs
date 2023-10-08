use crate::{color::Color, ray::Ray, vectors::utils::reflect};

use super::material::{Material, Scatter};

pub struct Metal {
    pub albedo: Color,
}

impl From<Color> for Metal {
    fn from(albedo: Color) -> Self {
        Metal { albedo }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray: &crate::ray::Ray,
        hit_record: &crate::objects::hittable::HitRecord<'_>,
    ) -> Option<Scatter> {
        let reflected = reflect(&ray.direction.to_unit(), &hit_record.norm);
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
