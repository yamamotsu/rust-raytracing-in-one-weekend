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

/// Return new vector reflected by the surface with grant normal vector
pub fn reflect(vec: &Vector3<f32>, norm: &Vector3<f32>) -> Vector3<f32> {
    let _v = *norm * vec.dot(norm) * 2.0;
    *vec - _v
}
