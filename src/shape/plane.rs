use crate::intersection::Intersection;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::shape::shape::BaseShape;
use crate::shape::shape::Shape;
use crate::tuple::Tuple;
use std::f32;

#[derive(Clone, Debug, PartialEq)]
pub struct Plane {
	base: BaseShape,
}
impl Plane {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn build(transform: Matrix, material: Material) -> Self {
		let mut s = Plane::new();
		s.set_transformation(transform);
		s.set_material(material);
		s
	}
}

impl Default for Plane {
	fn default() -> Self {
		Plane {
			base: BaseShape::new(),
		}
	}
}

impl Shape for Plane {
	fn local_intersect(&self, object_ray: Ray) -> Vec<Intersection> {
		// the plane is in the xz plane, so it's y is 0.
		// if the ray is roughly coplanar or parallel with the plane,
		// we won't be able to see it
		if object_ray.direction.y.abs() < f32::EPSILON * 10000.0 {
			vec![]
		} else {
			// this formula works because the plain sits in the xz plane
			let distance = -object_ray.origin.y / object_ray.direction.y;
			vec![Intersection::new(distance, self)]
		}
	}
	fn local_norm_at(&self, _object_point: Tuple) -> Tuple {
		vector!(0, 1, 0)
	}

	// forward these to BaseShape (TODO: need delegation RFC to be accepted!)
	fn transformation(&self) -> &Matrix {
		&self.base.transformation()
	}
	fn set_transformation(&mut self, t: Matrix) {
		self.base.set_transformation(t);
	}
	fn material(&self) -> &Material {
		self.base.material()
	}
	fn set_material(&mut self, m: Material) {
		self.base.set_material(m);
	}
	fn transformation_inverse(&self) -> &Matrix {
		self.base.transformation_inverse()
	}
	fn transformation_inverse_transpose(&self) -> &Matrix {
		self.base.transformation_inverse_transpose()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn normal_of_plane_is_constant_everywhere() {
		let p = Plane::new();
		let n1 = p.local_norm_at(point!(0, 0, 0));
		let n2 = p.local_norm_at(point!(10, 0, -10));
		let n3 = p.local_norm_at(point!(-5, 0, 150));
		assert_eq!(n1, vector!(0, 1, 0));
		assert_eq!(n2, vector!(0, 1, 0));
		assert_eq!(n3, vector!(0, 1, 0));
	}

	#[test]
	fn intersect_with_parallel_ray() {
		let p = Plane::new();
		let r = Ray::new(point!(0, 10, 0), vector!(0, 0, 1));
		let xs = p.local_intersect(r);
		assert!(xs.is_empty());
	}

	#[test]
	fn intersect_with_coplanar_ray() {
		let p = Plane::new();
		let r = Ray::new(point!(0, 0, 0), vector!(0, 0, 1));
		let xs = p.local_intersect(r);
		assert!(xs.is_empty());
	}

	#[test]
	fn ray_intersects_plane_from_above() {
		let p = Plane::new();
		let r = Ray::new(point!(0, 1, 0), vector!(0, -1, 0));
		let xs = p.local_intersect(r);
		assert_eq!(xs.len(), 1);
		assert_eq!(xs[0].distance, 1.0);
	}

	#[test]
	fn ray_intersects_plane_from_below() {
		let p = Plane::new();
		let r = Ray::new(point!(0, -1, 0), vector!(0, 1, 0));
		let xs = p.local_intersect(r);
		assert_eq!(xs.len(), 1);
		assert_eq!(xs[0].distance, 1.0);
	}
}
