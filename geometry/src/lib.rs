#![allow(unused)]

pub struct Point(f64, f64, f64);
struct Vector(f64, f64, f64);

pub struct Triangle {}//??tree points?
pub struct Ray {} // ?Two points, point and a vector??

impl Point {
    fn distance(&self, other: &Point) -> f64 {
        self.vector_to(other).norm()
    }

    fn vector_to(&self, endpoint: &Point) -> Vector {
        Vector(endpoint.0 - self.0,
               endpoint.1 - self.1,
               endpoint.2 - self.2)
    }

    fn shift_by(&self, shift: &Vector) -> Point {
        Point(self.0 + shift.0,
              self.1 + shift.1,
              self.2 + shift.2)
    }
}

impl Vector {
    fn add(&self, other: &Vector) -> Vector {
        Vector(self.0 + other.0,
               self.1 + other.1,
               self.2 + other.2)
    }

    fn norm(&self) -> f64 {
        (self.0.powi(2) +
         self.1.powi(2) +
         self.2.powi(2)).sqrt()
    }

    fn multiply_by(&self, scalar: f64) -> Vector {
        Vector(self.0 * scalar,
               self.1 * scalar,
               self.2 * scalar)
    }

    fn scalar_product(&self, other: &Vector) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    fn vector_product(&self, other: &Vector) -> Vector {
        Vector(self.1 * other.2 - self.2 * other.1,
               self.2 * other.0 - self.0 * other.2,
               self.0 * other.1 - self.1 * other.0)
    }

    fn normalize(&mut self) -> Vector {
        let norm = self.norm();
        Vector(self.0 / norm,
               self.1 / norm,
               self.2 / norm)
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
