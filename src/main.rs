mod geometry;
use self::geometry::Point;
mod camera;
mod image;
mod scene;

fn main() {
    let filename = "scene.obj";
    let scene = scene::Scene::from_file(filename);
    let cam = camera::Camera::new(
        Point::new(0.0, 0.0, 1.0),
        Point::new(1.0, 1.0, 0.0),
        0.0,
        256, // Dots per centimeter.
        1.0,
        (2.0, 2.0),
    );
    let mut image = image::LinearImage::new(cam.resolution().0, cam.resolution().1);
    for (position, ray) in cam.generate_rays() {
        let (object, point) = scene.intersects(&ray).unwrap();
        image.set_pixel(position.0, position.1, &object.material().diffuse);
        // TODO Step 1: implement phong model to take in account the light sources.
        // TODO Step 2: implement reflections of the rays.
        // TODO Step 3: implement refractions of the rays.
        // TODO Step 4: add support for textures.
        // TODO Step 5: Generate several rays for a single pixel and take average color. (?)
    }
    let image = image.convert_to_rgb();
    image.write_to("output.png").unwrap();
}
