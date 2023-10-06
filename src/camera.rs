use log::debug;
use rand::random;

use crate::{
    vector3::{Point3, Vector3},
    objects::hittable::{Hittables, Raycaster, HitRecord},
    ray::Ray, interval::Interval,
    color::{Color, write_color},
};

const VIEWPORT_HEIGHT: f32 = 2.0;

pub struct Camera {
    aspect_ratio: f32,
    image_width: u32,
    focal_length: f32,
    samples_per_pixel: u32,

    image_height: u32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vector3,
    pixel_delta_v: Vector3,
}

impl Camera {
    pub fn render(&self, world: &Hittables) {
        print!("P3\n{} {}\n255\n", self.image_width, self.image_height);
        for y in 0..self.image_height {
            debug!("\rScanlines remaining: {}   ", self.image_height - y);
            for x in 0..self.image_width {
                let mut color: Color = Color::from((0.0, 0.0, 0.0));
                for _ in 0..self.samples_per_pixel {
                    color += self.ray_color(&self.get_ray(x, y), world);
                }
                color /= self.samples_per_pixel as f32;

                let mut string = &mut String::new();
                write_color(&mut string, &color);
                print!("{}", string);
            }
        }

        debug!("\rDone.                 \n");
    }

    fn get_ray(&self, x: u32, y: u32) -> Ray {
        let pixel_center = self.pixel00_loc + (self.pixel_delta_u * x as f32) + (self.pixel_delta_v * y as f32);
        let pixel_sample = pixel_center + self.pixel_sample_square();
        let ray_direction = (pixel_sample - self.center).to_unit();
        Ray::from((self.center, ray_direction))
    }

    fn pixel_sample_square(&self) -> Vector3 {
        // Returns a random point in the square surrounding a pixel at the origin.
        let px = -0.5 + random::<f32>();
        let py = -0.5 + random::<f32>();
        (self.pixel_delta_u * px) + (self.pixel_delta_v * py)
    }

    fn ray_color_background(&self, r: &Ray) -> Color {
        let dir = r.direction.to_unit();
        let alpha = 0.5 * (dir.y + 1.0);
        Color::from((1.0, 1.0, 1.0)) * (1.0 - alpha) + Color::from((0.5, 0.7, 1.0)) * alpha
    }

    fn ray_color_object(&self, hit_record: &HitRecord) -> Color {
        let norm = hit_record.norm;
        Color::from((norm.x + 1.0, norm.y + 1.0, norm.z + 1.0)) * 0.5
    }

    fn ray_color(&self, ray: &Ray, world: &Hittables) -> Color {
        match world.hit(&ray, Interval::from((0.0, f32::INFINITY))) {
            Some(result) => self.ray_color_object(&result),
            None => self.ray_color_background(&ray)
        }
    }
}

pub struct CameraParams {
    pub aspect_ratio: f32,
    pub image_width: u32,
    pub focal_length: f32,
    pub samples_per_pixel: u32,
}

impl From<CameraParams> for Camera {
    fn from(value: CameraParams) -> Self {
        let aspect_ratio = value.aspect_ratio;
        let image_width = value.image_width;
        let focal_length = value.focal_length;
        let samples_per_pixel = value.samples_per_pixel;

        let _image_height = (image_width as f32 / aspect_ratio) as u32;
        let image_height = if _image_height >= 1 { _image_height } else { 1 };
        let center = Point3::from((0.0, 0.0, 0.0));

        let viewport_height: f32 = VIEWPORT_HEIGHT;
        let viewport_width = viewport_height * (image_width as f32 / image_height as f32);

        let viewport_u = Vector3::from((viewport_width, 0.0, 0.0));
        let viewport_v = Vector3::from((0.0, -viewport_height, 0.0));

        let pixel_delta_u = viewport_u / image_width as f32;
        let pixel_delta_v = viewport_v / image_height as f32;

        // Calc. the location of the upper left pixel.
        let focal_vec = Vector3::from((0.0, 0.0, focal_length));
        let viewport_upper_left = center - focal_vec - viewport_u/2.0 - viewport_v/2.0;
        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        Camera {
            aspect_ratio,
            image_width,
            image_height,
            focal_length,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel
        }
    }
}