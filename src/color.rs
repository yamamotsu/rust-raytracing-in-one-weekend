use image::Rgb;
use num_traits::ToPrimitive;

use crate::{interval::Interval, vectors::vector3::Vector3};
use std::fmt;

pub type Color = Vector3;

pub fn write_color(output: &mut dyn std::fmt::Write, color: &Color) -> () {
    let intensity: Interval = Interval::from((0.0, 0.999));
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
    let intensity: Interval = Interval::from((0.0, 0.999));
    let _color = color.sqrt();
    let rgb = [
        (intensity.clamp(_color.x) * 256.0).to_u8().unwrap(),
        (intensity.clamp(_color.y) * 256.0).to_u8().unwrap(),
        (intensity.clamp(_color.z) * 256.0).to_u8().unwrap(),
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
