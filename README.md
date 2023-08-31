# Rust Ray Tracer [![Rust](https://github.com/EricFalkenberg/rust-ray-tracer/actions/workflows/rust.yml/badge.svg)](https://github.com/EricFalkenberg/rust-ray-tracer/actions/workflows/rust.yml) [![rust-clippy analyze](https://github.com/EricFalkenberg/rust-ray-tracer/actions/workflows/rust-clippy.yml/badge.svg)](https://github.com/EricFalkenberg/rust-ray-tracer/security/code-scanning?query=is%3Aopen+branch%3Amaster+tool%3Aclippy++)
A simple ray tracer written in rust

### Usage

```
Usage: ray-tracer [OPTIONS] [SCENE]

Arguments:
  [SCENE]
          [default: spheres]

          Possible values:
          - spheres: The final scene render of "Ray Tracing in One Weekend"
          - link:    A custom scene using an .obj model of Link from the game Ocarina of Time (Work in Progress)

Options:
  -o, --output <FILE>
          

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

### Spheres 

<p align="center">
  <img src="https://github.com/EricFalkenberg/rust-ray-tracer/blob/master/examples/spheres.jpg"/>
</p>

### Link (Work in Progress)

<p align="center">
  <img src="https://github.com/EricFalkenberg/rust-ray-tracer/blob/master/examples/link.jpg"/>
</p>

### Benchmark

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `Spheres` | 25.121 ± 0.798 | 23.810 | 26.836 | 1.00 |
| `Link` | 77.191 ± 0.221 | 76.884 | 77.568 | 3.07 ± 0.10 |

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
