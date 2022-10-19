#![allow(unused)]
#[derive(Debug, PartialEq)]
pub struct Point(f64, f64, f64);

#[derive(Debug, PartialEq)]
pub struct Vector(f64, f64, f64);

#[derive(Debug, PartialEq)]
pub struct Triangle {} //??tree points?

#[derive(Debug, PartialEq)]
pub struct Ray {} // ?Two points, point and a vector??

impl Point {
    fn distance(&self, other: &Point) -> f64 {
        self.vector_to(other).norm()
    }

    fn vector_to(&self, endpoint: &Point) -> Vector {
        Vector(
            endpoint.0 - self.0,
            endpoint.1 - self.1,
            endpoint.2 - self.2,
        )
    }

    fn shift_by(&self, shift: &Vector) -> Point {
        Point(self.0 + shift.0, self.1 + shift.1, self.2 + shift.2)
    }
}

impl Vector {
    fn add(&self, other: &Vector) -> Vector {
        Vector(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }

    fn norm(&self) -> f64 {
        (self.0.powi(2) + self.1.powi(2) + self.2.powi(2)).sqrt()
    }

    fn multiply_by(&self, scalar: f64) -> Vector {
        Vector(self.0 * scalar, self.1 * scalar, self.2 * scalar)
    }

    fn scalar_product(&self, other: &Vector) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    fn vector_product(&self, other: &Vector) -> Vector {
        Vector(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }

    fn normalize(&self) -> Vector {
        let norm = self.norm();
        if norm == 0.0 {
            panic!("Normalizing zero vector!")
        }
        Vector(self.0 / norm, self.1 / norm, self.2 / norm)
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
// этой вершины в текстуре. Кроме того, для улучшения качества нужно помнить нормали в
// каждой вершине, и рассчитывать нормаль в точке как взвешенную сумму нормалей в
// вершинах, с весами, равными барацентрическим координатам.

// struct Sphere {...}
// Хотим поддержку шариков ведь, так?)

#[cfg(test)]
mod tests {
    use super::*;

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
        assert!(
            a_point.distance(&b_point) + b_point.distance(&c_point) >= a_point.distance(&c_point)
        );
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
    fn shift_by_zero_vector() {
        let point = Point(1., 5., 6.);
        let vector = Vector(0., 0., 0.);
        assert_eq!(point, point.shift_by(&vector));
    }

    #[test]
    fn vector_norm() {
        let vectors_norms_pairs = vec![
            (Vector(0., 0., 0.), 0.),
            (Vector(0., 0., 7.), 7.),
            (Vector(1., 0., 0.), 1.),
            (Vector(0., 3., 0.), 3.),
            (Vector(4., 3., 0.), 5.),
            (Vector(14., 2., 5.), 15.),
        ];
        for (vector, expected_norm) in vectors_norms_pairs {
            assert_eq!(vector.norm(), expected_norm);
        }
    }

    #[test]
    fn normalizing_vector() {
        let vector = Vector(1., 3., 7.);
        assert_eq!(1., vector.normalize().norm());
    }

    #[test]
    #[should_panic]
    fn normalizing_zero_vector() {
        let zero_vector = Vector(0., 0., 0.);
        let other = zero_vector.normalize();
        print!("{other:?}")
    }

    #[test]
    fn vector_product_for_standard_basis() {
        let a_vector = Vector(1., 0., 0.);
        let b_vector = Vector(0., 1., 0.);
        let product = a_vector.vector_product(&b_vector);
        assert_eq!(Vector(0.0, 0.0, 1.0), product);
    }

    #[test]
    fn vector_product_is_orthogonal() {
        let a_vector = Vector(0., 2., 5.);
        let b_vector = Vector(1., -4., 12.);
        let product = a_vector.vector_product(&b_vector);
        assert_eq!(0., product.scalar_product(&a_vector));
        assert_eq!(0., product.scalar_product(&b_vector));
    }

    #[test]
    fn vector_product_with_zero_vector() {
        let zero_vector = Vector(0., 0., 0.);
        let vector = Vector(-4., 2., 9.);
        assert_eq!(0., zero_vector.vector_product(&vector).norm());
    }
}
