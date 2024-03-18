#![allow(unused)]
use crate::geometry::{Point, Ray, Triangle, Vector}; // TODO Add Sphere

pub struct Material {
    pub spectral: (f64, f64, f64),
    pub diffuse: (f64, f64, f64), // So on ...
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

impl Scene {
    pub fn intersects(&self, ray: &Ray) -> Option<(Box<dyn MaterialObject>, Point)> // better Option<f64> - distance?
    {
        panic!("Not implemented!");
    }

    pub fn from_file(filename: &str) -> Scene {
        panic!("Not implemented!");
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
