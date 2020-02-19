// Produce image of (squished) sphere's silhouette
use ray_tracer_challenge::canvas::Canvas;
use ray_tracer_challenge::color::Color;
use ray_tracer_challenge::light::phong_lighting;
use ray_tracer_challenge::light::PointLight;
use ray_tracer_challenge::material::default_material;
use ray_tracer_challenge::ray::Intersection;
use ray_tracer_challenge::ray::Ray;
use ray_tracer_challenge::shape::shape::Shape;
use ray_tracer_challenge::shape::sphere::Sphere;

use ray_tracer_challenge::constants::white;
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
	let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
	// let color = red();
	let mut material = default_material();
	material.color = color!(1, 0.2, 1);
	let mut shape = Sphere::new();
	shape.set_material(material);
	let light = PointLight::new(point!(-10, 10, -10), white());

	shape.set_transformation(&shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0) * &scaling(0.5, 1.0, 1.0));
	// for each row of pixels in the canvas
	for y in 0..canvas_pixels - 1 {
		let world_y = half - pixel_size * y as f32;
		// for each pixel in the row
		for x in 0..canvas_pixels - 1 {
			// spans from -half to half
			let world_x = -half + pixel_size * x as f32;
			let target = point!(world_x, world_y, wall_z);
			let ray_direction = (target - ray_origin).norm();
			let r = Ray::new(ray_origin, ray_direction);
			let xs = shape.intersect(r);
			match Intersection::hit(&xs) {
				Some(hit) => {
					let hit_point = r.position(hit.distance);
					let normal = hit.object.normal_at(hit_point);
					let eye = -ray_direction;
					let color = phong_lighting(
						hit.object,
						hit.object.material(),
						light,
						hit_point,
						eye,
						normal,
						false,
					);
					canvas.write_pixel(x, y, color)
				}
				None => {}
			}
		}
	}
	println!("{}", canvas.to_ppm());
}
