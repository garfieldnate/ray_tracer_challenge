use crate::color::Color;
use crate::matrix::Matrix;
use crate::shape::shape::Shape;
use crate::tuple::Tuple;
use dyn_clone::DynClone;
use std::fmt::Debug;

pub trait Pattern: Debug + DynClone {
	// tthe BasePattern that the wrapping instance is delegating to
	fn get_base(&self) -> &BasePattern;
	fn get_base_mut(&mut self) -> &mut BasePattern;
	fn color_at_world(&self, object_point: Tuple) -> Color;

	// don't override these
	fn color_at_object(&self, world_point: Tuple, object: &dyn Shape) -> Color {
		let object_point = object.transformation_inverse() * &world_point;
		let pattern_point = self.transformation_inverse() * &object_point;
		self.color_at_world(pattern_point)
	}
	fn set_transformation(&mut self, t: Matrix) {
		self.get_base_mut().set_transformation(t)
	}
	fn transformation_inverse(&self) -> &Matrix {
		self.get_base().transformation_inverse()
	}
}

dyn_clone::clone_trait_object!(Pattern);

// Other pattern implementations should delegate to this one where these defaults are acceptable.
#[derive(Default, Clone, Debug, PartialEq)]
pub struct BasePattern {
	t_inverse: Matrix,
}

impl BasePattern {
	pub fn new() -> Self {
		Self::default()
	}
}

impl Pattern for BasePattern {
	// these two are unimplemented because BasePattern is not meant to be instantiated by itself
	fn get_base(&self) -> &BasePattern {
		unimplemented!()
	}

	fn get_base_mut(&mut self) -> &mut BasePattern {
		unimplemented!()
	}

	fn set_transformation(&mut self, t: Matrix) {
		self.t_inverse = t.inverse();
	}

	fn transformation_inverse(&self) -> &Matrix {
		&self.t_inverse
	}

	// These methods cannot be delegated to
	fn color_at_world(&self, _object_point: Tuple) -> Color {
		unimplemented!()
	}
}

#[derive(Clone, Debug, PartialEq)]
pub struct TestPattern {
	base: BasePattern,
}

impl TestPattern {
	pub fn new() -> Self {
		TestPattern {
			base: BasePattern::new(),
		}
	}
}

impl Pattern for TestPattern {
	fn get_base(&self) -> &BasePattern {
		&self.base
	}
	fn get_base_mut(&mut self) -> &mut BasePattern {
		&mut self.base
	}
	// color value will allow client to test that world_point was transformed
	fn color_at_world(&self, world_point: Tuple) -> Color {
		color!(world_point.x, world_point.y, world_point.z)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::material::Material;
	use crate::shape::sphere::Sphere;
	use crate::transformations::scaling;
	use crate::transformations::translation;

	#[test]
	fn pattern_with_object_transformation() {
		let object = Sphere::build(scaling(2.0, 2.0, 2.0), Material::default());
		let test_pattern = TestPattern::new();
		let c = test_pattern.color_at_object(point!(2, 3, 4), &object);
		assert_eq!(c, color!(1, 1.5, 2));
	}

	#[test]
	fn pattern_with_pattern_transformation() {
		let object = Sphere::new();
		let mut test_pattern = TestPattern::new();
		test_pattern.set_transformation(scaling(2.0, 2.0, 2.0));
		let c = test_pattern.color_at_object(point!(2, 3, 4), &object);
		assert_eq!(c, color!(1, 1.5, 2));
	}

	#[test]
	fn pattern_with_both_object_and_pattern_transformation() {
		let object = Sphere::build(scaling(2.0, 2.0, 2.0), Material::default());
		let mut test_pattern = TestPattern::new();
		test_pattern.set_transformation(translation(0.5, 1.0, 1.5));
		let c = test_pattern.color_at_object(point!(2.5, 3, 3.5), &object);
		assert_eq!(c, color!(0.75, 0.5, 0.25));
	}
}
