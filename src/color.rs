use std::{fmt};
use crate::vector3::Vector3;

pub type Color = Vector3;

pub fn write_color(output: &mut dyn std::fmt::Write, color: &Color) -> () {
    let rgb = Color::from((color.x * 255.999, color.y * 255.999, color.z * 255.999));
    fmt::write(output, format_args!("{:.0} {:.0} {:.0}\n", rgb.x.floor(), rgb.y.floor(), rgb.z.floor())).unwrap()
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
