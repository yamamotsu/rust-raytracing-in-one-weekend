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
use rand::random;
use std::io::Write;
use vectors::vector3::{Point3, Vector3};

use crate::camera::{Camera, CameraParams};

const ASPECT_RATIO: f32 = 4.0 / 3.0; // 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 1200;
const SAMPLES_PER_PIXEL: u32 = 500;
const MAX_DEPTH: i32 = 50;

fn main() {
    // render
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .format(|buf, record| write!(buf, "{}", record.args()))
        .init();

    let material_ground = Lambertian {
        albedo: Color::from((0.5, 0.5, 0.5)),
    };
    let material_center = DiElectric {
        index_of_refraction: 1.5, // vs air: glass = 1.3-1.7, diamond = 2.4
    };
    let material_left = Lambertian {
        albedo: Color::from((0.4, 0.2, 0.1)),
    };
    let material_right = Metal::from((Color::from((0.7, 0.6, 0.5)), 0.0));

    let ground = Sphere::from((1000.0, Point3::from((0.0, -1000.0, 0.0)), &material_ground));
    let sphere_center = Sphere::from((1.0, Point3::from((0.0, 1.0, 0.0)), &material_center));
    let sphere_left = Sphere::from((1.0, Point3::from((-4.0, 1.0, 0.0)), &material_left));
    let sphere_left_inside = Sphere::from((-0.9, Point3::from((-4.0, 1.0, 0.0)), &material_left));
    let sphere_right = Sphere::from((1.0, Point3::from((4.0, 1.0, 0.0)), &material_right));

    let mut objects: Vec<&dyn Raycaster> = vec![
        &ground,
        &sphere_center,
        &sphere_left,
        &sphere_left_inside,
        &sphere_right,
    ];

    let mut diffuse_materials: Vec<(Lambertian, Point3)> = vec![];
    let mut metal_materials: Vec<(Metal, Point3)> = vec![];
    let mut grass_materials: Vec<(DiElectric, Point3)> = vec![];
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand::random::<f32>();
            let center = Point3::from((
                a as f32 * 0.9 * rand::random::<f32>(),
                0.2,
                b as f32 + 0.9 * rand::random::<f32>(),
            ));

            if (center - Point3::from((4.0, 0.2, 0.0))).norm() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let material: Lambertian = Lambertian {
                        albedo: Color::random() * Color::random(),
                    };
                    diffuse_materials.push((material, center));
                } else if choose_mat < 0.95 {
                    // metal
                    let material = Metal {
                        albedo: Color::random_range(0.5, 1.0),
                        fuzzy: random::<f32>() * 0.5,
                    };
                    metal_materials.push((material, center));
                } else {
                    let material = DiElectric {
                        index_of_refraction: 1.5,
                    };
                    grass_materials.push((material, center));
                };
            }
        }
    }

    let diffused_spheres: Vec<Sphere<'_, _>> = diffuse_materials
        .iter()
        .map(|(material, center)| Sphere {
            r: 0.2,
            center: *center,
            material,
        })
        .collect();
    let metal_spheres: Vec<Sphere<'_, _>> = metal_materials
        .iter()
        .map(|(material, center)| Sphere {
            r: 0.2,
            center: *center,
            material,
        })
        .collect();
    let grass_spheres: Vec<Sphere<'_, _>> = grass_materials
        .iter()
        .map(|(material, center)| Sphere {
            r: 0.2,
            center: *center,
            material,
        })
        .collect();

    for sphere_ref in diffused_spheres.iter() {
        objects.push(sphere_ref);
    }
    for sphere_ref in metal_spheres.iter() {
        objects.push(sphere_ref);
    }
    for sphere_ref in grass_spheres.iter() {
        objects.push(sphere_ref);
    }

    let hittables = Hittables::from(objects);

    let camera_center = Point3::from((13.0, 2.0, 3.0));
    let camera_lookat = Point3::from((0.0, 0.0, 0.0));
    let camera_up = Vector3::from((0.0, 1.0, 0.0));
    let camera_fov = 20.0;
    let camera = Camera::from(CameraParams {
        aspect_ratio: ASPECT_RATIO,
        image_width: IMAGE_WIDTH,
        samples_per_pixel: SAMPLES_PER_PIXEL,
        max_depth: MAX_DEPTH,
        vfov_deg: camera_fov,
        center: camera_center,
        lookat: camera_lookat,
        up: camera_up,
        focus_dist: 10.0,
        defocus_angle: 0.6,
    });
    camera.render(&hittables);
}
