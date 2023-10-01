mod vector3;
mod color;
mod ray;

use std::io::Write;

use color::Color;
use env_logger;
use log::debug;
use vector3::{Point3, Vector3};

use crate::{ray::Ray, color::write_color, vector3::MatrixDot};

const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMAGE_WIDTH: i32 = 256;
const _IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;
const IMAGE_HEIGHT: i32 = if _IMAGE_HEIGHT >= 1 { _IMAGE_HEIGHT } else { 1 };
const FOCAL_LENGTH: f32 = 1.0;
const VIEWPORT_HEIGHT: f32 = 2.0;
const VIEWPORT_WIDTH: f32 = VIEWPORT_HEIGHT * (IMAGE_WIDTH as f32 / IMAGE_HEIGHT as f32);

fn ray_color(r: &Ray) -> Color {
    let dir = r.direction.to_unit();
    let alpha = 0.5 * (dir.y + 1.0);
    Color::from((1.0, 1.0, 1.0)) * (1.0 - alpha) + Color::from((0.5, 0.7, 1.0)) * alpha
}

fn main() {
    // render
    env_logger::builder()
        .format(|buf, record| {
            write!(buf, "{}", record.args())
        })
        .init();

    let camera_center = Point3::from((0.0, 0.0, 0.0));
    let viewport_u = Vector3::from((VIEWPORT_WIDTH, 0.0, 0.0));
    let viewport_v = Vector3::from((0.0, -VIEWPORT_HEIGHT, 0.0));

    let sphere_center = Point3::from((0.0, 0.0, -1.0));
    let sphere_radius = 0.5_f32;

    let cam_to_sphere = camera_center - sphere_center;

    let pixel_delta_u = viewport_u / IMAGE_WIDTH as f32;
    let pixel_delta_v = viewport_v / IMAGE_HEIGHT as f32;

    // Calc. the location of the upper left pixel.
    let focal_vec = Vector3::from((0.0, 0.0, FOCAL_LENGTH));
    let viewport_upper_left = camera_center - focal_vec - viewport_u/2.0 - viewport_v/2.0;

    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    // Render
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
    for y in 0..IMAGE_HEIGHT {
        debug!("\rScanlines remaining: {}", IMAGE_HEIGHT - y);
        for x in 0..IMAGE_WIDTH {
            let pixel_center = pixel00_loc + (pixel_delta_u * x as f32) + (pixel_delta_v * y as f32);
            let ray_direction = (pixel_center - camera_center).to_unit();
            let a = ray_direction.norm_squared();
            let b = ray_direction.dot(&cam_to_sphere);
            let c = cam_to_sphere.norm_squared() - sphere_radius.powi(2);
            let sphere_condition = b.powi(2) - a*c;

            let ray = Ray::from((camera_center, ray_direction));
            let color = if sphere_condition < 0.0 { ray_color(&ray) } else { Color::from((1.0, 0.0, 0.0)) };

            let mut string = &mut String::new();
            write_color(&mut string, &color);
            print!("{}", string);
        }
    }

    debug!("\rDone.                 \n");
}
