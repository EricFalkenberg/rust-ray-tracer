mod image;
mod viewport;
mod ray;
mod util;
mod hittable;
mod camera;
mod material;
mod model;
mod scene;

use std::io::{Error};
use std::path::PathBuf;
use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(author, version, about)]
/// A ray tracer written in the Rust programming language
struct Cli {
    #[arg(value_enum, default_value_t = Scene::Spheres)]
    scene: Scene,
    #[arg(short, long, value_name="FILE")]
    output: Option<PathBuf>
}

#[derive(Clone, ValueEnum)]
pub enum Scene {
    /// The final scene render of "Ray Tracing in One Weekend"
    Spheres,
    /// A custom scene using an .obj model of Link from the game Ocarina of Time (Work in Progress)
    Link
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();
    scene::render(cli.scene, cli.output)
}
