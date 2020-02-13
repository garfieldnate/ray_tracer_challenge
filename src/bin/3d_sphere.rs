// Produce image of (squished) sphere's silhouette
use ray_tracer_challenge::canvas::build_canvas;
use ray_tracer_challenge::color::build_color;
use ray_tracer_challenge::light::build_point_light;
use ray_tracer_challenge::light::phong_lighting;
use ray_tracer_challenge::material::build_material;
use ray_tracer_challenge::point;
use ray_tracer_challenge::ray::build_ray;
use ray_tracer_challenge::ray::build_sphere;
use ray_tracer_challenge::ray::Intersection;
use ray_tracer_challenge::transformations::scaling;
use ray_tracer_challenge::transformations::shearing;
use ray_tracer_challenge::tuple::build_tuple;

fn main() {
	let ray_origin = point!(0, 0, -5);
	let wall_z = 10.0;
	let wall_size = 7.0;
	let canvas_pixels = 100;
	let pixel_size = wall_size / canvas_pixels as f32;
	let half = wall_size / 2.0;
	let mut canvas = build_canvas(canvas_pixels, canvas_pixels);
	// red
	// let color = build_color(1.0, 0.0, 0.0);
	let mut material = build_material();
	material.color = build_color(1.0, 0.2, 1.0);
	let mut shape = build_sphere();
	shape.set_material(material);
	let light = build_point_light(point!(-10, 10, -10), build_color(1.0, 1.0, 1.0));

	shape.set_transform(&shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0) * &scaling(0.5, 1.0, 1.0));
	// for each row of pixels in the canvas
	for y in 0..canvas_pixels - 1 {
		let world_y = half - pixel_size * y as f32;
		// for each pixel in the row
		for x in 0..canvas_pixels - 1 {
			// spans from -half to half
			let world_x = -half + pixel_size * x as f32;
			let target = point!(world_x, world_y, wall_z);
			let ray_direction = (target - ray_origin).norm();
			let r = build_ray(ray_origin, ray_direction);
			let xs = shape.intersect(r);
			match Intersection::hit(&xs) {
				Some(hit) => {
					let hit_point = r.position(hit.distance);
					let normal = hit.object.normal_at(hit_point);
					let eye = -ray_direction;
					let color = phong_lighting(hit.object.material, light, hit_point, eye, normal);
					canvas.write_pixel(x, y, color)
				}
				None => {}
			}
		}
	}
	println!("{}", canvas.to_ppm());
}
