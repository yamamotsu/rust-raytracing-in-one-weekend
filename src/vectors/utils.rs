use crate::vectors::{ vector3::Vector3, ops::MatrixDot };

pub fn random_on_hemisphere(norm: &Vector3::<f32>) -> Vector3::<f32> {
    let vec = Vector3::<f32>::random_unit_vector();
    if vec.dot(norm) > 0.0 { vec } else { -vec }
}
