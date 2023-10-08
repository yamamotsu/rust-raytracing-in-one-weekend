use log::debug;
use rand::random;

use crate::{
    color::{write_color, Color},
    coordinate::Coordinate,
    interval::Interval,
    objects::hittable::{Hittables, Raycaster},
    ray::Ray,
    vectors::{
        ops::MatrixCross,
        vector3::{Point3, Vector3},
    },
};

pub struct Camera {
    aspect_ratio: f32,
    image_width: u32,
    image_height: u32,
    samples_per_pixel: u32,
    max_depth: i32,
    vfov_deg: f32,
    coordinate: Coordinate,

    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vector3,
    pixel_delta_v: Vector3,

    defocus_angle: f32,
    defocus_disk_u: Vector3,
    defocus_disk_v: Vector3,
}

impl Camera {
    pub fn render(&self, world: &Hittables) {
        print!("P3\n{} {}\n255\n", self.image_width, self.image_height);
        for y in 0..self.image_height {
            debug!("\rScanlines remaining: {}   ", self.image_height - y);
            for x in 0..self.image_width {
                let mut color: Color = Color::from((0.0, 0.0, 0.0));
                for _ in 0..self.samples_per_pixel {
                    color += self.ray_color(&self.get_ray(x, y), world, self.max_depth);
                }
                color /= self.samples_per_pixel as f32;

                let mut string = &mut String::new();
                write_color(&mut string, &color);
                print!("{}", string);
            }
        }

        debug!("\rDone.                 \n");
    }

    /// Get a randomly-asmpled camera ray for the pixel at location (x, y),
    /// originating from the camera defocus disk.
    fn get_ray(&self, x: u32, y: u32) -> Ray {
        let pixel_center =
            self.pixel00_loc + (self.pixel_delta_u * x as f32) + (self.pixel_delta_v * y as f32);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = (pixel_sample - ray_origin).to_unit();
        Ray::from((ray_origin, ray_direction))
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = Vector3::<f32>::random_in_unit_disk();
        self.center + (self.defocus_disk_u * p.x) + (self.defocus_disk_v * p.y)
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

    fn ray_color_object(&self, direction: &Vector3) -> Color {
        Color::from((direction.x + 1.0, direction.y + 1.0, direction.z + 1.0)) * 0.5
    }

    fn ray_color(&self, ray: &Ray, world: &Hittables, depth: i32) -> Color {
        if depth <= 0 {
            return Color::from((0.0, 0.0, 0.0));
        }

        match world.hit(&ray, Interval::from((0.001, f32::INFINITY))) {
            Some(result) => match result.material.scatter(ray, &result) {
                Some(scattered) => {
                    self.ray_color(&scattered.ray, world, depth - 1) * scattered.attenuation
                }
                None => Color::from((0.0, 0.0, 0.0)),
            },
            None => self.ray_color_background(&ray),
        }
    }
}

pub struct CameraParams {
    pub aspect_ratio: f32,
    pub image_width: u32,
    pub samples_per_pixel: u32,
    pub max_depth: i32,
    pub vfov_deg: f32,
    pub center: Point3,
    pub lookat: Point3,
    pub up: Vector3,
    pub focus_dist: f32,
    pub defocus_angle: f32,
}

impl From<CameraParams> for Camera {
    fn from(value: CameraParams) -> Self {
        let aspect_ratio = value.aspect_ratio;
        let image_width = value.image_width;
        let samples_per_pixel = value.samples_per_pixel;
        let max_depth = value.max_depth;
        let vfov_deg = value.vfov_deg;
        let center = value.center;
        let lookat = value.lookat;
        let up = value.up;
        let focus_dist = value.focus_dist;
        let defocus_angle = value.defocus_angle;

        let _image_height = (image_width as f32 / aspect_ratio) as u32;
        let image_height = if _image_height >= 1 { _image_height } else { 1 };

        // Determine viewport dimensions.
        // let focal_length = (center - lookat).norm();
        let h = (vfov_deg.to_radians() / 2.0).tan();
        let viewport_height: f32 = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * (image_width as f32 / image_height as f32);

        let coord_w = (center - lookat).to_unit();
        let coord_u = up.cross(&coord_w).to_unit();
        let coord_v = coord_w.cross(&coord_u);
        let coordinate = Coordinate {
            u: coord_u,
            v: coord_v,
            w: coord_w,
        };

        let viewport_u = coord_u * viewport_width;
        let viewport_v = -coord_v * viewport_height;

        let pixel_delta_u = viewport_u / image_width as f32;
        let pixel_delta_v = viewport_v / image_height as f32;

        // Calc. the location of the upper left pixel.
        let viewport_upper_left =
            center - coord_w * focus_dist - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        // Calculate the camera defocus disk basis vectors
        let defocus_radius = focus_dist * (defocus_angle / 2.0).to_radians().tan();
        let defocus_disk_u = coord_u * defocus_radius;
        let defocus_disk_v = coord_v * defocus_radius;

        Camera {
            aspect_ratio,
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel,
            max_depth,
            vfov_deg,
            coordinate,
            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
        }
    }
}
