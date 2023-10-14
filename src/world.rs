use uuid::Uuid;

use crate::{materials::material::Materials, objects::hittables::Hittables};

pub struct World {
    pub objects: Hittables<Uuid>,
    pub materials: Materials,
}
