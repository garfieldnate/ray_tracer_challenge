use crate::material::default_material;
use crate::material::Material;
use crate::matrix::identity_4x4;
use crate::matrix::Matrix;
use crate::tuple::{build_tuple, Tuple};
use std::cmp::Ordering::Equal;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Ray {
	origin: Tuple,
	direction: Tuple,
}

pub fn build_ray(origin: Tuple, direction: Tuple) -> Ray {
	debug_assert!(origin.is_point());
	debug_assert!(direction.is_vector());
	Ray { origin, direction }
}

impl Ray {
	pub fn position(&self, distance: f32) -> Tuple {
		self.origin + self.direction * distance
	}
	pub fn transform(&self, transform_matrix: &Matrix) -> Ray {
		build_ray(
			transform_matrix * &self.origin,
			transform_matrix * &self.direction,
		)
	}
	// derivation: think of a rhombus shape sitting on point on the surface, with the
	// bottom left and right sides being the incoming and reflected vectors and
	// the surface normal pointing through the middle of the rhombus.
	// To find the reflected vector from the incoming vector, project
	// the incoming vector onto the surface normal, then double its height to get the
	// the top point of the rhombus. Finally, subtract the incoming vector from this
	// point to get the left side of the rhombus, or the reflected vector.
	// This gives us 2 * projection * normal - incoming. The sign needs to be flipped
	// to get the reflection direction right, though, so we have
	// incoming - 2 * projection * normal.
	pub fn reflect(in_vector: Tuple, normal_vector: Tuple) -> Tuple {
		-(normal_vector * 2.0 * in_vector.dot(normal_vector) - in_vector)
	}
}

#[derive(Clone, Debug, PartialEq)]
pub struct Sphere {
	center: Tuple,
	transform: Matrix,
	pub material: Material,
}

pub fn default_sphere() -> Sphere {
	Sphere {
		center: point!(0, 0, 0),
		transform: identity_4x4(),
		material: default_material(),
	}
}

pub fn build_sphere(transform: Matrix, material: Material) -> Sphere {
	Sphere {
		center: point!(0, 0, 0),
		transform,
		material,
	}
}

impl Sphere {
	pub fn set_transform(&mut self, transform_matrix: Matrix) {
		self.transform = transform_matrix;
	}
	pub fn set_material(&mut self, m: Material) {
		self.material = m;
	}
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Intersection<'a> {
	pub distance: f32,
	pub object: &'a Sphere,
}

fn build_intersection<'a>(distance: f32, object: &'a Sphere) -> Intersection<'a> {
	Intersection { distance, object }
}

impl Intersection<'_> {
	// returns the a reference to the intersection with the lowest non-negative distance (or None if all are negative)
	pub fn hit<'a>(intersections: &'a Vec<Intersection<'a>>) -> Option<&'a Intersection<'a>> {
		// TODO: kind of annoying that we have to dereference to get a reference here
		intersections
			.iter()
			.filter(|i| i.distance >= 0.0)
			.min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap_or(Equal))
		// .map(|i| *i)
	}
}

impl Sphere {
	pub fn intersect(&self, ray: Ray) -> Vec<Intersection> {
		let transformed_ray = ray.transform(&self.transform.inverse());
		// ​# the vector from the sphere's center to the ray origin​
		let sphere_to_ray = transformed_ray.origin - self.center;
		// println!("sphere to ray: {:?}", sphere_to_ray);
		let a = transformed_ray.direction.dot(transformed_ray.direction);
		// println!("a: {}", a);
		let b = 2.0 * transformed_ray.direction.dot(sphere_to_ray);
		// println!("b: {}", b);
		let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
		// println!("c: {}", c);
		let discriminant = b.powf(2.0) - 4.0 * a * c;
		// println!("discriminant: {}", discriminant);
		if discriminant < 0.0 {
			return vec![];
		}

		// Jingle bells!
		vec![
			build_intersection((-b - discriminant.sqrt()) / (2.0 * a), self),
			build_intersection((-b + discriminant.sqrt()) / (2.0 * a), self),
		]
	}
	pub fn normal_at(&self, world_point: Tuple) -> Tuple {
		let object_point = &self.transform.inverse() * &world_point;
		let object_normal = object_point - self.center;
		// TODO: why?
		let mut world_normal = &self.transform.inverse().transpose() * &object_normal;
		// transpose of translation matrix will mess with w; manually setting it back
		// to 0 here is faster and simpler than avoiding the computation by taking the
		// 3x3 submatrix before the computation.
		world_normal.w = 0.0;
		world_normal.norm()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::matrix::identity_4x4;
	use crate::transformations::rotation_z;
	use crate::transformations::scaling;
	use crate::transformations::translation;
	use crate::tuple::build_tuple;
	use std::f32::consts::FRAC_1_SQRT_2;
	use std::f32::consts::PI;

	fn frac_1_sqrt_3() -> f32 {
		1.0 / (3f32.sqrt())
	}

	#[test]
	fn basic_ray_creation() {
		let origin = point!(1, 2, 3);
		let direction = vector!(4, 5, 6);
		let r = build_ray(origin, direction);
		assert_eq!(r.origin, origin);
		assert_eq!(r.direction, direction);
	}

	#[test]
	fn compute_point_from_distance() {
		let r = build_ray(point!(2, 3, 4), vector!(1, 0, 0));
		assert_eq!(r.position(0.0), point!(2, 3, 4));
		assert_eq!(r.position(1.0), point!(3, 3, 4));
		assert_eq!(r.position(-1.0), point!(1, 3, 4));
		assert_eq!(r.position(2.5), point!(4.5, 3, 4));
	}

	#[test]
	fn ray_intersects_sphere_at_two_points() {
		let r = build_ray(point!(0, 0, -5), vector!(0, 0, 1));
		let s = default_sphere();
		let intersections = s.intersect(r);
		assert_eq!(intersections.len(), 2);
		assert_eq!(intersections[0].distance, 4.0);
		assert_eq!(intersections[1].distance, 6.0);
	}

	#[test]
	fn ray_intersects_sphere_at_tangent() {
		let r = build_ray(point!(0, 1, -5), vector!(0, 0, 1));
		let s = default_sphere();
		let intersections = s.intersect(r);
		assert_eq!(intersections.len(), 2);
		assert_eq!(intersections[0].distance, 5.0);
		assert_eq!(intersections[1].distance, 5.0);
	}

	#[test]
	fn ray_misses_sphere() {
		let r = build_ray(point!(0, 2, -5), vector!(0, 0, 1));
		let s = default_sphere();
		let intersections = s.intersect(r);
		assert!(intersections.is_empty());
	}

	#[test]
	fn ray_originates_inside_sphere() {
		let r = build_ray(point!(0, 0, 0), vector!(0, 0, 1));
		let s = default_sphere();
		let intersections = s.intersect(r);
		assert_eq!(intersections.len(), 2);
		assert_eq!(intersections[0].distance, -1.0);
		assert_eq!(intersections[1].distance, 1.0);
	}

	#[test]
	fn sphere_is_behind_ray() {
		let r = build_ray(point!(0, 0, 5), vector!(0, 0, 1));
		let s = default_sphere();
		let intersections = s.intersect(r);
		assert_eq!(intersections.len(), 2);
		assert_eq!(intersections[0].distance, -6.0);
		assert_eq!(intersections[1].distance, -4.0);
	}

	#[test]
	fn basic_intersection_creation() {
		let s = default_sphere();
		let i = build_intersection(1.0, &s);
		assert_eq!(i.distance, 1.0);
		assert_eq!(&s as *const _, i.object as *const _);
	}

	#[test]
	fn hit_all_intersections_have_positive_distance() {
		let s = default_sphere();
		let i1 = Intersection {
			distance: 1.0,
			object: &s,
		};
		let i2 = Intersection {
			distance: 2.0,
			object: &s,
		};

		let intersections = vec![i1, i2];
		let i = Intersection::hit(&intersections).unwrap();
		assert_eq!(i, &i1);
	}

	#[test]
	fn hit_some_interactions_have_negative_distance() {
		let s = default_sphere();
		let i1 = Intersection {
			distance: -1.0,
			object: &s,
		};
		let i2 = Intersection {
			distance: 1.0,
			object: &s,
		};
		let i3 = Intersection {
			distance: -0.5,
			object: &s,
		};
		let interactions = vec![i1, i2, i3];
		let i = Intersection::hit(&interactions).unwrap();
		assert_eq!(&i2, i);
	}

	#[test]
	fn no_hit_when_all_interactions_negative() {
		let s = default_sphere();
		let i1 = Intersection {
			distance: -2.0,
			object: &s,
		};
		let i2 = Intersection {
			distance: -1.0,
			object: &s,
		};
		let i3 = Intersection {
			distance: -0.5,
			object: &s,
		};
		let interactions = vec![i1, i2, i3];
		let i = Intersection::hit(&interactions);
		assert!(i.is_none());
	}

	#[test]
	fn hit_is_lowest_nonnegative_intersection() {
		let s = default_sphere();
		let i1 = Intersection {
			distance: 5.0,
			object: &s,
		};
		let i2 = Intersection {
			distance: 7.0,
			object: &s,
		};
		let i3 = Intersection {
			distance: -3.0,
			object: &s,
		};
		let i4 = Intersection {
			distance: 2.0,
			object: &s,
		};
		let interactions = vec![i1, i2, i3, i4];
		let i = Intersection::hit(&interactions).unwrap();
		assert_eq!(&i4, i);
	}

	#[test]
	fn ray_translation() {
		let r = build_ray(point!(1, 2, 3), vector!(0, 1, 0));
		let m = translation(3.0, 4.0, 5.0);
		let r2 = r.transform(&m);
		assert_eq!(r2.origin, point!(4, 6, 8));
		assert_eq!(r2.direction, vector!(0, 1, 0));
	}

	#[test]
	fn ray_scaling() {
		let r = build_ray(point!(1, 2, 3), vector!(0, 1, 0));
		let m = scaling(2.0, 3.0, 4.0);
		let r2 = r.transform(&m);
		assert_eq!(r2.origin, point!(2, 6, 12));
		assert_eq!(r2.direction, vector!(0, 3, 0));
	}

	#[test]
	fn sphere_default_values() {
		let s = default_sphere();
		assert_eq!(s.transform, identity_4x4());
		assert_eq!(s.material, default_material());
	}

	#[test]
	fn set_sphere_values() {
		let mut s = default_sphere();
		let t = translation(2.0, 3.0, 4.0);
		let mut m = default_material();
		m.ambient = 1.0;
		s.set_transform(t.clone());
		s.set_material(m);
		assert_eq!(s.transform, t);
		assert_eq!(s.material, m);
	}

	#[test]
	fn intersect_scaled_sphere_with_ray() {
		let r = build_ray(point!(0, 0, -5), vector!(0, 0, 1));
		let mut s = default_sphere();
		s.set_transform(scaling(2.0, 2.0, 2.0));
		let xs = s.intersect(r);
		assert_eq!(xs[0].distance, 3.0);
		assert_eq!(xs[1].distance, 7.0);
	}

	#[test]
	fn intersect_translated_sphere_with_ray() {
		let r = build_ray(point!(0, 0, -5), vector!(0, 0, 1));
		let mut s = default_sphere();
		s.set_transform(translation(5.0, 0.0, 0.0));
		let xs = s.intersect(r);
		assert_eq!(xs.len(), 0);
	}

	#[test]
	fn sphere_normal_on_x_axis() {
		let s = default_sphere();
		let n = s.normal_at(point!(1, 0, 0));
		assert_eq!(n, vector!(1, 0, 0));
	}

	#[test]
	fn sphere_normal_on_y_axis() {
		let s = default_sphere();
		let n = s.normal_at(point!(0, 1, 0));
		assert_eq!(n, vector!(0, 1, 0));
	}

	#[test]
	fn sphere_normal_on_z_axis() {
		let s = default_sphere();
		let n = s.normal_at(point!(0, 0, 1));
		assert_eq!(n, vector!(0, 0, 1));
	}

	#[test]
	fn sphere_normal_on_nonaxial_point() {
		let s = default_sphere();
		let n = s.normal_at(point!(frac_1_sqrt_3(), frac_1_sqrt_3(), frac_1_sqrt_3()));
		assert_abs_diff_eq!(
			n,
			vector!(frac_1_sqrt_3(), frac_1_sqrt_3(), frac_1_sqrt_3())
		);
	}

	#[test]
	fn normal_is_normalized_vector() {
		let s = default_sphere();
		let n = s.normal_at(point!(frac_1_sqrt_3(), frac_1_sqrt_3(), frac_1_sqrt_3()));
		assert_abs_diff_eq!(n, n.norm());
	}

	#[test]
	fn normal_of_translated_sphere() {
		let mut s = default_sphere();
		s.set_transform(translation(0.0, 1.0, 0.0));
		let n = s.normal_at(point!(0, 1.70711, -0.70711));
		assert_abs_diff_eq!(n, vector!(0, 0.7071068, -0.70710677));
	}

	#[test]
	fn normal_of_transformed_sphere() {
		let mut s = default_sphere();
		let m = &scaling(1.0, 0.5, 1.0) * &rotation_z(PI / 5.0);
		s.set_transform(m);
		let n = s.normal_at(point!(0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2));
		assert_abs_diff_eq!(n, vector!(0, 0.97014254, -0.24253564));
	}

	#[test]
	fn reflect_vector_approaching_at_45_degrees() {
		let v = vector!(1, -1, 0);
		let n = vector!(0, 1, 0);
		let r = Ray::reflect(v, n);
		assert_eq!(r, vector!(1, 1, 0));
	}

	#[test]
	fn reflect_vector_off_slanted_surface() {
		let v = vector!(0, -1, 0);
		let n = vector!(FRAC_1_SQRT_2, FRAC_1_SQRT_2, 0);
		let r = Ray::reflect(v, n);
		assert_abs_diff_eq!(r, vector!(1, 0, 0));
	}
}
