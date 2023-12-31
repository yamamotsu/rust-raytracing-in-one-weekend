use image::RgbImage;

use crate::world::World;

pub trait Renderer {
    fn render(&self, world: &'static World) -> RgbImage;
}
