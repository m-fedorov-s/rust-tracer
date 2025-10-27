#![allow(unused)]
use crate::geometry::{Point, Ray, Triangle, Vector};
use std::collections::HashMap;
use std::error;
use std::fs;
use std::io;
use std::fmt;
use std::num::{ParseFloatError, ParseIntError};
use std::io::BufRead;

// See https://en.wikipedia.org/wiki/Illumination_model#Illumination_models
// This may be outdated ?
#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub enum IlluminationModel {
    // 0. Color on and Ambient off
    #[default]
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

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Material {
    pub ambient: (f64, f64, f64),
    pub spectral: (f64, f64, f64),
    pub diffuse: (f64, f64, f64),
    pub specular_exponent: f64,
    pub transparency: f64,
    pub optical_density: f64,
    pub illumination_model: IlluminationModel,
}

struct LightSource {
    place: Point,
    saturation: (f64, f64, f64),
}

pub trait MaterialObject {
    // MaterialObject may be some of geometric figure with a material describing it.
    fn intersects(&self, ray: &Ray) -> Option<Point>;

    fn material(&self) -> Material;

    fn normale(&self, point: &Point) -> Option<Vector>;
}

pub struct Scene {
    // vector of Object's
    objects: Vec<Box<dyn MaterialObject>>,
    // vector of LightSource's
    lights: Vec<LightSource>,
}

#[derive(Debug)]
enum UnderlyingPasingError {
    IoError(io::Error),
    ParseIntError(ParseIntError),
    ParseFloatError(ParseFloatError),
    Other(String),
}

#[derive(Debug)]
struct ParsingError {
    filename: String,
    line_number: usize,
    error: UnderlyingPasingError,
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "file {} line {}: {:?}",
            self.filename, self.line_number, self.error
        )
    }
}

impl error::Error for ParsingError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self.error {
            UnderlyingPasingError::IoError(err) => Some(err as &(dyn error::Error + 'static)),
            UnderlyingPasingError::ParseIntError(err) => Some(err as &(dyn error::Error + 'static)),
            UnderlyingPasingError::ParseFloatError(err) => Some(err as &(dyn error::Error + 'static)),
            UnderlyingPasingError::Other(_) => None,
        }
    }
}

impl ParsingError {
    pub fn from_error(filename: String, line_number: &usize, err: UnderlyingPasingError) -> Self {
        ParsingError {
            filename,
            line_number: *line_number,
            error: err,
        }
    }
}

fn import_material_file(filename: &str) -> Result<HashMap<String, Material>, ParsingError> {
    let mut materials: HashMap<String, Material> = HashMap::new();
    let file = fs::File::open(filename).unwrap();
    let mut current_material = Default::default();
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
        if words.is_empty() {
            continue;
        }
        match words.first() {
            Some(token) => match *token {
                "newmtl" => {
                    if words.len() < 2 {
                        return Err(ParsingError {
                            filename: filename.to_string(),
                            line_number,
                            error: UnderlyingPasingError::Other(
                                "Expected material name after newmtl".to_string(),
                            ),
                        });
                    }
                    let material_name = words[1].to_string();
                    current_material = Default::default();
                    materials.insert(material_name, current_material);
                }
                "Ka" | "Ke" => {
                    if words.len() != 4 {
                        return Err(ParsingError {
                            filename: filename.to_string(),
                            line_number,
                            error: UnderlyingPasingError::Other(format!(
                                "Expected three coordinates after token `{}`, got {} instead.",
                                words[0],
                                words.len() - 1
                            )),
                        });
                    }
                    current_material.ambient = (
                        words[1].parse::<f64>().map_err(|e| {
                            ParsingError::from_error(
                                filename.to_string(),
                                &line_number,
                                UnderlyingPasingError::ParseFloatError(e),
                            )
                        })?,
                        words[2].parse::<f64>().map_err(|e| {
                            ParsingError::from_error(
                                filename.to_string(),
                                &line_number,
                                UnderlyingPasingError::ParseFloatError(e),
                            )
                        })?,
                        words[3].parse::<f64>().map_err(|e| {
                            ParsingError::from_error(
                                filename.to_string(),
                                &line_number,
                                UnderlyingPasingError::ParseFloatError(e),
                            )
                        })?,
                    );
                }
                "Kd" => {
                    current_material.diffuse = (
                        words[1].parse::<f64>().map_err(|e| {
                            ParsingError::from_error(
                                filename.to_string(),
                                &line_number,
                                UnderlyingPasingError::ParseFloatError(e),
                            )
                        })?,
                        words[2].parse::<f64>().map_err(|e| {
                            ParsingError::from_error(
                                filename.to_string(),
                                &line_number,
                                UnderlyingPasingError::ParseFloatError(e),
                            )
                        })?,
                        words[3].parse::<f64>().map_err(|e| {
                            ParsingError::from_error(
                                filename.to_string(),
                                &line_number,
                                UnderlyingPasingError::ParseFloatError(e),
                            )
                        })?,
                    );
                }
                "Ks" => {
                    current_material.spectral = (
                        words[1].parse::<f64>().map_err(|e| {
                            ParsingError::from_error(
                                filename.to_string(),
                                &line_number,
                                UnderlyingPasingError::ParseFloatError(e),
                            )
                        })?,
                        words[2].parse::<f64>().map_err(|e| {
                            ParsingError::from_error(
                                filename.to_string(),
                                &line_number,
                                UnderlyingPasingError::ParseFloatError(e),
                            )
                        })?,
                        words[3].parse::<f64>().map_err(|e| {
                            ParsingError::from_error(
                                filename.to_string(),
                                &line_number,
                                UnderlyingPasingError::ParseFloatError(e),
                            )
                        })?,
                    );
                }
                "Ns" => {
                    current_material.specular_exponent = words[1].parse::<f64>().map_err(|e| {
                        ParsingError::from_error(
                            filename.to_string(),
                            &line_number,
                            UnderlyingPasingError::ParseFloatError(e),
                        )
                    })?;
                }
                "Ni" => {
                    current_material.optical_density = words[1].parse::<f64>().map_err(|e| {
                        ParsingError::from_error(
                            filename.to_string(),
                            &line_number,
                            UnderlyingPasingError::ParseFloatError(e),
                        )
                    })?;
                }
                "d" => {
                    current_material.transparency = 1.0
                        - words[1].parse::<f64>().map_err(|e| {
                            ParsingError::from_error(
                                filename.to_string(),
                                &line_number,
                                UnderlyingPasingError::ParseFloatError(e),
                            )
                        })?;
                }
                "Tr" => {
                    current_material.transparency = words[1].parse::<f64>().map_err(|e| {
                        ParsingError::from_error(
                            filename.to_string(),
                            &line_number,
                            UnderlyingPasingError::ParseFloatError(e),
                        )
                    })?;
                }
                "illum" => {
                    current_material.illumination_model =
                        match words[1].parse::<i32>().map_err(|e| {
                            ParsingError::from_error(
                                filename.to_string(),
                                &line_number,
                                UnderlyingPasingError::ParseIntError(e),
                            )
                        })? {
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
                            _ => {
                                return Err(ParsingError {
                                    filename: filename.to_string(),
                                    line_number,
                                    error: UnderlyingPasingError::Other(format!(
                                        "Unsupported illumination model {} in line {} of file {}",
                                        words[1], line_number, filename
                                    )),
                                });
                            }
                        }
                }
                _ => continue,
            },
            None => continue,
        }
    }
    Ok(materials)
}

impl Scene {
    pub fn intersects(&self, ray: &Ray) -> Option<(Box<dyn MaterialObject>, Point)> // better Option<f64> - distance?
    {
        panic!("Not implemented!");
    }

    pub fn from_file(filename: &str) -> Scene {
        let file = fs::File::open(filename).unwrap();
        let mut scene = Scene {
            lights: Vec::new(),
            objects: Vec::new(),
        };
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
                    vertexes.push(Box::new(Point::new(
                        words[1].parse().unwrap(),
                        words[2].parse().unwrap(),
                        words[3].parse().unwrap(),
                    )));
                }
                "vt" => {
                    panic!("Not implemented!");
                    // textures_coordinates.append();
                }
                "vn" => {
                    normales.push(Box::new(Vector::new(
                        words[1].parse().unwrap(),
                        words[2].parse().unwrap(),
                        words[3].parse().unwrap(),
                    )));
                }
                "f" => {
                    if current_material.is_none() {
                        panic!("Material is not set!");
                    }
                    panic!("Not implemented!");
                    // let point_a = vertexes[words[1].parse().unwrap()];
                    // let point_b = vertexes[words[2].parse().unwrap()];
                    // for index_c in words.iter().skip(3).map(|i| i.parse().unwrap()) {
                    //     let triangle = Triangle::new(point_a, point_b, vertexes[index_c]);
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
                    let place = Point::new(
                        words[1].parse().unwrap(),
                        words[2].parse().unwrap(),
                        words[3].parse().unwrap(),
                    );
                    let source = LightSource {
                        place,
                        saturation: (
                            words[4].parse().unwrap(),
                            words[5].parse().unwrap(),
                            words[6].parse().unwrap(),
                        ),
                    };
                    scene.lights.push(source);
                }
                "mtllib" => {
                    let mtl_filename = words[1];
                    let result = import_material_file(mtl_filename);
                    match result {
                        Ok(new_materials) => materials.extend(new_materials),
                        Err(err) => println!("Error during importing file: {}", err),
                    };
                }
                "usemtl" => match materials.get(words[1]) {
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

    fn add_object(object: Box<dyn MaterialObject>) {
        panic!("Not implemented!");
    }

    fn add_light_source(light: LightSource) {
        panic!("Not implemented!");
    }
}

#[cfg(test)]
mod tests {}
