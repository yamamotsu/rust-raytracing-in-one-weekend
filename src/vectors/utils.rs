use crate::vectors::{ops::MatrixDot, vector3::Vector3};

const EPSILON: f32 = 1.0e-8;

/// Return random unit vector on the hemisphere as normal
pub fn random_on_hemisphere(norm: &Vector3<f32>) -> Vector3<f32> {
    let vec = Vector3::<f32>::random_unit_vector();
    if vec.dot(norm) > 0.0 {
        vec
    } else {
        -vec
    }
}

/// Return true if the vector is close to zero in all dimensions.
pub fn near_zero(vec: &Vector3<f32>) -> bool {
    (vec.x.abs() < EPSILON) && (vec.y.abs() < EPSILON) && (vec.z.abs() < EPSILON)
}

/// Check if refract is able to occur with granted direction of light, normal of surface, and refraction ratio
pub fn refractable(cos_theta_in: f32, refraction_ratio: f32) -> bool {
    // No solution for Snell's law.
    let sin_theta = (1.0 - cos_theta_in.powi(2)).sqrt();
    refraction_ratio * sin_theta <= 1.0
}

/// Return new vector reflected by the surface with granted normal vector
pub fn reflect(vec: &Vector3<f32>, norm: &Vector3<f32>) -> Vector3<f32> {
    let _v = *norm * vec.dot(norm) * 2.0;
    *vec - _v
}

/// Return refracted direction.
pub fn refract(vec: &Vector3<f32>, norm: &Vector3<f32>, eta_i_over_t: f32) -> Option<Vector3<f32>> {
    let cos_theta = -vec.dot(norm) / (vec.norm() * norm.norm());

    // No solution for Snell's law. (Means not refractable)
    let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();
    if eta_i_over_t * sin_theta > 1.0 {
        return None;
    }

    let refract_perp = (*vec + *norm * cos_theta) * eta_i_over_t;
    let refract_parallel = -*norm * (1.0 - refract_perp.norm_squared()).sqrt();
    Some(refract_perp + refract_parallel)
}

/// calculate reflectance using Schlick's approximation
pub fn reflectance(cosine_theta_in: f32, reflection_ratio: f32) -> f32 {
    let r0 = ((1.0 - reflection_ratio) / (1.0 + reflection_ratio)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine_theta_in).powi(5)
}

pub fn refract_or_reflect(
    vec: &Vector3<f32>,
    norm: &Vector3<f32>,
    refraction_ratio: f32,
) -> Vector3<f32> {
    let cos_theta = -vec.dot(norm) / (vec.norm() * norm.norm());
    let reflectance = reflectance(cos_theta, refraction_ratio);
    if !refractable(cos_theta, refraction_ratio) || reflectance > rand::random() {
        reflect(vec, norm)
    } else {
        refract(vec, norm, refraction_ratio).unwrap()
    }
}
