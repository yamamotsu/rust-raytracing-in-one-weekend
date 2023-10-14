use std::collections::HashMap;

use crate::{color::Color, objects::hittable::HitRecord, ray::Ray};

use uuid::Uuid;

pub struct Scatter {
    pub attenuation: Color,
    pub ray: Ray,
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<Scatter>;
}

pub struct MaterialContainer {
    pub material: Box<dyn Material>,
    pub id: Uuid,
}

impl<T: Material + 'static> From<T> for MaterialContainer {
    fn from(value: T) -> Self {
        MaterialContainer {
            material: Box::new(value),
            id: Uuid::new_v4(),
        }
    }
}

pub struct Materials {
    pub materials: HashMap<Uuid, MaterialContainer>,
}

impl Materials {
    pub fn new() -> Self {
        Materials {
            materials: HashMap::<Uuid, MaterialContainer>::new(),
        }
    }
    pub fn insert(&mut self, material: MaterialContainer) {
        self.materials.insert(material.id, material);
    }
}

impl std::ops::Index<Uuid> for Materials {
    type Output = MaterialContainer;
    fn index(&self, index: Uuid) -> &Self::Output {
        &self.materials[&index]
    }
}
