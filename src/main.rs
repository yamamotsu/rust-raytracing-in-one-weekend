mod color;
mod geometry;
mod interval;
mod materials;
mod objects;
mod optical;
mod renderers;
mod vectors;
mod world;

use color::Color;
use env_logger;
use geometry::axis::Axes3D;
use geometry::coordinate::CoordinateSystem;
use materials::dielectric::DiElectric;
use materials::lambertian::Lambertian;
use materials::material::{MaterialContainer, Materials};
use materials::metal::Metal;
use objects::hittables::Hittables;
use objects::object::Object;
use objects::plane::Plane;
use objects::sphere::Sphere;
use rand::random;
use renderers::camera::{Camera, CameraGeometryParam, CameraOpticalParam, ImageSize};
use renderers::renderer::Renderer;
use std::io::Write;
use uuid::Uuid;
use vectors::vector3::{Point3, Vector3};
use world::World;

use once_cell::sync::Lazy;

const ASPECT_RATIO: f32 = 4.0 / 3.0;
const IMAGE_WIDTH: u32 = 4096;
const SAMPLES_PER_PIXEL: u32 = 512;
const MAX_DEPTH: i32 = 100;
const MAX_WORKERS: usize = 16;

static WORLD: Lazy<World> = Lazy::new(|| initialize_world());

fn initialize_world() -> World {
    let mut materials: Materials = Materials::new();
    let mut objects: Hittables<Uuid> = Hittables::new();

    let material_ground = MaterialContainer::from(Lambertian {
        albedo: Color::from((0.5, 0.5, 0.5)),
    });
    let material_center = MaterialContainer::from(DiElectric {
        index_of_refraction: 1.5, // vs air: glass = 1.3-1.7, diamond = 2.4
    });
    let material_left = MaterialContainer::from(Lambertian {
        albedo: Color::from((0.4, 0.2, 0.1)),
    });
    let material_right =
        MaterialContainer::from(Metal::from((Color::from((0.95, 0.9, 0.95)), 0.0)));

    let material_mirror = MaterialContainer::from(Metal {
        albedo: Color::from((0.95, 0.95, 0.95)),
        fuzzy: 0.0,
    });

    let ground = Sphere {
        r: 1000.0,
        center: Point3::from((0.0, -1000.0, 0.0)),
        material_id: material_ground.id,
    };
    let sphere_center = Sphere {
        r: 1.0,
        center: Point3::from((0.0, 1.0, 0.0)),
        material_id: material_center.id,
    };
    let sphere_center_inside = Sphere {
        r: -0.85,
        center: Point3::from((0.0, 1.0, 0.0)),
        material_id: material_center.id,
    };
    let sphere_left = Sphere {
        r: 1.0,
        center: Point3::from((-4.0, 1.0, 0.0)),
        material_id: material_left.id,
    };
    let sphere_right = Sphere {
        r: 1.0,
        center: Point3::from((4.0, 1.0, 0.0)),
        material_id: material_right.id,
    };

    let plane_mirror = Plane {
        coordinate: CoordinateSystem {
            axes: Axes3D {
                u: Axes3D::UNIVERSE.u,
                v: Axes3D::UNIVERSE.w,
                w: -Axes3D::UNIVERSE.v,
            },
            origin: Point3 {
                x: 0.0,
                y: 2.0,
                z: -3.0,
            },
        },
        material_id: material_mirror.id,
        width: 20.0,
        height: 4.0,
    };

    materials.insert(material_ground);
    materials.insert(material_center);
    materials.insert(material_left);
    materials.insert(material_right);
    materials.insert(material_mirror);

    objects.insert(Object::from(ground));
    objects.insert(Object::from(sphere_center));
    objects.insert(Object::from(sphere_center_inside));
    objects.insert(Object::from(sphere_left));
    objects.insert(Object::from(sphere_right));
    objects.insert(Object::from(plane_mirror));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand::random::<f32>();
            let center = Point3::from((
                a as f32 * 0.9 * rand::random::<f32>(),
                0.2,
                b as f32 + 0.9 * rand::random::<f32>(),
            ));

            if (center - Point3::from((4.0, 0.2, 0.0))).norm() > 0.9 {
                let material = if choose_mat < 0.8 {
                    // diffuse
                    MaterialContainer::from(Lambertian {
                        albedo: Color::random() * Color::random(),
                    })
                } else if choose_mat < 0.95 {
                    // metal
                    MaterialContainer::from(Metal {
                        albedo: Color::random_range(0.5, 1.0),
                        fuzzy: random::<f32>() * 0.5,
                    })
                } else {
                    MaterialContainer::from(DiElectric {
                        index_of_refraction: 1.5,
                    })
                };
                objects.insert(Object::from(Sphere {
                    r: 0.2,
                    center,
                    material_id: material.id,
                }));
                materials.insert(material);
            }
        }
    }

    World { objects, materials }
}

fn main() {
    // render
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .format(|buf, record| write!(buf, "{}", record.args()))
        .init();

    let camera_center = Point3::from((4.0, 4.0, 10.0));
    let camera_lookat = Point3::from((0.0, 0.0, 0.0));
    let camera_up = Vector3::from((0.0, 1.0, 0.0));
    let camera_fov = 45.0;
    let camera = Camera {
        image_size: ImageSize {
            aspect_ratio: ASPECT_RATIO,
            width: IMAGE_WIDTH,
        },
        geometry: CameraGeometryParam {
            center: camera_center,
            lookat: camera_lookat,
            up: camera_up,
        },
        optical_params: CameraOpticalParam {
            vfov_deg: camera_fov,
            focus_dist: 12.0,
            defocus_angle: 0.6,
        },
        samples_per_pixel: SAMPLES_PER_PIXEL,
        max_depth: MAX_DEPTH,
        max_workers: MAX_WORKERS,
    };

    let img = camera.render(&WORLD);
    img.save_with_format("test.png", image::ImageFormat::Png)
        .unwrap();
}
