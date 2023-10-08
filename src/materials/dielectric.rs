use crate::{
    color::Color,
    ray::Ray,
    vectors::utils::{reflect, reflectance, refract, refract_or_reflect, refractable},
};

use super::material::{Material, Scatter};

pub struct DiElectric {
    pub index_of_refraction: f32,
}

impl Material for DiElectric {
    fn scatter(
        &self,
        ray: &crate::ray::Ray,
        hit_record: &crate::objects::hittable::HitRecord<'_>,
    ) -> Option<super::material::Scatter> {
        let refraction_ratio = if hit_record.front_face {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };

        let unit_direction = ray.direction.to_unit();
        let scattered_direction =
            refract_or_reflect(&unit_direction, &hit_record.norm, refraction_ratio);

        let scattered = Ray {
            origin: hit_record.point,
            direction: scattered_direction,
        };
        let attenuation = Color::from((1.0, 1.0, 1.0));
        Some(Scatter {
            attenuation,
            ray: scattered,
        })
    }
}