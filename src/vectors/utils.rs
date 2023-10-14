use crate::vectors::vector3::Vector3;

const EPSILON: f32 = 1.0e-8;

/// Return true if the vector is close to zero in all dimensions.
pub fn near_zero(vec: &Vector3<f32>) -> bool {
    (vec.x.abs() < EPSILON) && (vec.y.abs() < EPSILON) && (vec.z.abs() < EPSILON)
}
