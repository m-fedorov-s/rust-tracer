#![allow(unused)]

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point(f64, f64, f64);

#[derive(Debug, PartialEq)]
pub struct Vector(f64, f64, f64);

#[derive(Debug, PartialEq)]
pub struct Triangle(Point, Point, Point);

#[derive(Debug, PartialEq)]
pub struct Ray {} // ?Two points, point and a vector??

const FLOAT_THRESHOLD: f64 = 1e-8;

pub trait GeometricObject {
    fn intersects(&self, ray: &Ray) -> Option<Point>;
    fn normale(&self, point: &Point) -> Option<Vector>;
    // If the point does not belong to the object then the behavior is undefined.
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point(x, y, z)
    }

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

/*
So we can write
let a = Point(...);
let b = Point(...);
let v: Vector = &a - &b;
*/
impl<'a, 'b> std::ops::Sub<&'a Point> for &'b Point {
    type Output = Vector;

    fn sub(self: &'b Point, other: &'a Point) -> Self::Output {
        Vector(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

/*
So we can write
let a = Point(...);
let v = Vector(...);
let b: Point = a + &v;
*/
impl std::ops::Add<&Vector> for &Point {
    type Output = Point;

    fn add(self, other: &Vector) -> Self::Output {
        Point(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl std::ops::Add<&Vector> for &Vector {
    type Output = Vector;

    fn add(self, other: &Vector) -> Self::Output {
        Vector(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

/*
So we can write
let v = Vector(...);
let w = 0.4 * &v;
*/
impl std::ops::Mul<&Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: &Vector) -> Self::Output {
        Vector(self * rhs.0, self * rhs.1, self * rhs.2)
    }
}

/*
So we can write
let v = Vector(...);
let w = &v * 0.4;
*/
impl std::ops::Mul<f64> for &Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

/*
So we can write
let v = Vector(...);
let w = &v / 0.4;
*/
impl std::ops::Div<f64> for &Vector {
    type Output = Vector;

    fn div(self, rhs: f64) -> Self::Output {
        Vector(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector(x, y, z)
    }

    pub fn add(&self, other: &Vector) -> Vector {
        Vector(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }

    fn norm(&self) -> f64 {
        (self.0.powi(2) + self.1.powi(2) + self.2.powi(2)).sqrt()
    }

    pub fn multiply_by(&self, scalar: f64) -> Vector {
        Vector(self.0 * scalar, self.1 * scalar, self.2 * scalar)
    }

    fn scalar_product(&self, other: &Vector) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    pub fn vector_product(&self, other: &Vector) -> Vector {
        Vector(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }

    pub fn normalize(&self) -> Vector {
        let norm = self.norm();
        if norm.abs() < FLOAT_THRESHOLD {
            panic!("Normalizing zero vector!")
        }
        Vector(self.0 / norm, self.1 / norm, self.2 / norm)
    }

    pub fn rotate(&self, axis: &Vector, angle: f64) -> Vector {
        // Not needed right now
        panic!("Not implemented!");
    }
}

impl Triangle {
    fn area(&self) -> f64 {
        let side_one = &self.0 - &self.1;
        let side_two = &self.0 - &self.2;
        (side_one.vector_product(&side_two)).norm() / 2.0
    }

    fn baracentric_coordinates(&self, point: &Point) -> (f64, f64, f64) {
        let normale_0 = (point - &self.1).vector_product(&(point - &self.2));
        let area_0 = normale_0.norm();
        let normale_1 = (point - &self.2).vector_product(&(point - &self.0));
        let area_1 = normale_1.norm();
        let normale_2 = (point - &self.0).vector_product(&(point - &self.1));
        let area_2 = normale_2.norm();
        let sum = area_0 + area_1 + area_2;
        (area_0 / sum, area_1 / sum, area_2 / sum)
    }
}

impl GeometricObject for Triangle {
    fn intersects(&self, ray: &Ray) -> Option<Point> {
        // I have no definition of Ray right now
        panic!("Not implemented!");
    }

    fn normale(&self, point: &Point) -> Option<Vector> {
        let side_one = &self.0 - &self.1;
        let side_two = &self.0 - &self.2;
        Some(side_one.vector_product(&side_two).normalize())
    }
}

impl Ray {
    pub fn from_to(from: &Point, to: &Vector) -> Option<Ray> {
        Some(Ray {})
    }
}

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

    mod point {
        use crate::geometry::{Point, FLOAT_THRESHOLD};
        #[test]
        fn zero_distance_between_point_and_itself() {
            let point = Point(1., 4., 5.);
            assert!(point.distance(&point).abs() < FLOAT_THRESHOLD);
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
                a_point.distance(&b_point) + b_point.distance(&c_point)
                    >= a_point.distance(&c_point)
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
    }
    mod vector {
        use crate::geometry::{Point, Vector};
        #[test]
        fn zero_vector() {
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

    mod triangle {
        use crate::geometry::FLOAT_THRESHOLD;
        use crate::geometry::{GeometricObject, Point, Triangle};

        #[test]
        fn triangle_area() {
            let triangles_areas = vec![
                (
                    Triangle(Point(0., 0., 0.), Point(0., 1., 0.), Point(0., 0., 2.)),
                    1.0,
                ),
                (
                    Triangle(Point(-1., 0., 0.), Point(0., 1., 0.), Point(1., 1., 0.)),
                    0.5,
                ),
                (
                    Triangle(Point(-1., 0., 0.), Point(0., 1., 0.), Point(0., 0., -1.)),
                    3.0_f64.sqrt() / 2.,
                ),
            ];
            for (triangle, expected_area) in triangles_areas {
                assert_eq!(triangle.area(), expected_area)
            }
        }

        #[test]
        fn area_do_not_depend_on_permutations() {
            let points_sets = vec![
                (Point(1., 2., 3.), Point(-3., -2., -1.), Point(-6., 4., -2.)),
                (Point(-1., 1., 0.), Point(10., -3., 4.), Point(5., -1., 0.)),
                (Point(0., 0., 0.), Point(-10., 0., 2.), Point(0., -2., 3.)),
            ];
            for (a, b, c) in points_sets {
                let area = Triangle(a, b, c).area();
                assert_eq!(area, Triangle(a, c, b).area());
                assert_eq!(area, Triangle(b, a, c).area());
                assert_eq!(area, Triangle(b, c, a).area());
                assert_eq!(area, Triangle(c, a, b).area());
                assert_eq!(area, Triangle(c, b, a).area());
            }
        }

        #[test]
        fn triangle_zero_area() {
            let degenerate_triangles = vec![
                Triangle(Point(0., 0., 0.), Point(1., 1., 1.), Point(3., 3., 3.)),
                Triangle(Point(4., 0., 0.), Point(3., -1., -1.), Point(1., -3., -3.)),
                Triangle(Point(1., -2., 3.), Point(4., 5., -6.), Point(-2., -9., 12.)),
            ];
            for triangle in degenerate_triangles {
                assert!(triangle.area() >= 0.0 && triangle.area() < FLOAT_THRESHOLD);
            }
        }

        #[test]
        fn normale_to_triangle() {
            let a = Point(4., 2., 6.);
            let b = Point(-3., 2., -5.);
            let c = Point(-5., 0., -5.);
            let triangle = Triangle(a, b, c);
            let center = &a + &(&(&(&b - &a) / 3.0_f64) + &(&(&c - &a) / 3.0_f64));
            let normale = triangle.normale(&center).unwrap();
            assert!((normale.norm() - 1.0).abs() < FLOAT_THRESHOLD);
            assert!((normale.scalar_product(&(&a - &b))).abs() < FLOAT_THRESHOLD);
            assert!((normale.scalar_product(&(&a - &c))).abs() < FLOAT_THRESHOLD);
            assert!((normale.scalar_product(&(&b - &c))).abs() < FLOAT_THRESHOLD);
        }

        #[test]
        #[should_panic]
        fn normale_to_degenerate_triangle() {
            let a = Point(4., 2., 6.);
            let b = Point(-3., 2., -5.);
            let c = &a + &(0.4_f64 * &(&a - &b));
            let triangle = Triangle(a, b, c);
            assert_eq!(triangle.normale(&a), None);
        }
    }
}
