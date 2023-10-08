mod camera;
mod color;
mod interval;
mod materials;
mod objects;
mod ray;
mod vectors;

use color::Color;
use env_logger;
use materials::lambertian::Lambertian;
use materials::metal::Metal;
use objects::hittable::{Hittables, Raycaster};
use objects::sphere::Sphere;
use std::io::Write;
use vectors::vector3::{Point3, Vector3};

use crate::camera::{Camera, CameraParams};

const ASPECT_RATIO: f32 = 1.0; // 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 512;
const FOCAL_LENGTH: f32 = 0.8;
const SAMPLES_PER_PIXEL: u32 = 100;
const MAX_DEPTH: i32 = 50;

fn main() {
    // render
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .format(|buf, record| write!(buf, "{}", record.args()))
        .init();

    let material_ground = Lambertian {
        albedo: Color::from((0.8, 0.8, 0.0)),
    };
    let material_center = Lambertian {
        albedo: Color::from((0.7, 0.3, 0.3)),
    };
    let material_left = Metal::from(Color::from((0.8, 0.8, 0.8)));
    let material_right = Metal::from(Color::from((0.8, 0.6, 0.2)));

    let ground = Sphere::from((100.0, Point3::from((0.0, -100.5, -1.0)), &material_ground));
    let sphere_center = Sphere::from((0.5, Point3::from((0.0, 0.0, -1.0)), &material_center));
    let sphere_left = Sphere::from((0.5, Point3::from((-1.0, 0.0, -1.0)), &material_left));
    let sphere_right = Sphere::from((0.5, Point3::from((1.0, 0.0, -1.0)), &material_right));

    let objects: Vec<&dyn Raycaster> = vec![&ground, &sphere_center, &sphere_left, &sphere_right];
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
