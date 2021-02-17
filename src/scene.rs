// import geometry???
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
    Sphere(Material, Sphere)
}

impl Object {
    fn Intersects(&self, ray: &Ray) -> // bool or Point or distance or optional<Point> ?
    {
        panic!("Not implemented");
    }

    fn Reflect(&self, ray: &Ray) -> Ray {
        panic!("Not implemented");
    }

    fn 
}

struct Scene {
    // vector of Object's
    // vector of LightSource's
}

impl Scene {
    fn Intersects(&self, ray: &Ray) -> optional<Point> // beter optional<f64> - distance&
    {
        panic!("Not implemented!");
    }

    fn FromFile(filename: str&) -> Scene {
        panic!("Not implemented!");
    }
}