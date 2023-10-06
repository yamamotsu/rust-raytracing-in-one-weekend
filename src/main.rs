mod vector3;
mod color;
mod ray;
mod objects;
mod interval;
mod camera;

use std::io::Write;
use env_logger;
use vector3::{Point3, Vector3};
use objects::sphere::Sphere;
use objects::hittable::{Hittables, Raycaster};

use crate::camera::{Camera, CameraParams};

const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 256;
const FOCAL_LENGTH: f32 = 1.0;
const SAMPLES_PER_PIXEL: u32 = 100;

fn main() {
    // render
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .format(|buf, record| {
            write!(buf, "{}", record.args())
        })
        .init();

    let sphere = Sphere::from((0.5, Point3::from((0.0, 0.0, -1.0))));
    let ground = Sphere::from((100.0, Point3::from((0.0, -100.5, -1.0))));
    let objects: Vec<&dyn Raycaster> = vec![&sphere, &ground];
    let hittables = Hittables::from(objects);

    let camera = Camera::from(CameraParams {
        aspect_ratio: ASPECT_RATIO,
        image_width: IMAGE_WIDTH,
        focal_length: FOCAL_LENGTH,
        samples_per_pixel: SAMPLES_PER_PIXEL,
    });
    camera.render(&hittables);
}
