# Ray Tracing in One Weekend in Rust

Personal project for my first Rust practice.
Simple ray-tracer script, used / migrated the algorithms / C++ codes written in [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html).

Final output

![Final output image](image.png)

## Run

```.sh
# install dependencies
cargo install

# run & write stdout into file as `ppm` formatted data
cargo run > output.ppm

# ..or you can use release(optimized) binary for fast execution
cargo build -r
./target/release/rust-tutorial > output.ppm
```

You can easily preview exported ppm file in some website like below:

https://www.cs.rhodes.edu/welshc/COMP141_F16/ppmReader.html
