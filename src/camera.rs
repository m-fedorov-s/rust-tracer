use crate::geometry::{Point, Ray, Vector};

pub struct Camera {
    from: Point,
    to: Vector,
    camera_up: Vector,
    camera_side: Vector,
    resolution: (usize, usize),
    focal_length: f32, // in centimeters
    canvas_size: (f32, f32), // in centimeters
}

// We implement an iterator to write
// for i in camera.generate_rays()
// Here we specify that the life time of iterator is the same as the lifetime of the camera. (It is `'a`).
pub struct CameraIter<'a> {
    camera: &'a Camera,
    position: (usize, usize),
}

// iterator should have `next` method.
impl<'a> Iterator for CameraIter<'a> {
    type Item = ((usize, usize), Ray);

    fn next(&mut self) -> Option<Self::Item> {
        self.position.0 += 1;
        self.position.1 += self.position.0 / self.camera.resolution.0;
        if self.position.1 == self.camera.resolution.1 {
            return None;
        }
        self.position.0 %= self.camera.resolution.0;
        let pixel_size = (self.camera.canvas_size.0 / self.camera.resolution.0 as f32) as f64;
        let shift = self
            .camera
            .camera_up
            .multiply_by(
                (self.position.0 as f64 - self.camera.resolution.0 as f64 / 2.0) * pixel_size,
            )
            .add(&self.camera.camera_side.multiply_by(
                (self.position.1 as f64 - self.camera.resolution.1 as f64 / 2.0) * pixel_size,
            ));
        // let ray = Ray::from_to(self.camera.from, self.camera.to.multiply_by(self.camera.focal_length) + shift);
        // return Some(self.position, Ray())
        panic!("Not implemented!");
    }
}

impl Camera {
    pub fn new(
        from: Point,
        to: Point,
        rotation: f64,
        dpcm: u32,
        focal_length: f32,
        canvas_size: (f32, f32),
    ) -> Camera {
        let to_vector = (from - &to).normalize();
        let resolution = (
            (canvas_size.0 * dpcm as f32).round() as usize,
            (canvas_size.1 * dpcm as f32).round() as usize,
        );
        let side_vector = to_vector
            .vector_product(&Vector::new(0.0, 0.0, 1.0))
            .normalize();
        let up_vector = to_vector.vector_product(&side_vector);
        // Now rotate `side_vector` and `up_vector` around `to_vector` by `rotation`.
        let side_vector = side_vector.rotate(&to_vector, rotation);
        let up_vector = up_vector.rotate(&to_vector, rotation);
        Camera {
            from,
            to: to_vector,
            camera_up: up_vector,
            camera_side: side_vector,
            resolution,
            focal_length,
            canvas_size,
        }
    }

    pub fn generate_rays(&self) -> CameraIter {
        CameraIter {
            camera: self,
            position: (0, 0),
        }
    }

    pub fn resolution(&self) -> (usize, usize) {
        self.resolution
    }
}
