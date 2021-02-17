struct Point(f64, f64, f64);
struct Vector(f64, f64, f64);

// struct Triangle ??tree points?
// struct Ray ?Two points, point and a vector??

impl Point {
    fn Distance(&self, other: &Point) -> f64 {
        panic!("Not implemented!");
    }

    fn VectorTo(&self, endpoint: &Point) -> Vector {
        panic!("Not implemented!");
    }

    fn ShiftBy(&self, sift: &Vector) -> Point {

    }
}

impl Vector {
    fn Add(&self, other: &Vector) -> Point {
        panic!("Not implemented!");
    }

    fn Norm(&self) -> f64 {
        panic!("Not implemented!");
    }

    fn MultiplyBy(&self, scalar: f64) {
        panic!("Not implemented!");
    }

    fn ScalarProduct(&self, other: &Vector) -> f64 {
        panic!("Not implemented!");
    }

    fn VectorProduct(&self, other: &Vector) -> Vector {
        panic!("Not implemented!");
    }

    fn Normalize(&mut self) {
        panic!("Not implemented!");
    }

    fn Rotate(&self, axis: &Vector, angle: f64) -> Vector {
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