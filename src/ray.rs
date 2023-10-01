use crate::vector3::{Point3, Vector3};

pub struct Ray {
    pub origin: Point3,
    pub direction: Vector3,
}

impl Ray {
    pub fn new() -> Self {
        Ray {
            origin: Point3::from((0.0, 0.0, 0.0)),
            direction: Vector3::from((1.0, 0.0, 0.0)),
        }
    }

    pub fn at(self, t: f32) -> Vector3 {
        self.origin + self.direction * t
    }
}

impl From<(Point3, Vector3)> for Ray {
    fn from(value: (Point3, Vector3)) -> Self {
        Ray {
            origin: value.0,
            direction: value.1,
        }
    }
}
