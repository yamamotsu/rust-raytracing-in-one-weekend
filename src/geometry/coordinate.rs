use crate::vectors::vector3::Point3;

use super::axis::Axes3D;

pub struct CoordinateSystem {
    pub axes: Axes3D,
    pub origin: Point3,
}

impl CoordinateSystem {
    pub const UNIVERSE: CoordinateSystem = CoordinateSystem {
        axes: Axes3D::UNIVERSE,
        origin: Point3::ZERO,
    };
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
