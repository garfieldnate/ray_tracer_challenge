// Produce image of (squished) sphere's silhouette
use ray_tracer_challenge::canvas::build_canvas;
use ray_tracer_challenge::color::Color;
use ray_tracer_challenge::ray::Intersection;
use ray_tracer_challenge::ray::Ray;
use ray_tracer_challenge::shape::shape::Shape;
use ray_tracer_challenge::shape::sphere::Sphere;
use ray_tracer_challenge::transformations::scaling;
use ray_tracer_challenge::transformations::shearing;
use ray_tracer_challenge::tuple::Tuple;
use ray_tracer_challenge::{color, point};

fn main() {
    let ray_origin = point!(0, 0, -5);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let canvas_pixels = 100;
    let pixel_size = wall_size / canvas_pixels as f32;
    let half = wall_size / 2.0;
    let mut canvas = build_canvas(canvas_pixels, canvas_pixels);
    // red
    let color = color!(1, 0, 0);
    let mut shape = Sphere::new();
    shape.set_transformation(&shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0) * &scaling(0.5, 1.0, 1.0));
    // for each row of pixels in the canvas
    for y in 0..canvas_pixels - 1 {
        let world_y = half - pixel_size * y as f32;
        // for each pixel in the row
        for x in 0..canvas_pixels - 1 {
            // spans from -half to half
            let world_x = -half + pixel_size * x as f32;
            let target = point!(world_x, world_y, wall_z);
            let r = Ray::new(ray_origin, target - ray_origin);
            let xs = shape.intersect(r);
            match Intersection::hit(&xs) {
                Some(_) => canvas.write_pixel(x, y, color),
                None => {}
            }
        }
    }
    println!("{}", canvas.to_ppm());
}
