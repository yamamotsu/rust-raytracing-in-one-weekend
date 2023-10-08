mod vectors;
mod color;
mod ray;
mod objects;
mod interval;
mod camera;
mod materials;

use std::io::Write;
use color::Color;
use env_logger;
use materials::material::Material;
use materials::lambertian::Lambertian;
use vectors::vector3::{Point3, Vector3};
use objects::sphere::Sphere;
use objects::hittable::{Hittables, Raycaster};

use crate::camera::{Camera, CameraParams};

const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 765;
const FOCAL_LENGTH: f32 = 1.5;
const SAMPLES_PER_PIXEL: u32 = 100;
const MAX_DEPTH: i32 = 50;

fn main() {
    // render
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .format(|buf, record| {
            write!(buf, "{}", record.args())
        })
        .init();

    let lambertian = Lambertian { albedo: Color::from((0.5, 0.5, 0.5)) };
    let sphere = Sphere::from((0.5, Point3::from((0.0, 0.0, -1.0)), &lambertian));
    let ground = Sphere::from((100.0, Point3::from((0.0, -100.5, -1.0)), &lambertian));
    let objects: Vec<&dyn Raycaster> = vec![&sphere, &ground];
    let hittables = Hittables::from(objects);

    let camera = Camera::from(CameraParams {
        aspect_ratio: ASPECT_RATIO,
        image_width: IMAGE_WIDTH,
        focal_length: FOCAL_LENGTH,
        samples_per_pixel: SAMPLES_PER_PIXEL,
        max_depth: MAX_DEPTH,
    });
    camera.render(&hittables);
}
