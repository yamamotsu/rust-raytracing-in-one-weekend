use crate::{
    color::Color,
    objects::hittable::HitRecord,
    optical::ray::Ray,
    vectors::{utils::near_zero, vector3::Vector3},
};

use super::material::{Material, Scatter};

/// Lambertian scattering algorithm
/// https://raytracing.github.io/books/RayTracingInOneWeekend.html#diffusematerials/truelambertianreflection
pub struct Lambertian {
    pub albedo: Color,
}

impl From<Color> for Lambertian {
    fn from(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, hit_record: &HitRecord) -> Option<Scatter> {
        let _scatter_direction = Vector3::<f32>::random_unit_vector() + hit_record.norm;
        let scatter_direction = if near_zero(&_scatter_direction) {
            hit_record.norm
        } else {
            _scatter_direction
        };
        let scattered = Ray {
            origin: hit_record.point,
            direction: scatter_direction,
        };
        let attenuation = self.albedo;
        Some(Scatter {
            attenuation,
            ray: scattered,
        })
    }
}
