use crate::geometry::GeometricObject;
use crate::geometry::{Point, Ray, Sphere, Triangle, Vector};
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::io;
use std::io::BufRead;
use std::num::{ParseFloatError, ParseIntError};
use std::{error, str};

// See https://en.wikipedia.org/wiki/Illumination_model#Illumination_models
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

// Returns a sensible default gray material used when an OBJ face/sphere
// references no material. OBJ files may omit usemtl for some geometry,
// so we fall back to a neutral gray instead of treating that as an error.
fn default_gray_material() -> Material {
    Material {
        // modest ambient + diffuse gray
        ambient: (0.25, 0.25, 0.25),
        diffuse: (0.5, 0.5, 0.5),
        // no spectral/specular color by default
        spectral: (0.0, 0.0, 0.0),
        specular_exponent: 10.0,
        transparency: 0.0,
        optical_density: 1.0,
        illumination_model: IlluminationModel::ColorOnly,
    }
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

pub struct MaterialObjectImpl {
    material: Material,
    object: Box<dyn GeometricObject>,
}

impl MaterialObject for MaterialObjectImpl {
    fn intersects(&self, ray: &Ray) -> Option<Point> {
        self.object.intersects(ray)
    }
    fn material(&self) -> Material {
        self.material
    }
    fn normale(&self, point: &Point) -> Option<Vector> {
        self.object.normale(point)
    }
}

impl MaterialObjectImpl {
    fn new(m: Material, obj: Box<dyn GeometricObject>) -> MaterialObjectImpl {
        MaterialObjectImpl {
            material: m,
            object: obj,
        }
    }
}

pub struct Scene {
    // vector of Object's
    objects: Vec<Box<dyn MaterialObject>>,
    // vector of LightSource's
    lights: Vec<LightSource>,
}

impl fmt::Debug for Scene {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "Scene: {} objects and {} lights",
            self.objects.len(),
            self.lights.len()
        )
    }
}

#[derive(Debug)]
pub enum UnderlyingPasingError {
    IoError(io::Error),
    ParseIntError(ParseIntError),
    ParseFloatError(ParseFloatError),
    Other(String),
}

impl fmt::Display for UnderlyingPasingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnderlyingPasingError::IoError(err) => write!(f, "IoError: {}", err),
            UnderlyingPasingError::ParseIntError(err) => write!(f, "ParseIntError: {}", err),
            UnderlyingPasingError::ParseFloatError(err) => write!(f, "ParseFloatError: {}", err),
            UnderlyingPasingError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl From<ParseFloatError> for UnderlyingPasingError {
    fn from(err: ParseFloatError) -> Self {
        UnderlyingPasingError::ParseFloatError(err)
    }
}

impl From<ParseIntError> for UnderlyingPasingError {
    fn from(err: ParseIntError) -> Self {
        UnderlyingPasingError::ParseIntError(err)
    }
}

#[derive(Debug)]
pub struct ParsingError {
    filename: String,
    pub line_number: usize,
    pub error: UnderlyingPasingError,
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "file \"{}\" at line {}: {}",
            self.filename, self.line_number, self.error
        )
    }
}

impl error::Error for ParsingError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self.error {
            UnderlyingPasingError::IoError(err) => Some(err as &(dyn error::Error + 'static)),
            UnderlyingPasingError::ParseIntError(err) => Some(err as &(dyn error::Error + 'static)),
            UnderlyingPasingError::ParseFloatError(err) => {
                Some(err as &(dyn error::Error + 'static))
            }
            UnderlyingPasingError::Other(_) => None,
        }
    }
}

impl ParsingError {
    pub fn new(filename: &str, line_number: usize, err: UnderlyingPasingError) -> Self {
        ParsingError {
            filename: filename.to_owned(),
            line_number,
            error: err,
        }
    }
}

/**
* Parsing the following lines:
* newmtl <material name> # define a material with a new name
* Ka <float> <float> <float> # set the ambient color
* Kd <float> <float> <float> # set the diffuse color
* Ks <float> <float> <float> # set the specular color
* Ns <float> # set the specular exponent
* Tr <float> # set material transperency
* d <float> # same with Tr = 1 - d
* Ni <float> # set optical density (aka index of refraction)
* illum <int> # set one of the illumination models
*
* Lines, starting with '#' are comments, so ignored.
*/
pub fn import_material_file(filename: &str) -> Result<HashMap<String, Material>, ParsingError> {
    let file = fs::File::open(filename)
        .map_err(|err| ParsingError::new(filename, 0, UnderlyingPasingError::IoError(err)))?;

    let mut materials: HashMap<String, Material> = HashMap::new();
    let mut current_material_name = String::new();

    for (line_number, line_result) in io::BufReader::new(file).lines().enumerate() {
        let inputline = line_result.map_err(|err| {
            ParsingError::new(filename, line_number, UnderlyingPasingError::IoError(err))
        })?;

        let trimmed = inputline.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        let words: Vec<&str> = trimmed.split_whitespace().collect();
        let token = words[0];

        match token {
            "newmtl" => {
                let name = words.get(1).ok_or_else(|| {
                    ParsingError::new(
                        filename,
                        line_number,
                        UnderlyingPasingError::Other("Expected material name after newmtl".into()),
                    )
                })?;
                current_material_name = name.to_string();
                materials.insert(current_material_name.clone(), Material::default());
            }
            "Ka" | "Ke" | "Kd" | "Ks" | "Ns" | "Ni" | "d" | "Tr" | "illum" => {
                let material = materials.get_mut(&current_material_name).ok_or_else(|| {
                    ParsingError::new(
                        filename,
                        line_number,
                        UnderlyingPasingError::Other(format!(
                            "Token '{}' found before any 'newmtl' declaration",
                            token
                        )),
                    )
                })?;

                match token {
                    "Ka" | "Ke" => {
                        material.ambient = parse_triple(&words[1..], filename, line_number)?
                    }
                    "Kd" => material.diffuse = parse_triple(&words[1..], filename, line_number)?,
                    "Ks" => material.spectral = parse_triple(&words[1..], filename, line_number)?,
                    "Ns" => {
                        material.specular_exponent =
                            parse_number::<f64>(words.get(1), filename, line_number)?
                    }
                    "Ni" => {
                        material.optical_density =
                            parse_number::<f64>(words.get(1), filename, line_number)?
                    }
                    "d" => {
                        material.transparency =
                            1.0 - parse_number::<f64>(words.get(1), filename, line_number)?
                    }
                    "Tr" => {
                        material.transparency =
                            parse_number::<f64>(words.get(1), filename, line_number)?
                    }
                    "illum" => {
                        material.illumination_model =
                            parse_illumination_model(words.get(1), filename, line_number)?
                    }
                    _ => unreachable!(),
                }
            }
            _ => continue,
        }
    }
    Ok(materials)
}

fn parse_number<N: str::FromStr>(
    value: Option<&&str>,
    filename: &str,
    line_number: usize,
) -> Result<N, ParsingError>
where
    UnderlyingPasingError: From<<N as str::FromStr>::Err>,
{
    value
        .ok_or_else(|| {
            ParsingError::new(
                filename,
                line_number,
                UnderlyingPasingError::Other(format!(
                    "Expected value of type {}",
                    std::any::type_name::<N>()
                )),
            )
        })?
        .parse::<N>()
        .map_err(|e| ParsingError::new(filename, line_number, e.into()))
}

fn parse_triple(
    words: &[&str],
    filename: &str,
    line_number: usize,
) -> Result<(f64, f64, f64), ParsingError> {
    if words.len() != 3 {
        return Err(ParsingError::new(
            filename,
            line_number,
            UnderlyingPasingError::Other(format!(
                "Expected three floats, got {} instead.",
                words.len()
            )),
        ));
    }
    let r = parse_number::<f64>(Some(&words[0]), filename, line_number)?;
    let g = parse_number::<f64>(Some(&words[1]), filename, line_number)?;
    let b = parse_number::<f64>(Some(&words[2]), filename, line_number)?;
    Ok((r, g, b))
}

fn parse_illumination_model(
    value: Option<&&str>,
    filename: &str,
    line_number: usize,
) -> Result<IlluminationModel, ParsingError> {
    let val = parse_number::<i32>(value, filename, line_number)?;

    match val {
        0 => Ok(IlluminationModel::ColorOnly),
        1 => Ok(IlluminationModel::ColorAndAmbient),
        2 => Ok(IlluminationModel::ColorHighlight),
        3 => Ok(IlluminationModel::ReflectionOn),
        4 => Ok(IlluminationModel::TransparencyOn),
        5 => Ok(IlluminationModel::ReflectionFresnel),
        6 => Ok(IlluminationModel::RefractionAndReflectionOn),
        7 => Ok(IlluminationModel::RefractionAndReflectionFresnel),
        8 => Ok(IlluminationModel::ReflectionAndRayTraceOff),
        9 => Ok(IlluminationModel::TransparencyOnAndRayTraceOff),
        10 => Ok(IlluminationModel::Shadows),
        _ => Err(ParsingError::new(
            filename,
            line_number,
            UnderlyingPasingError::Other(format!(
                "Unsupported illumination model {} in line {} of file {}",
                val, line_number, filename
            )),
        )),
    }
}

impl Scene {
    pub fn intersects(&self, ray: &Ray) -> Option<(Box<dyn MaterialObject>, Point)> // better Option<f64> - distance?
    {
        for object in &self.objects {
            let point = object.intersects(ray);
            if let Some(point) = point {
                if object.normale(&point).is_some() {
                    break;
                }
            }
        }
        panic!("Not implemented!");
    }

    pub fn from_file(filename: &str) -> Result<Scene, ParsingError> {
        let file = fs::File::open(filename)
            .map_err(|err| ParsingError::new(filename, 0, UnderlyingPasingError::IoError(err)))?;
        let mut scene = Scene {
            lights: Vec::new(),
            objects: Vec::new(),
        };
        let mut materials: HashMap<String, Material> = HashMap::new();
        let mut vertexes: Vec<Point> = vec![];
        let mut textures_coordinates: Vec<(f64, f64, f64)> = vec![];
        let mut normales: Vec<Box<Vector>> = vec![];
        let mut current_material: Material = default_gray_material();
        for (line_number, line_result) in io::BufReader::new(file).lines().enumerate() {
            let inputline = line_result.map_err(|err| {
                ParsingError::new(filename, line_number, UnderlyingPasingError::IoError(err))
            })?;
            let inputline = inputline.trim();
            if inputline.is_empty() || inputline.starts_with('#') {
                // Skip comments and empty lines
                continue;
            }
            let words: Vec<&str> = inputline.split_whitespace().map(|s| s.trim()).collect();
            match words[0] {
                "v" => {
                    vertexes.push(Point::from(parse_triple(
                        &words[1..],
                        filename,
                        line_number,
                    )?));
                }
                "vt" => {
                    textures_coordinates.push((
                        parse_number::<f64>(words.get(1), filename, line_number)?,
                        parse_number::<f64>(words.get(2), filename, line_number).unwrap_or(0.),
                        parse_number::<f64>(words.get(3), filename, line_number).unwrap_or(0.),
                    ));
                }
                "vn" => {
                    normales.push(Box::new(Vector::from(parse_triple(
                        &words[1..],
                        filename,
                        line_number,
                    )?)));
                }
                "f" => {
                    let a_idx = parse_number::<i32>(words.get(1), filename, line_number)?;
                    let b_idx = parse_number::<i32>(words.get(2), filename, line_number)?;
                    if a_idx < 1 || a_idx > vertexes.len() as i32 {
                        return Err(ParsingError::new(
                            filename,
                            line_number,
                            UnderlyingPasingError::Other(format!(
                                "Vertex index {} is out of range",
                                a_idx
                            )),
                        ));
                    }
                    let point_a = &vertexes[(a_idx - 1) as usize];
                    let mut point_b = &vertexes[(b_idx - 1) as usize];
                    for index_c in words
                        .iter()
                        .skip(3)
                        .map(|i| parse_number::<i32>(Some(i), filename, line_number))
                    {
                        let index_c = index_c?;
                        if index_c < 1 || index_c > vertexes.len() as i32 {
                            return Err(ParsingError::new(
                                filename,
                                line_number,
                                UnderlyingPasingError::Other(format!(
                                    "Vertex index {} is out of range",
                                    a_idx
                                )),
                            ));
                        }
                        let triangle = Box::new(Triangle::new(
                            *point_a,
                            *point_b,
                            vertexes[(index_c - 1) as usize],
                        ));
                        scene.add_object(Box::new(MaterialObjectImpl::new(
                            current_material,
                            triangle,
                        )));
                        point_b = &vertexes[(index_c - 1) as usize];
                    }
                }
                "S" => {
                    let center = Point::from(parse_triple(&words[1..4], filename, line_number)?);
                    let raw_object = Box::new(Sphere::new(
                        center,
                        parse_number::<f64>(words.get(4), filename, line_number)?,
                    ));

                    scene.add_object(Box::new(MaterialObjectImpl::new(
                        current_material,
                        raw_object,
                    )));
                }
                "P" => {
                    let place = Point::from(parse_triple(&words[1..4], filename, line_number)?);
                    let source = LightSource {
                        place,
                        saturation: parse_triple(&words[4..], filename, line_number)?,
                    };
                    scene.add_light_source(source);
                }
                "mtllib" => {
                    let mtl_filename = words[1];
                    materials.extend(import_material_file(mtl_filename)?.into_iter());
                }
                "usemtl" => {
                    if words.len() != 2 {
                        return Err(ParsingError::new(
                            filename,
                            line_number,
                            UnderlyingPasingError::Other(
                                "Expected material name after usemtl".into(),
                            ),
                        ));
                    }
                    current_material = *materials.get(words[1]).ok_or(ParsingError::new(
                        filename,
                        line_number,
                        UnderlyingPasingError::Other(format!("Unknown material {}", words[1])),
                    ))?;
                }
                _ => continue,
            }
        }
        Ok(scene)
    }

    fn add_object(&mut self, object: Box<dyn MaterialObject>) {
        // TODO use more efficient data scructure for faster ray-object intersection.
        self.objects.push(object);
    }

    fn add_light_source(&mut self, light: LightSource) {
        self.lights.push(light);
    }
}

#[cfg(test)]
mod tests {}
