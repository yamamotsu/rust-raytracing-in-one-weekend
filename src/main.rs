mod vector3;
mod color;
mod ray;

use std::io::Write;

use env_logger;
use log::debug;

use crate::vector3::MatrixCross;

const IMAGE_WIDTH:i32 = 256;
const IMAGE_HEIGHT:i32 = 256;

fn main() {
    // render
    env_logger::builder()
        .format(|buf, record| {
            write!(buf, "{}", record.args())
        })
        .init();

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    let vec = vector3::Vector3 {
        x: 15.0,
        y: 10.0,
        z: 5.0,
    };
    let a = vec.cross(&vec);
    println!("vec+5 {} {} {}", a.x, a.y, a.z);

    // for y in 0..IMAGE_HEIGHT {
    //     log!(Level::Trace, "\rScanlines remaining: {}", IMAGE_HEIGHT - y);
    //     for x in 0..IMAGE_WIDTH {
    //         let r = x as f64 / ((IMAGE_WIDTH - 1) as f64);
    //         let g = (y as f64) / ((IMAGE_HEIGHT - 1) as f64);
    //         let b = 0f64;

    //         let ir = (r * 255.999) as i64;
    //         let ig = (g * 255.999) as i64;
    //         let ib = (b * 255.999) as i64;
    //         println!("{} {} {}", ir, ig, ib);
    //     }
    // }
    debug!("\rDone.                     \n");
}
