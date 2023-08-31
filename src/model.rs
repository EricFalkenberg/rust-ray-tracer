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

pub struct Model {
    pub path: String,
    pub vertices: Vec<Point3<f64>>,
    pub faces: Vec<Hittable>,
}
impl Model {
    pub fn new(
        path: String,
        filename: String,
        transforms: Vec<Box<dyn Fn(Point3<f64>) -> Point3<f64>>>
    ) -> Self {
        let obj_file = format!("{0}/{1}.obj", path, filename);
        let mtl_file = format!("{0}/{1}.mtl", path, filename);
        let mtl = MaterialTemplateLibrary::read(mtl_file);
        let mut img: ImageResult<DynamicImage> = ImageResult::from(Ok(DynamicImage::new_rgb8(0, 0)));
        let input = BufReader::new(File::open(obj_file).unwrap());
        let mut vertices: Vec<Point3<f64>> = vec![];
        let mut texture_coords: Vec<Point2<f64>> = vec![];
        let mut faces: Vec<Hittable> = vec![];
        for line in input.lines() {
            let s = line.unwrap_or(String::from(""));
            let tokens = s.split(" ").collect::<Vec<&str>>();
            let id = *tokens.first().unwrap_or(&"#");
            let points = &tokens[1..];
            match id {
                "v" => {
                    vertices.push(Model::parse_vertex(points, &transforms));
                }
                "f" => {
                    faces.push(Model::parse_face(points, &img, &vertices, &texture_coords));
                }
                "vt" => {
                    let x = points.get(0).unwrap_or(&"").parse::<f64>().unwrap_or(0.0);
                    let y = points.get(1).unwrap_or(&"").parse::<f64>().unwrap_or(0.0);
                    let texture_coord = Point2::new(x, y);
                    texture_coords.push(texture_coord);
                }
                "usemtl" => {
                    let mat_name = String::from(*points.get(0).unwrap_or(&""));
                    let material_file_name = format!("{0}/{1}", path, mtl.materials.get(mat_name.as_str()).unwrap());
                    img = image::open(material_file_name);
                }
                _ => {}
            }
        }
        Self {
            path,
            vertices,
            faces
        }
    }
    fn parse_vertex(
        points: &[&str],
        transforms: &Vec<Box<dyn Fn(Point3<f64>) -> Point3<f64>>>
    ) -> Point3<f64> {
        let x = points.get(0)
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
    fn parse_face(points: &[&str], img: &ImageResult<DynamicImage>, vertices: &Vec<Point3<f64>>, texture_coords: &Vec<Vector2<f64>>) -> Hittable {
        let i = img.as_ref().expect("asdf");
        let (x_v, r_t, _) = Model::parse_face_index(points[0]);
        let (y_v, g_t, _) = Model::parse_face_index(points[1]);
        let (z_v, b_t, _) = Model::parse_face_index(points[2]);
        let r_coord = texture_coords[r_t];
        let g_coord = texture_coords[g_t];
        let b_coord = texture_coords[b_t];
        let width = i.width() as f64;
        let height = i.height() as f64;
        let r = i.get_pixel((width*r_coord.x) as u32, (height*r_coord.y) as u32).to_rgb().0;
        let g = i.get_pixel((width*g_coord.x) as u32, (height*g_coord.y) as u32).to_rgb().0;
        let b = i.get_pixel((width*b_coord.x) as u32, (height*b_coord.y) as u32).to_rgb().0;
        Hittable::Triangle {
            a: vertices[x_v],
            b: vertices[y_v],
            c: vertices[z_v],
            material: Material::Lambertian {
                albedo: Color3::new(
                (r[0] as f64 + r[1] as f64 + r[2] as f64) / 3.0 / 255.0,
                (g[0] as f64 + g[1] as f64 + g[2] as f64) / 3.0 / 255.0,
                (b[0] as f64 + b[1] as f64 + b[2] as f64) / 3.0 / 255.0
                )
            }
        }
    }
    fn parse_face_index(s: &str) -> (usize, usize, usize) {
        let tokens = s.split("/").collect::<Vec<&str>>();
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
                    if name.is_some() {
                        current_material = String::from(name.unwrap());
                    }
                },
                "map_Kd" => {
                    let file_name = points.get(0);
                    if file_name.is_some() {
                        materials.insert(String::from(&current_material), String::from(file_name.unwrap()));
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
    let tokens = s.split(" ").collect::<Vec<&str>>();
    let id = String::from(*tokens.first().unwrap_or(&"#"));
    let mut data: Vec<String> = vec![];
    for point in &tokens[1..] {
        data.push(String::from(*point));
    }
    (id, data)
}
