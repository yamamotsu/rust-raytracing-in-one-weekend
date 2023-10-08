mod camera;
mod color;
mod coordinate;
mod interval;
mod materials;
mod objects;
mod ray;
mod vectors;

use color::Color;
use env_logger;
use materials::dielectric::DiElectric;
use materials::lambertian::Lambertian;
use materials::metal::Metal;
use objects::hittable::{Hittables, Raycaster};
use objects::sphere::Sphere;
use std::io::Write;
use vectors::vector3::{Point3, Vector3};

use crate::camera::{Camera, CameraParams};

const ASPECT_RATIO: f32 = 1.0; // 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 480;
const SAMPLES_PER_PIXEL: u32 = 150;
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
        albedo: Color::from((0.1, 0.2, 0.5)),
    };
    let material_left = DiElectric {
        index_of_refraction: 1.5, // vs air: glass = 1.3-1.7, diamond = 2.4
    };
    let material_right = Metal::from((Color::from((0.8, 0.6, 0.2)), 0.0));

    let ground = Sphere::from((100.0, Point3::from((0.0, -100.5, -1.0)), &material_ground));
    let sphere_center = Sphere::from((0.5, Point3::from((0.0, 0.0, -1.0)), &material_center));
    let sphere_left = Sphere::from((0.5, Point3::from((-1.0, 0.0, -1.0)), &material_left));
    let sphere_left_inside = Sphere::from((-0.4, Point3::from((-1.0, 0.0, -1.0)), &material_left));
    let sphere_right = Sphere::from((0.5, Point3::from((1.0, 0.0, -1.0)), &material_right));

    let objects: Vec<&dyn Raycaster> = vec![
        &ground,
        &sphere_center,
        &sphere_left,
        &sphere_left_inside,
        &sphere_right,
    ];
    let hittables = Hittables::from(objects);

    let camera_center = Point3::from((-2.0, 2.0, 1.0));
    let camera_lookat = Point3::from((0.0, 0.0, -1.0));
    let camera_up = Vector3::from((0.0, 1.0, 0.0));
    let camera_fov = 30.0;
    let camera = Camera::from(CameraParams {
        aspect_ratio: ASPECT_RATIO,
        image_width: IMAGE_WIDTH,
        samples_per_pixel: SAMPLES_PER_PIXEL,
        max_depth: MAX_DEPTH,
        vfov_deg: camera_fov,
        center: camera_center,
        lookat: camera_lookat,
        up: camera_up,
    });
    camera.render(&hittables);
}
