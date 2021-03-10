#![allow(unused)]
use crate::geometry::{Point, Triangle, Ray};
// TODO

struct Material {
    spectral: (f64, f64, f64),
    diffuse: (f64, f64, f64)
    // So on ...
}

struct LightSource {
    place: Point,
    saturation: (f64, f64, f64)
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

impl Scene {
    fn intersects(&self, ray: &Ray) -> Option<Point> // beter Option<f64> - distance&
    {
        panic!("Not implemented!");
    }

    fn from_file(filename: &str) -> Scene {
        panic!("Not implemented!");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
