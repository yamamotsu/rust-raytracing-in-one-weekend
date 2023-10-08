use crate::{color::Color, objects::hittable::{Raycaster, HitRecord}, ray::Ray, vectors::{vector3::Vector3, utils::near_zero}};

use super::material::Material;

/*
 * Lambertian scattering algorithm
 * https://raytracing.github.io/books/RayTracingInOneWeekend.html#diffusematerials/truelambertianreflection
 */
pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, hit_record: &HitRecord::<'_>) -> (Color, Ray) {
        let _scatter_direction = Vector3::<f32>::random_unit_vector() + hit_record.norm;
        let scatter_direction = if near_zero(&_scatter_direction) { hit_record.norm } else { _scatter_direction };
        let scattered = Ray { origin: hit_record.point, direction: scatter_direction };
        let attenuation = self.albedo;
        (attenuation, scattered)
    }
}
