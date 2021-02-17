#![allow(unused)]

pub struct Point(f64, f64, f64);
struct Vector(f64, f64, f64);

pub struct Triangle {}//??tree points?
pub struct Ray {} // ?Two points, point and a vector??

impl Point {
    fn distance(&self, other: &Point) -> f64 {
        panic!("Not implemented!");
    }

    fn vector_to(&self, endpoint: &Point) -> Vector {
        panic!("Not implemented!");
    }

    fn shift_by(&self, sift: &Vector) -> Point {
        panic!("Not implemented!");

    }
}

impl Vector {
    fn add(&self, other: &Vector) -> Point {
        panic!("Not implemented!");
    }

    fn norm(&self) -> f64 {
        panic!("Not implemented!");
    }

    fn multiply_by(&self, scalar: f64) {
        panic!("Not implemented!");
    }

    fn scalar_product(&self, other: &Vector) -> f64 {
        panic!("Not implemented!");
    }

    fn vector_product(&self, other: &Vector) -> Vector {
        panic!("Not implemented!");
    }

    fn normalize(&mut self) {
        panic!("Not implemented!");
    }

    fn rotate(&self, axis: &Vector, angle: f64) -> Vector {
        // Not needed right now
        panic!("Not implemented!");
    }
}

// impl Triangle {
//     fn Intersects(&self, ray: &Ray) -> bool {}
//     fn Intersects(&self, ray: &Ray) -> (bool, Point) {}
//     ???
//
//     fn Area() -> f64
//
//     fn Normle() -> Vector
//
//     fn BaracentricCoordinates() -> ???
// }

// Чтобы правильно рассчитывать освещенность в точке, нужно считать нормаль в этой точке. 
// Кроме того, чтобы поддерживать текстуры необходимо рассчитывать барацентрические 
// координаты точки в треугольнике, а для каждой вершины треугольника помнить координаты
// этой верщины в текстуре. Кроме того, для улучшения качества нужно помнить нормали в
// каждой вершине, и рассчитывать нормаль в точке как взвешенную сумму нормалей в 
// вершинах, с весами, равными барацентрическим координатам.

// struct Sphere {...}
// Хотим поддержку шариков ведь, так?)