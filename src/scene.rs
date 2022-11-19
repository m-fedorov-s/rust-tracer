#![allow(unused)]
use crate::geometry::{Point, Ray, Triangle, Vector};
use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::BufRead;

// See https://en.wikipedia.org/wiki/Illumination_model#Illumination_models
// This may be outdated ?
#[derive(Debug, Copy, Clone, PartialEq)]
enum IlluminationModel {
    // 0. Color on and Ambient off
    ColorOnly,
    // 1. Color on and Ambient on
    ColorAndAmbient,
    // 2. Highlight on
    ColorHighlight,
    // 3. Reflection on and Ray trace on
    ReflectionOn,
    // 4. Transparency: Glass on, Reflection: Ray trace on
    TransparencyOn,
    // 5. Reflection: Fresnel on and Ray trace on
    ReflectionFresnel,
    // 6. Transparency: Refraction on, Reflection: Fresnel off and Ray trace on
    RefractionAndReflectionOn,
    // 7. Transparency: Refraction on, Reflection: Fresnel on and Ray trace on
    RefractionAndReflectionFresnel,
    // 8. Reflection on and Ray trace off
    ReflectionAndRayTraceOff,
    // 9. Transparency: Glass on, Reflection: Ray trace off
    TransparencyOnAndRayTraceOff,
    // 10. Casts shadows onto invisible surfaces
    Shadows,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Material {
    ambient: (f64, f64, f64),
    spectral: (f64, f64, f64),
    diffuse: (f64, f64, f64),
    specular_exponent: f64,
    transparency: f64,
    optical_density: f64,
    illumination_model: IlluminationModel,
}

impl Material {
    fn empty() -> Material {
        Material {
            ambient: (0.0, 0.0, 0.0),
            spectral: (0.0, 0.0, 0.0),
            diffuse: (0.0, 0.0, 0.0),
            specular_exponent: (0.0),
            transparency: 1.0,
            optical_density: 1.0,
            illumination_model: IlluminationModel::ReflectionOn,
        }
    }
}

struct LightSource {
    place: Point,
    saturation: (f64, f64, f64),
}

enum Object {
    // Objects may be some of geometric figure with a material describing it.
    Triangle(Material, Triangle),
    // Sphere(Material, Sphere),
}

impl Object {
    fn intersects(&self, ray: &Ray) -> bool // or Point or distance or Option<Point> ?
    {
        panic!("Not implemented");
    }

    fn reflect(&self, ray: &Ray) -> Ray {
        panic!("Not implemented");
    }
}

struct Scene {
    // vector of Object's
    // vector of LightSource's
}

fn import_material_file(filename: &str, materials: &mut HashMap<String, Material>) {
    let file = fs::File::open(filename).unwrap();
    let mut current_material = Material::empty();
    let mut material_name: Option<String> = None;
    for (line_number, inputline) in io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .enumerate()
    {
        if inputline.starts_with('#') {
            // Skip comments
            continue;
        }
        let words: Vec<&str> = inputline.split_whitespace().map(|s| s.trim()).collect();
        match words[0] {
            "newmtl" => {
                if let Some(name) = material_name {
                    materials.insert(name, current_material);
                    current_material = Material::empty();
                }
                material_name = Some(words[1].to_string());
            }
            "Ka" | "Ke" => {}
            "Kd" => {}
            "Ks" => {}
            "Ns" => {}
            "Ni" => {}
            "d" => {}
            "Tr" => {}
            "illum" => {
                current_material.illumination_model = match words[1].parse().unwrap() {
                    0 => IlluminationModel::ColorOnly,
                    1 => IlluminationModel::ColorAndAmbient,
                    2 => IlluminationModel::ColorHighlight,
                    3 => IlluminationModel::ReflectionOn,
                    4 => IlluminationModel::TransparencyOn,
                    5 => IlluminationModel::ReflectionFresnel,
                    6 => IlluminationModel::RefractionAndReflectionOn,
                    7 => IlluminationModel::RefractionAndReflectionFresnel,
                    8 => IlluminationModel::ReflectionAndRayTraceOff,
                    9 => IlluminationModel::TransparencyOnAndRayTraceOff,
                    10 => IlluminationModel::Shadows,
                    _ => panic!(
                        "Unsupported illumination model {} in line {} of file {}",
                        words[1], line_number, filename
                    ),
                }
            }
            _ => continue,
        }
    }
}

impl Scene {
    fn intersects(&self, ray: &Ray) -> Option<Point> // better Option<f64> - distance&
    {
        panic!("Not implemented!");
    }

    fn from_file(filename: &str) -> Scene {
        let file = fs::File::open(filename).unwrap();
        let mut scene = Scene {};
        let mut materials: HashMap<String, Material> = HashMap::new();
        let mut vertexes: Vec<Box<Point>> = vec![];
        let mut textures_coordinates: Vec<(f64, f64)> = vec![];
        let mut normales: Vec<Box<Vector>> = vec![];
        let mut current_material: Option<Material> = None;
        for inputline in io::BufReader::new(file).lines().map(|l| l.unwrap()) {
            if inputline.starts_with('#') {
                // Skip comments
                continue;
            }
            let words: Vec<&str> = inputline.split_whitespace().map(|s| s.trim()).collect();
            match words[0] {
                "v" => {
                    panic!("Not implemented!");
                    // vertexes.append(Point::new(
                    //     words[1].parse().unwrap(),
                    //     words[2].parse().unwrap(),
                    //     words[3].parse().unwrap(),
                    // ));
                }
                "vt" => {
                    panic!("Not implemented!");
                    // textures_coordinates.append();
                }
                "vn" => {
                    panic!("Not implemented!");
                    // normales.append(Vector::new(
                    //     words[1].parse().unwrap(),
                    //     words[2].parse().unwrap(),
                    //     words[3].parse().unwrap(),
                    // ));
                }
                "f" => {
                    if current_material.is_none() {
                        panic!("Material is not set!");
                    }
                    panic!("Not implemented!");
                    // let point_a = vertexes[words[1].parse().unwrap()];
                    // let point_b = vertexes[words[2].parse().unwrap()];
                    // for index_c in words.iter().skip(3).map(|i| i.parse().unwrap()) {
                    //     let triangle = Triangle::new(point_a, point_b, vertexes[index_c])
                    //     scene.add(triangle, current_material);
                    // }
                }
                "S" => {
                    if current_material.is_none() {
                        panic!("Material is not set!");
                    }
                    panic!("Not implemented!");
                    // let center = Point::new(
                    //     words[1].parse().unwrap(),
                    //     words[2].parse().unwrap(),
                    //     words[3].parse().unwrap(),
                    // );
                    // scene.add(Sphere::new(center, words[4].parse().unwrap()), current_material)
                }
                "P" => {
                    panic!("Not implemented!");
                    // let place = Point::new(
                    //     words[1].parse().unwrap(),
                    //     words[2].parse().unwrap(),
                    //     words[3].parse().unwrap(),
                    // );
                    // let source = LightSource {
                    //     place,
                    //     saturation: (
                    //         words[4].parse().unwrap(),
                    //         words[5].parse().unwrap(),
                    //         words[6].parse().unwrap(),
                    //     ),
                    // };
                    // scene.add(LightSource());
                }
                "mtllib" => {
                    let mtl_filename = words[1];
                    import_material_file(mtl_filename, &mut materials);
                }
                "usemtl" => match materials.get(&words[1].to_string()) {
                    Some(material) => {
                        current_material = Some(*material);
                    }
                    None => {
                        panic!("Material {} is not found", words[1]);
                    }
                },
                _ => continue,
            }
        }
        scene
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
