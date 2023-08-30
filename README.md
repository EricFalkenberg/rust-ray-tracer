# Rust Ray Tracer [![Rust](https://github.com/EricFalkenberg/rust-ray-tracer/actions/workflows/rust.yml/badge.svg)](https://github.com/EricFalkenberg/rust-ray-tracer/actions/workflows/rust.yml) [![rust-clippy analyze](https://github.com/EricFalkenberg/rust-ray-tracer/actions/workflows/rust-clippy.yml/badge.svg)](https://github.com/EricFalkenberg/rust-ray-tracer/security/code-scanning?query=is%3Aopen+branch%3Amaster+tool%3Aclippy++)
A simple ray tracer written in rust

### Output
<p align="center">
  <img src="https://github.com/EricFalkenberg/rust-ray-tracer/blob/master/examples/complex.jpg"/>
</p>

### Benchmark
| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `./target/release/ray-tracer` | 20.602 ± 0.571 | 19.814 | 21.586 | 1.00 |

### Citations
| Entry          | Value                                                                  |
|----------------|------------------------------------------------------------------------|
| Title (series) | Ray Tracing in One Weekend Series                                      |
| Title (book)   | Ray Tracing in One Weekend                                             |
| Author         | Peter Shirley, Trevor David Black, Steve Hollasch                      |
| Edition        | v4.0.0-alpha.1                                                         |
| Date           | 2023-08-06                                                             |
| URL (series)   | [link](https://raytracing.github.io/)                                  |
| URL (book)     | [link](https://raytracing.github.io/books/RayTracingInOneWeekend.html) |
