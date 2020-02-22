use crate::color::Color;
use crate::constants::white;
use crate::intersection::Intersection;
use crate::light::phong_lighting;
use crate::light::PointLight;
use crate::material::Material;
use crate::matrix::identity_4x4;
use crate::ray::Ray;
use crate::shape::shape::Shape;
use crate::shape::sphere::Sphere;
use crate::transformations::scaling;
use crate::tuple::Tuple;
use std::cmp::Ordering::Equal;
use std::f32;

// TODO: book said no light by default, but that seems weird. We always have a light, otherwise we can't see anything! Plus using Option complicates/makes dangerous everything.
pub struct World {
	pub objects: Vec<Box<dyn Shape>>,
	pub light: Option<PointLight>,
}

impl World {
	pub fn new() -> World {
		World {
			objects: vec![],
			light: Option::None,
		}
	}
}

impl Default for World {
	fn default() -> Self {
		let mut m = Material::default();
		m.color = color!(0.8, 1.0, 0.6);
		m.diffuse = 0.7;
		m.specular = 0.2;
		let s1 = Sphere::build(identity_4x4(), m);
		let s2 = Sphere::build(scaling(0.5, 0.5, 0.5), Material::default());
		World {
			objects: vec![Box::new(s1), Box::new(s2)],
			light: Some(PointLight::new(point!(-10.0, 10.0, -10.0), white())),
		}
	}
}

impl World {
	pub fn intersect(&self, r: Ray) -> Vec<Intersection> {
		let mut intersections: Vec<Intersection> = (&self.objects)
			.iter()
			.map(|o| o.intersect(r))
			.flatten()
			.collect();
		intersections.sort_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap_or(Equal));
		intersections
	}

	pub fn shade_hit(&self, comps: PrecomputedValues, remaining_recursive_steps: i16) -> Color {
		let surface_color = phong_lighting(
			comps.object,
			comps.object.material(),
			self.light.unwrap(),
			comps.over_point,
			comps.eye_vector,
			comps.surface_normal,
			self.is_shadowed(comps.over_point),
		);
		let reflected_color = self.reflected_color(comps, remaining_recursive_steps);
		surface_color + reflected_color
	}

	pub fn color_at(&self, r: Ray, remaining_recursive_steps: i16) -> Color {
		let intersections = self.intersect(r);
		if intersections.is_empty() {
			color!(0, 0, 0)
		} else {
			match Intersection::hit(&intersections) {
				Some(hit) => {
					let comps = precompute_values(r, hit, &intersections);
					self.shade_hit(comps, remaining_recursive_steps)
				}
				None => color!(0, 0, 0),
			}
		}
	}

	pub fn is_shadowed(&self, point: Tuple) -> bool {
		// create a ray from a point to the light
		// if there's an intersection between the light and the point, then the point is in shadow
		let light_to_point_vector = self.light.unwrap().position - point;
		let distance = light_to_point_vector.magnitude();
		let direction = light_to_point_vector.norm();

		let r = Ray::new(point, direction);
		let intersections = self.intersect(r);

		let hit = Intersection::hit(&intersections);
		match hit {
			Some(i) => i.distance < distance,
			None => false,
		}
	}

	pub fn reflected_color(
		&self,
		comps: PrecomputedValues,
		remaining_recursive_steps: i16,
	) -> Color {
		if comps.object.material().reflective == 0.0 || remaining_recursive_steps < 1 {
			color!(0, 0, 0)
		} else {
			let reflected_ray = Ray::new(comps.over_point, comps.reflection_vector);
			let c = self.color_at(reflected_ray, remaining_recursive_steps - 1);
			c * comps.object.material().reflective
		}
	}
}

pub struct PrecomputedValues<'a> {
	distance: f32,
	object: &'a dyn Shape,
	point: Tuple,
	eye_vector: Tuple,
	reflection_vector: Tuple,
	surface_normal: Tuple,
	inside: bool,
	// a point a tiny distance above the surface to avoid self-shadowing/salt-and-pepper noise, caused
	// by finite precision in floating point calculations
	over_point: Tuple,

	pub n1: f32,
	pub n2: f32,
}
const SELF_SHADOW_AVOIDANCE_EPSILON: f32 = f32::EPSILON * 10000.0;

pub fn precompute_values<'a>(
	r: Ray,
	hit: &Intersection<'a>,
	intersections: &Vec<Intersection<'a>>,
) -> PrecomputedValues<'a> {
	let point = r.position(hit.distance);
	let mut surface_normal = hit.object.normal_at(point);
	let eye_vector = -r.direction;
	let reflection_vector = Ray::reflect(r.direction, surface_normal);

	let inside;
	if surface_normal.dot(eye_vector) < 0.0 {
		// surface and eye are pointed in opposite directions, so the hit must be inside
		inside = true;
		surface_normal = -surface_normal;
	} else {
		inside = false;
	}

	let over_point = point + surface_normal * SELF_SHADOW_AVOIDANCE_EPSILON;
	// println!("point: {:?}, over_point:{:?}", point, over_point);

	PrecomputedValues {
		// copy the intersection's properties, for convenience
		distance: hit.distance,
		object: hit.object,
		// precompute some useful values
		point,
		eye_vector,
		reflection_vector,
		surface_normal,
		inside,
		over_point,

		n1: f32::NAN,
		n2: f32::NAN,
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::shape::plane::Plane;
	use crate::transformations::translation;
	use std::f32::consts::FRAC_1_SQRT_2;
	use std::f32::consts::SQRT_2;

	#[test]
	fn create_blank_world() {
		let w = World::new();
		assert!(w.objects.is_empty());
		assert!(w.light.is_none());
	}

	#[test]
	fn intersect_world_with_ray() {
		let w = World::default();
		let r = Ray::new(point!(0, 0, -5), vector!(0, 0, 1));
		let xs = w.intersect(r);
		assert_eq!(xs.len(), 4);
		assert_eq!(xs[0].distance, 4.0);
		assert_eq!(xs[1].distance, 4.5);
		assert_eq!(xs[2].distance, 5.5);
		assert_eq!(xs[3].distance, 6.0);
	}

	#[test]
	fn precompute_intersection_state() {
		let r = Ray::new(point!(0, 0, -5), vector!(0, 0, 1));
		let shape = Sphere::new();
		let i = Intersection::new(4.0, &shape);
		let comps = precompute_values(r, &i, &vec![i]);
		assert_eq!(comps.distance, i.distance);
		assert_eq!(comps.point, point!(0, 0, -1));
		assert_eq!(comps.eye_vector, vector!(0, 0, -1));
		assert_eq!(comps.surface_normal, vector!(0, 0, -1));
	}

	#[test]
	fn precompute_hit_occurs_outside() {
		let r = Ray::new(point!(0, 0, -5), vector!(0, 0, 1));
		let shape = Sphere::new();
		let i = Intersection::new(4.0, &shape);
		let comps = precompute_values(r, &i, &vec![i]);
		assert!(!comps.inside);
	}

	#[test]
	fn precompute_hit_occurs_inside() {
		let r = Ray::new(point!(0, 0, 0), vector!(0, 0, 1));
		let shape = Sphere::new();
		let i = Intersection::new(1.0, &shape);
		let comps = precompute_values(r, &i, &vec![i]);
		assert_eq!(comps.point, point!(0, 0, 1));
		assert_eq!(comps.eye_vector, vector!(0, 0, -1));
		assert_eq!(comps.inside, true);
		assert_eq!(
			comps.surface_normal,
			vector!(0, 0, -1),
			"Surface normal should be inverted because hit is inside shape"
		);
	}

	#[test]
	fn precompute_reflection_vector() {
		let shape = Plane::new();
		let r = Ray::new(point!(0, 1, -1), vector!(0, -FRAC_1_SQRT_2, FRAC_1_SQRT_2));
		let i = Intersection::new(SQRT_2, &shape);
		let comps = precompute_values(r, &i, &vec![i]);
		assert_eq!(
			comps.reflection_vector,
			vector!(0, FRAC_1_SQRT_2, FRAC_1_SQRT_2)
		);
	}

	#[test]
	fn reflected_color_for_nonreflective_material() {
		let mut w = World::default();
		w.objects[1].material().ambient = 1.0;
		let r = Ray::new(point!(0, 0, 0), vector!(0, 0, 1));
		let i = Intersection::new(1.0, w.objects[1].as_ref());
		let comps = precompute_values(r, &i, &vec![i]);
		let color = w.reflected_color(comps, 1);
		assert_eq!(color, color!(0, 0, 0));
	}

	#[test]
	fn reflected_color_for_reflective_material() {
		let mut w = World::default();
		let mut m = Material::default();
		m.reflective = 0.5;
		let plane = Box::new(Plane::build(translation(0.0, -1.0, 0.0), m));
		w.objects.push(plane);

		let r = Ray::new(point!(0, 0, -3), vector!(0, -FRAC_1_SQRT_2, FRAC_1_SQRT_2));
		let i = Intersection::new(SQRT_2, w.objects.last().unwrap().as_ref());
		let comps = precompute_values(r, &i, &vec![i]);
		let color = w.reflected_color(comps, 1);
		assert_abs_diff_eq!(color, color!(0.19052197, 0.23815246, 0.14289148));
	}

	#[test]
	fn shade_hit_with_reflective_material() {
		let mut w = World::default();
		let mut m = Material::default();
		m.reflective = 0.5;
		let plane = Box::new(Plane::build(translation(0.0, -1.0, 0.0), m));
		w.objects.push(plane);

		let r = Ray::new(point!(0, 0, -3), vector!(0, -FRAC_1_SQRT_2, FRAC_1_SQRT_2));
		let i = Intersection::new(SQRT_2, w.objects.last().unwrap().as_ref());
		let comps = precompute_values(r, &i, &vec![i]);
		let color = w.shade_hit(comps, 1);
		assert_abs_diff_eq!(color, color!(0.8769108, 0.9245413, 0.8292803));
	}

	#[test]
	fn shade_hit_with_mutually_reflective_surfaces() {
		let mut w = World::new();
		w.light = Some(PointLight::new(point!(0, 0, 0), color!(0, 0, 0)));
		let mut m = Material::default();
		m.reflective = 1.0;
		let lower = Plane::build(translation(0.0, -1.0, 0.0), m.clone());
		let upper = Plane::build(translation(0.0, 1.0, 0.0), m.clone());
		w.objects.push(Box::new(lower));
		w.objects.push(Box::new(upper));

		let r = Ray::new(point!(0, 0, 0), vector!(0, 1, 0));
		// just testing that this terminates without blowing the stack
		w.color_at(r, 1);
	}

	#[test]
	fn reflected_color_at_max_recursive_depth() {
		let mut w = World::default();
		let mut m = Material::default();
		m.reflective = 0.5;
		let plane = Box::new(Plane::build(translation(0.0, -1.0, 0.0), m));
		w.objects.push(plane);

		let r = Ray::new(point!(0, 0, -3), vector!(0, -FRAC_1_SQRT_2, FRAC_1_SQRT_2));
		let i = Intersection::new(SQRT_2, w.objects.last().unwrap().as_ref());
		let comps = precompute_values(r, &i, &vec![i]);
		let color = w.reflected_color(comps, 0);
		assert_abs_diff_eq!(color, color!(0, 0, 0));
	}

	#[test]
	fn shade_intersection() {
		let w = World::default();
		let r = Ray::new(point!(0, 0, -5), vector!(0, 0, 1));
		let shape = &w.objects[0];
		let i = Intersection::new(4.0, shape.as_ref());
		let comps = precompute_values(r, &i, &vec![i]);
		let c = w.shade_hit(comps, 1);
		assert_abs_diff_eq!(c, color!(0.38063288, 0.47579104, 0.28547466))
	}

	#[test]
	fn shade_intersection_from_inside() {
		let mut w = World::default();
		w.light = Some(PointLight::new(point!(0, 0.25, 0), white()));
		let r = Ray::new(point!(0, 0, 0), vector!(0, 0, 1));
		let shape = &w.objects[1];
		let i = Intersection::new(0.5, shape.as_ref());
		let comps = precompute_values(r, &i, &vec![i]);
		let c = w.shade_hit(comps, 1);
		assert_abs_diff_eq!(c, color!(0.9045995, 0.9045995, 0.9045995))
	}

	#[test]
	fn color_when_ray_misses() {
		let w = World::default();
		let r = Ray::new(point!(0, 0, -5), vector!(0, 1, 0));
		let c = w.color_at(r, 1);
		assert_eq!(c, color!(0, 0, 0));
	}

	#[test]
	fn color_when_ray_hits() {
		let w = World::default();
		let r = Ray::new(point!(0, 0, -5), vector!(0, 0, 1));
		let c = w.color_at(r, 1);
		assert_abs_diff_eq!(c, color!(0.38063288, 0.47579104, 0.28547466))
	}

	#[test]
	fn color_when_intersection_behind_ray() {
		let mut w = World::default();
		// TODO: can't take w.objects[x] and mutate it...
		// outer
		let mut material = Material::default();
		material.ambient = 1.0;
		w.objects[0].set_material(material.clone());
		// inner
		w.objects[1].set_material(material.clone());
		let r = Ray::new(point!(0, 0, 0.75), vector!(0, 0, -1));
		let c = w.color_at(r, 1);
		assert_eq!(c, w.objects[1].material().color);
	}

	#[test]
	fn no_shadow_when_nothing_is_colinear_with_point_and_light() {
		let w = World::default();
		let p = point!(0, 10, 0);
		assert_eq!(w.is_shadowed(p), false);
	}

	#[test]
	fn no_shadow_when_object_is_between_point_and_light() {
		let w = World::default();
		let p = point!(10, -10, 10);
		assert_eq!(w.is_shadowed(p), true);
	}

	#[test]
	fn no_shadow_when_object_is_behind_light() {
		let w = World::default();
		let p = point!(-20, 20, -20);
		assert_eq!(w.is_shadowed(p), false);
	}

	#[test]
	fn no_shadow_when_object_is_behind_point() {
		let w = World::default();
		let p = point!(-2, 2, -2);
		assert_eq!(w.is_shadowed(p), false);
	}

	#[test]
	fn hit_should_offset_point_for_shadow_calculations() {
		let r = Ray::new(point!(0, 0, -5), vector!(0, 0, 1));
		let shape = Sphere::build(translation(0.0, 0.0, 1.0), Material::default());
		let intersection = Intersection::new(5.0, &shape);
		let comps = precompute_values(r, &intersection, &vec![intersection]);
		// println!("{:?}", comps.point);
		// println!("{:?}", comps.over_point);
		assert!(comps.over_point.z < -SELF_SHADOW_AVOIDANCE_EPSILON / 2.0);
		assert!(comps.over_point.z > -SELF_SHADOW_AVOIDANCE_EPSILON * 2.0);
		assert!(comps.point.z > comps.over_point.z);
	}

	#[test]
	fn shade_hit_for_intersection_in_shadow() {
		let mut w = World::new();
		w.light = Some(PointLight::new(point!(0, 0, -10), white()));
		let s1 = Sphere::new();
		let s2 = Sphere::build(translation(0.0, 0.0, 10.0), Material::default());
		w.objects.push(Box::new(s1));
		w.objects.push(Box::new(s2));
		let r = Ray::new(point!(0, 0, 5), vector!(0, 0, 1));
		let i = Intersection::new(4.0, w.objects[1].as_ref());
		let comps = precompute_values(r, &i, &vec![i]);
		let c = w.shade_hit(comps, 1);
		assert_eq!(c, color!(0.1, 0.1, 0.1));
	}
}
