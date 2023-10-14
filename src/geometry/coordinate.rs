use crate::vectors::vector3::Point3;

use super::axis::Axes3D;

pub struct CoordinateSystem {
    pub axes: Axes3D,
    pub origin: Point3,
}
impl Copy for CoordinateSystem {}
impl Clone for CoordinateSystem {
    fn clone(&self) -> Self {
        CoordinateSystem {
            axes: self.axes,
            origin: self.origin,
        }
    }
}
