use crate::vectors::vector3::{Point3, Vector3};

pub struct Axes3D {
    pub u: Vector3,
    pub v: Vector3,
    pub w: Vector3,
}
impl Copy for Axes3D {}
impl Clone for Axes3D {
    fn clone(&self) -> Self {
        Axes3D {
            u: self.u,
            v: self.v,
            w: self.w,
        }
    }
}

pub struct Axes2D {
    pub u: Vector3,
    pub v: Vector3,
}
impl Copy for Axes2D {}
impl Clone for Axes2D {
    fn clone(&self) -> Self {
        Axes2D {
            u: self.u,
            v: self.v,
        }
    }
}
