use image::Rgb;
use log::{debug, warn};
use num_traits::ToPrimitive;

use crate::{interval::Interval, vectors::vector3::Vector3};
use std::fmt;

pub type Color = Vector3;

pub fn write_color(output: &mut dyn std::fmt::Write, color: &Color) -> () {
    let intensity: Interval<f32> = Interval::from((0.0, 0.999));
    let _color_gamma = color.sqrt();
    let rgb = Color::from((
        intensity.clamp(_color_gamma.x),
        intensity.clamp(_color_gamma.y),
        intensity.clamp(_color_gamma.z),
    )) * 256.0;

    fmt::write(
        output,
        format_args!("{:.0} {:.0} {:.0}\n", rgb.x, rgb.y, rgb.z),
    )
    .unwrap()
}

pub fn get_rgb(color: &Color) -> Rgb<u8> {
    let intensity = Interval::<f32> {
        min: 0.0,
        max: 255.0,
    };
    let _color = color.sqrt() * 255.999;
    let rgb = [
        (intensity.clamp(_color.x).to_u8().unwrap_or_else(|| {
            warn!("\ncolor R to_u8() failed! {}", _color.x);
            0
        })),
        (intensity.clamp(_color.y).to_u8().unwrap_or_else(|| {
            warn!("\ncolor G to_u8() failed! {}", _color.y);
            0
        })),
        (intensity.clamp(_color.z).to_u8().unwrap_or_else(|| {
            warn!("\ncolor B to_u8() failed! {}", _color.z);
            0
        })),
    ];
    Rgb(rgb)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vectors::vector3::Vector3;

    #[test]
    fn should_write_color() {
        let vec1 = Vector3 {
            x: 0.0,
            y: 0.5,
            z: 1.0,
        };
        let mut output = String::new();
        write_color(&mut output, &vec1);
        println!("{}", output);
        assert_eq!(output, String::from("0 127 255\n"));
    }
}
