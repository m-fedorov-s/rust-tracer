#![allow(unused)]
#[derive(Debug)]
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

    fn shift_by(&self, shift: &Vector) -> Point {
        panic!("Not implemented!");
    }
}

impl Vector {
    fn add(&self, other: &Vector) -> Vector {
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


#[cfg(test)]
mod tests {
    use crate::Vector;
    use crate::Point;

    #[test]
    fn zero_distance_between_point_and_itself() {
        let point = Point(1., 4., 5.);
        assert_eq!(point.distance(&point), 0.0);
    }

    #[test]
    fn distance_is_positive() {
        let point = Point(1., 4., 5.);
        let other_point = Point(2., 1., -4.);
        assert!(point.distance(&other_point) > 0.0);
    }

    #[test]
    fn distance_is_symmetric() {
        let point = Point(1., 4., 5.);
        let other_point = Point(2., 1., -4.);
        assert_eq!(point.distance(&other_point), other_point.distance(&point));
    }

    #[test]
    fn triangle_inequality() {
        let a_point = Point(1., 4., 5.);
        let b_point = Point(2., 1., -4.);
        let c_point = Point(2., -1., 2.);
        assert!(a_point.distance(&b_point) +  b_point.distance(&c_point) >= a_point.distance(&c_point));
    }

    #[test]
    fn distance_gives_right_answer() {
        let a_point = Point(1., 1., 2.);
        let b_point = Point(1., 1., 0.);
        assert_eq!(a_point.distance(&b_point), 2.);
        let a_point = Point(1., 1., 2.);
        let b_point = Point(1., 4., 2.);
        assert_eq!(a_point.distance(&b_point), 3.);
        let a_point = Point(7., 1., 0.);
        let b_point = Point(1., 1., 0.);
        assert_eq!(a_point.distance(&b_point), 6.);
    }

    #[test]
    fn zero_vector() {
        let point = Point(1., 5., 6.);
        let vector = Vector(0., 0., 0.);
        assert_eq!(point.0, point.shift_by(&vector).0);
        assert_eq!(point.1, point.shift_by(&vector).1);
        assert_eq!(point.2, point.shift_by(&vector).2);
        assert_eq!(0., vector.norm());
    }
}