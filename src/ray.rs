use crate::matrix::Matrix;
use crate::shape::shape::Shape;
use crate::tuple::Tuple;
use std::cmp::Ordering::Equal;
use std::ptr;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Ray {
	pub origin: Tuple,
	pub direction: Tuple,
}

impl Ray {
	pub fn new(origin: Tuple, direction: Tuple) -> Self {
		debug_assert!(origin.is_point());
		debug_assert!(direction.is_vector());
		Ray { origin, direction }
	}
	pub fn position(&self, distance: f32) -> Tuple {
		self.origin + self.direction * distance
	}
	pub fn transform(&self, transform_matrix: &Matrix) -> Ray {
		Self::new(
			transform_matrix * &self.origin,
			transform_matrix * &self.direction,
		)
	}
	// derivation: think of a rhombus shape sitting on point on the surface, with the
	// bottom left and right sides being the incoming and reflected vectors and
	// the surface normal pointing to the middle of the rhombus.
	// To find the reflected vector from the incoming vector, project
	// the incoming vector onto the surface normal, then double the resulting vector's height to get the
	// the top point of the rhombus. Finally, subtract the incoming vector from this top
	// point to get the left side of the rhombus, or the reflected vector.
	// This gives us 2 * projection * normal - incoming. The sign needs to be flipped
	// to get the reflection direction right, though, so we have
	// incoming - 2 * projection * normal.
	pub fn reflect(in_vector: Tuple, normal_vector: Tuple) -> Tuple {
		-(normal_vector * 2.0 * in_vector.dot(normal_vector) - in_vector)
	}
}

type BoxedShape<'a> = &'a dyn Shape<'a>;
#[derive(Copy, Clone, Debug)]
pub struct Intersection<'a> {
	pub distance: f32,
	pub object: BoxedShape<'a>,
}

impl<'a, 'b> PartialEq<Intersection<'b>> for Intersection<'a> {
	fn eq(&self, other: &Intersection<'b>) -> bool {
		self.distance.eq(&other.distance) && ptr::eq(self.object, other.object)
	}
}

impl Intersection<'_> {
	pub fn new<'a>(distance: f32, object: BoxedShape<'a>) -> Intersection<'a> {
		Intersection { distance, object }
	}
	// returns the a reference to the intersection with the lowest non-negative distance (or None if all are negative)
	pub fn hit<'a>(intersections: &'a Vec<Intersection<'a>>) -> Option<&'a Intersection<'a>> {
		intersections
			.iter()
			.filter(|i| i.distance >= 0.0)
			.min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap_or(Equal))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::shape::shape::Shape;
	use crate::shape::sphere::Sphere;
	use crate::transformations::scaling;
	use crate::transformations::translation;
	use std::f32::consts::FRAC_1_SQRT_2;

	#[test]
	fn basic_ray_creation() {
		let origin = point!(1, 2, 3);
		let direction = vector!(4, 5, 6);
		let r = Ray::new(origin, direction);
		assert_eq!(r.origin, origin);
		assert_eq!(r.direction, direction);
	}

	#[test]
	fn compute_point_from_distance() {
		let r = Ray::new(point!(2, 3, 4), vector!(1, 0, 0));
		assert_eq!(r.position(0.0), point!(2, 3, 4));
		assert_eq!(r.position(1.0), point!(3, 3, 4));
		assert_eq!(r.position(-1.0), point!(1, 3, 4));
		assert_eq!(r.position(2.5), point!(4.5, 3, 4));
	}

	#[test]
	fn ray_intersects_sphere_at_two_points() {
		let r = Ray::new(point!(0, 0, -5), vector!(0, 0, 1));
		let s = Sphere::new();
		let intersections = s.intersect(r);
		assert_eq!(intersections.len(), 2);
		assert_eq!(intersections[0].distance, 4.0);
		assert_eq!(intersections[1].distance, 6.0);
	}

	#[test]
	fn ray_intersects_sphere_at_tangent() {
		let r = Ray::new(point!(0, 1, -5), vector!(0, 0, 1));
		let s = Sphere::new();
		let intersections = s.intersect(r);
		assert_eq!(intersections.len(), 2);
		assert_eq!(intersections[0].distance, 5.0);
		assert_eq!(intersections[1].distance, 5.0);
	}

	#[test]
	fn ray_misses_sphere() {
		let r = Ray::new(point!(0, 2, -5), vector!(0, 0, 1));
		let s = Sphere::new();
		let intersections = s.intersect(r);
		assert!(intersections.is_empty());
	}

	#[test]
	fn ray_originates_inside_sphere() {
		let r = Ray::new(point!(0, 0, 0), vector!(0, 0, 1));
		let s = Sphere::new();
		let intersections = s.intersect(r);
		assert_eq!(intersections.len(), 2);
		assert_eq!(intersections[0].distance, -1.0);
		assert_eq!(intersections[1].distance, 1.0);
	}

	#[test]
	fn sphere_is_behind_ray() {
		let r = Ray::new(point!(0, 0, 5), vector!(0, 0, 1));
		let s = Sphere::new();
		let intersections = s.intersect(r);
		assert_eq!(intersections.len(), 2);
		assert_eq!(intersections[0].distance, -6.0);
		assert_eq!(intersections[1].distance, -4.0);
	}

	#[test]
	fn basic_intersection_creation() {
		let s = Sphere::new();
		let i = Intersection::new(1.0, &s);
		assert_eq!(i.distance, 1.0);
		assert!(ptr::eq(&s as &dyn Shape, i.object as &dyn Shape));
	}

	#[test]
	fn hit_all_intersections_have_positive_distance() {
		let s = Sphere::new();
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
		let s = Sphere::new();
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
		let s = Sphere::new();
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
		let s = Sphere::new();
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
		let r = Ray::new(point!(1, 2, 3), vector!(0, 1, 0));
		let m = translation(3.0, 4.0, 5.0);
		let r2 = r.transform(&m);
		assert_eq!(r2.origin, point!(4, 6, 8));
		assert_eq!(r2.direction, vector!(0, 1, 0));
	}

	#[test]
	fn ray_scaling() {
		let r = Ray::new(point!(1, 2, 3), vector!(0, 1, 0));
		let m = scaling(2.0, 3.0, 4.0);
		let r2 = r.transform(&m);
		assert_eq!(r2.origin, point!(2, 6, 12));
		assert_eq!(r2.direction, vector!(0, 3, 0));
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
