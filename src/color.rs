use std::fmt;
use crate::{vector3::Vector3, interval::Interval};

pub type Color = Vector3;

pub fn write_color(output: &mut dyn std::fmt::Write, color: &Color) -> () {
    let intensity: Interval = Interval::from((0.0, 0.999));
    let rgb = Color::from((intensity.clamp(color.x), intensity.clamp(color.y), intensity.clamp(color.z))) * 256.0;
    fmt::write(output, format_args!("{:.0} {:.0} {:.0}\n", rgb.x, rgb.y, rgb.z)).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vector3::Vector3;

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
