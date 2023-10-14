use std::sync::mpsc;

use image::RgbImage;
use log::debug;
use rand::random;
use threadpool::ThreadPool;

use crate::{
    color::{get_rgb, Color},
    geometry::{
        axis::{Axes2D, Axes3D},
        coordinate::CoordinateSystem,
    },
    interval::Interval,
    objects::hittable::Hittable,
    optical::ray::Ray,
    vectors::{
        ops::MatrixCross,
        vector3::{Point3, Vector3},
    },
    world::World,
};

use super::renderer::Renderer;

fn ray_color_background(r: &Ray) -> Color {
    let dir = r.direction.to_unit();
    let alpha = 0.5 * (dir.y + 1.0);
    Color::from((1.0, 1.0, 1.0)) * (1.0 - alpha) + Color::from((0.5, 0.7, 1.0)) * alpha
}

fn ray_color(ray: &Ray, world: &World, depth: i32) -> Color {
    if depth <= 0 {
        return Color::from((0.0, 0.0, 0.0));
    }

    let World { objects, materials } = world;
    match objects.hit(&ray, Interval::from((0.001, f32::INFINITY))) {
        Some(result) => {
            let id = result.material_id.clone();
            let material = &materials.materials[&id].material;
            match material.scatter(ray, &result) {
                Some(scattered) => {
                    ray_color(&scattered.ray, world, depth - 1) * scattered.attenuation
                }
                None => Color::from((0.0, 0.0, 0.0)),
            }
        }
        None => ray_color_background(&ray),
    }
}

/// Returns a random point in the square surrounding a pixel at the origin.
fn pixel_sample_square(axes: Axes3D) -> Vector3 {
    let px = -0.5 + random::<f32>();
    let py = -0.5 + random::<f32>();
    (axes.u * px) + (axes.v * py)
}
fn defocus_disk_sample(center: Vector3, defocus_disk_axes: Axes2D) -> Point3 {
    let p = Vector3::<f32>::random_in_unit_disk();
    center + (defocus_disk_axes.u * p.x) + (defocus_disk_axes.v * p.y)
}

pub struct CameraGeometryParam {
    pub center: Point3,
    pub lookat: Point3,
    pub up: Vector3,
}

pub struct CameraOpticalParam {
    pub focus_dist: f32,
    pub vfov_deg: f32,
    pub defocus_angle: f32,
}

pub struct ImageSize {
    pub aspect_ratio: f32,
    pub width: u32,
}

pub struct Rect<T: num_traits::Num> {
    pub width: T,
    pub height: T,
}
impl<T: num_traits::Num + Copy> Copy for Rect<T> {}
impl<T: num_traits::Num + Copy> Clone for Rect<T> {
    fn clone(&self) -> Self {
        Rect::<T> {
            width: self.width,
            height: self.height,
        }
    }
}

pub struct RenderingParameters {
    camera_coord: CoordinateSystem,
    viewport_coord: CoordinateSystem,
    image_coord: CoordinateSystem,
    defocus_disk_axes: Axes2D,
    image_rect: Rect<u32>,
}
impl Copy for RenderingParameters {}
impl Clone for RenderingParameters {
    fn clone(&self) -> Self {
        RenderingParameters {
            camera_coord: self.camera_coord,
            viewport_coord: self.viewport_coord,
            image_coord: self.image_coord,
            defocus_disk_axes: self.defocus_disk_axes,
            image_rect: self.image_rect,
        }
    }
}

pub struct Camera {
    // geometric params
    pub geometry: CameraGeometryParam,

    // image params
    pub image_size: ImageSize,

    // optical params
    pub optical_params: CameraOpticalParam,

    // render params
    pub samples_per_pixel: u32,
    pub max_depth: i32,
    pub max_workers: usize,
}

impl Camera {
    fn initialize(&self) -> RenderingParameters {
        let CameraGeometryParam { center, lookat, up } = self.geometry;
        let ImageSize {
            aspect_ratio,
            width: image_width,
        } = self.image_size;

        let CameraOpticalParam {
            vfov_deg,
            focus_dist,
            defocus_angle,
        } = self.optical_params;

        let _image_height = (image_width as f32 / aspect_ratio) as u32;
        let image_height = if _image_height >= 1 { _image_height } else { 1 };

        // Determine viewport dimensions.
        let h = (vfov_deg.to_radians() / 2.0).tan();
        let viewport_height: f32 = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * (image_width as f32 / image_height as f32);

        let coord_w = (center - lookat).to_unit();
        let coord_u = up.cross(&coord_w).to_unit();
        let coord_v = coord_w.cross(&coord_u);
        let coordinate = CoordinateSystem {
            axes: Axes3D {
                u: coord_u,
                v: coord_v,
                w: coord_w,
            },
            origin: center,
        };

        let viewport_u = coord_u * viewport_width;
        let viewport_v = -coord_v * viewport_height;

        let pixel_delta_u = viewport_u / image_width as f32;
        let pixel_delta_v = viewport_v / image_height as f32;

        // Calc. the location of the upper left pixel.
        let viewport_upper_left =
            center - coord_w * focus_dist - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        let viewport_coord = CoordinateSystem {
            axes: Axes3D {
                u: viewport_u,
                v: viewport_v,
                w: Vector3::zero(),
            },
            origin: viewport_upper_left,
        };
        let image_coord = CoordinateSystem {
            axes: Axes3D {
                u: pixel_delta_u,
                v: pixel_delta_v,
                w: Vector3::zero(),
            },
            origin: pixel00_loc,
        };

        // Calc. the camera defocus disk basis vectors
        let defocus_radius = focus_dist * (defocus_angle / 2.0).to_radians().tan();
        let defocus_disk_u = coord_u * defocus_radius;
        let defocus_disk_v = coord_v * defocus_radius;
        let defocus_disk_axes = Axes2D {
            u: defocus_disk_u,
            v: defocus_disk_v,
        };

        RenderingParameters {
            camera_coord: coordinate,
            viewport_coord,
            image_coord,
            defocus_disk_axes,
            image_rect: Rect {
                width: image_width,
                height: image_height,
            },
        }
    }

    /// Get a randomly-asmpled camera ray for the pixel at location (x, y),
    /// originating from the camera defocus disk.
    fn get_ray(&self, x: u32, y: u32, rendering_params: RenderingParameters) -> Ray {
        let image_coord = rendering_params.image_coord;
        let pixel_center =
            image_coord.origin + (image_coord.axes.u * x as f32) + (image_coord.axes.v * y as f32);
        let pixel_sample = pixel_center + pixel_sample_square(image_coord.axes);

        let ray_origin = if self.optical_params.defocus_angle <= 0.0 {
            self.geometry.center
        } else {
            defocus_disk_sample(self.geometry.center, rendering_params.defocus_disk_axes)
        };
        let ray_direction = (pixel_sample - ray_origin).to_unit();
        Ray::from((ray_origin, ray_direction))
    }
}

impl Renderer for Camera {
    fn render(&self, world: &'static World) -> RgbImage {
        let render_params = self.initialize();
        let thread_pool = ThreadPool::new(self.max_workers);

        let Rect {
            width: image_width,
            height: image_height,
        } = render_params.image_rect;

        let mut image = RgbImage::new(image_width, image_height);

        for y in 0..image_height {
            debug!("\rScanlines remaining: {}   ", image_height - y);
            for x in 0..image_width {
                let max_depth = self.max_depth;
                let (tx, rx) = mpsc::channel::<Color>();
                for _ in 0..self.samples_per_pixel {
                    let tx = tx.clone();
                    let ray = self.get_ray(x, y, render_params);

                    thread_pool.execute(move || {
                        let color = ray_color(&ray, world, max_depth);
                        tx.send(color).unwrap();
                    });
                }
                thread_pool.join();
                let mut color: Color = Color::from((0.0, 0.0, 0.0));
                for col in rx.iter().take(self.samples_per_pixel as usize) {
                    color += col;
                }
                color /= self.samples_per_pixel as f32;

                image.put_pixel(x, y, get_rgb(&color));
            }
        }
        debug!("\rDone.                 \n");
        image
    }
}
