use crate::{ Point3, Vector3, Ray, interval::Interval };

pub struct HitRecord {
    pub point: Point3,
    pub norm: Vector3,
    pub front_face: bool,
    pub t: f32,
}
pub trait Raycaster {
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitRecord>;
}

pub struct Hittables<'t> {
    pub objects: Vec<&'t dyn Raycaster>
}

impl Raycaster for Hittables<'_> {
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitRecord> {
        let hit_records = self.objects.clone().into_iter().filter_map(|obj| obj.hit(ray, interval));
        hit_records.reduce(|accumulator, hit| if hit.t < accumulator.t { hit } else { accumulator })
    }
}

impl<'t> From<Vec<&'t dyn Raycaster>> for Hittables<'t> {
    fn from(value: Vec<&'t dyn Raycaster>) -> Self {
        Hittables { objects: value }
    }
}
