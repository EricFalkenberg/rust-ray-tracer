use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Deref;
use cgmath::{Vector3, Vector2};
use image::{DynamicImage, GenericImageView, ImageResult, Pixel};
use crate::hittable::Hittable;
use crate::material::Material;

use Vector3 as Point3;
use Vector3 as Color3;
use Vector2 as Point2;

type TClosure = dyn Fn(Point3<f64>) -> Point3<f64>;
type Transform = Box<TClosure>;

pub struct Model {
    pub path: String,
    pub vertices: Vec<Point3<f64>>,
    pub faces: Vec<Hittable>,
    pub textures: HashMap<String, DynamicImage>
}
impl Model {
    pub fn new(
        path: String,
        filename: String,
        transforms: Vec<Transform>
    ) -> Self {
        let obj_file = format!("{0}/{1}.obj", path, filename);
        let mtl_file = format!("{0}/{1}.mtl", path, filename);
        let mtl = MaterialTemplateLibrary::read(mtl_file);
        let input = BufReader::new(File::open(obj_file).unwrap());
        let mut vertices: Vec<Point3<f64>> = vec![];
        let mut texture_coords: Vec<Point2<f64>> = vec![];
        let mut faces: Vec<Hittable> = vec![];
        let mut textures: HashMap<String, DynamicImage> = HashMap::new();
        let mut mat_name: String = String::new();
        for line in input.lines() {
            let s = line.unwrap_or(String::from(""));
            let tokens = s.split(' ').collect::<Vec<&str>>();
            let id = *tokens.first().unwrap_or(&"#");
            let points = &tokens[1..];
            match id {
                "v" => {
                    vertices.push(Model::parse_vertex(points, &transforms));
                }
                "f" => {
                    faces.push(Model::parse_face(points, mat_name.clone(), &vertices, &texture_coords));
                }
                "vt" => {
                    let x = points.first().unwrap_or(&"").parse::<f64>().unwrap_or(0.0);
                    let y = points.get(1).unwrap_or(&"").parse::<f64>().unwrap_or(0.0);
                    let texture_coord = Point2::new(x, y);
                    texture_coords.push(texture_coord);
                }
                "usemtl" => {
                    mat_name = String::from(*points.first().unwrap_or(&""));
                    let material_file_name = format!("{0}/{1}", path, mtl.materials.get(mat_name.as_str()).unwrap());
                    textures.insert(mat_name.clone(), image::open(material_file_name).unwrap());
                }
                _ => {}
            }
        }
        Self {
            path,
            vertices,
            faces,
            textures
        }
    }
    fn parse_vertex(
        points: &[&str],
        transforms: &Vec<Transform>
    ) -> Point3<f64> {
        let x = points.first()
            .unwrap_or(&"")
            .parse::<f64>()
            .unwrap_or(0.0) - 1.9;
        let y = points.get(1)
            .unwrap_or(&"")
            .parse::<f64>()
            .unwrap_or(0.0) - 1.0;
        let z = points.get(2)
            .unwrap_or(&"")
            .parse::<f64>()
            .unwrap_or(0.0);
        let mut point = Point3::new(x, y, z);
        for transform in transforms {
            point = transform.deref()(point);
        }
        point
    }
    fn parse_face(points: &[&str], material: String, vertices: &[Point3<f64>], texture_coords: &[Vector2<f64>]) -> Hittable {
        let (x_v, x_t, _) = Model::parse_face_index(points[0]);
        let (y_v, y_t, _) = Model::parse_face_index(points[1]);
        let (z_v, z_t, _) = Model::parse_face_index(points[2]);
        let at_coord = texture_coords[x_t];
        let bt_coord = texture_coords[y_t];
        let ct_coord = texture_coords[z_t];
        Hittable::Triangle {
            a: vertices[x_v],
            b: vertices[y_v],
            c: vertices[z_v],
            material: Material::Texture {
                key: material,
                a: at_coord,
                b: bt_coord,
                c: ct_coord
            }
        }
    }
    fn parse_face_index(s: &str) -> (usize, usize, usize) {
        let tokens = s.split('/').collect::<Vec<&str>>();
        let vertex_index = tokens[0].parse::<usize>().unwrap_or(1) - 1;
        let texture_index = tokens.get(1).unwrap_or(&"").parse::<usize>().unwrap_or(1) - 1;
        let normal_index = tokens.get(2).unwrap_or(&"").parse::<usize>().unwrap_or(1) - 1;
        (vertex_index, texture_index, normal_index)
    }
}
struct MaterialTemplateLibrary {
    pub materials: HashMap<String, String>
}
impl MaterialTemplateLibrary {
    fn read(mtl_file: String) -> Self {
        let input = BufReader::new(File::open(mtl_file).unwrap());
        let mut current_material = String::from("unnamed_mat");
        let mut materials: HashMap<String, String> = HashMap::new();
        for line in input.lines() {
            let (id, points) = tokenize_line(line.unwrap_or(String::from("#")));
            match id.as_str() {
                "newmtl" => {
                    let name = points.get(0);
                    if let Some(name_unwrapped) = name {
                        current_material = String::from(name_unwrapped);
                    }
                },
                "map_Kd" => {
                    let file_name = points.get(0);
                    if let Some(file_name_unwrapped) = file_name {
                        materials.insert(String::from(&current_material), String::from(file_name_unwrapped));
                    }
                }
                _ => {}
            }
        }
        Self {
            materials
        }
    }
}

fn tokenize_line(s: String) -> (String, Vec<String>) {
    let tokens = s.split(' ').collect::<Vec<&str>>();
    let id = String::from(*tokens.first().unwrap_or(&"#"));
    let mut data: Vec<String> = vec![];
    for point in &tokens[1..] {
        data.push(String::from(*point));
    }
    (id, data)
}
