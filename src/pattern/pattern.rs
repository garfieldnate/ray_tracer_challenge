use crate::color::Color;
use crate::matrix::identity_4x4;
use crate::matrix::Matrix;
use crate::shape::shape::Shape;
use crate::tuple::Tuple;
use dyn_clone::DynClone;
use std::fmt::Debug;

pub trait Pattern: Debug + DynClone {
    // don't override this one
    fn color_at_object(&self, world_point: Tuple, object: &dyn Shape) -> Color {
        let object_point = object.transformation_inverse() * &world_point;
        let pattern_point = self.transformation_inverse() * &object_point;
        self.color_at_world(pattern_point)
    }
    fn color_at_world(&self, world_point: Tuple) -> Color;
    // fn transformation(&self) -> &Matrix;
    fn set_transformation(&mut self, t: Matrix);
    fn transformation_inverse(&self) -> &Matrix;
}

dyn_clone::clone_trait_object!(Pattern);

// Other pattern implementations are meant to delegate to this one where these defaults are acceptable.
// TODO: Maybe someday Rust will support delegation: https://github.com/rust-lang/rfcs/pull/2393
// like Kotlin does. Could also use ambassador crate, if it adds partial delegation support.
#[derive(Clone, Debug, PartialEq)]
pub struct BasePattern {
    t_inverse: Matrix,
}

impl BasePattern {
    pub fn new() -> Self {
        BasePattern {
            t_inverse: identity_4x4(),
        }
    }
}

impl Pattern for BasePattern {
    fn set_transformation(&mut self, t: Matrix) {
        self.t_inverse = t.inverse();
    }

    fn transformation_inverse(&self) -> &Matrix {
        &self.t_inverse
    }

    // These methods cannot be delegated to
    fn color_at_world(&self, _world_point: Tuple) -> Color {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::material::default_material;
    use crate::shape::sphere::Sphere;
    use crate::transformations::scaling;
    use crate::transformations::translation;

    fn black() -> Color {
        color!(0, 0, 0)
    }

    fn white() -> Color {
        color!(1, 1, 1)
    }

    #[derive(Clone, Debug, PartialEq)]
    struct TestPattern {
        base: BasePattern,
    }

    impl TestPattern {
        fn new() -> Self {
            TestPattern {
                base: BasePattern::new(),
            }
        }
    }

    impl Pattern for TestPattern {
        fn set_transformation(&mut self, t: Matrix) {
            self.base.set_transformation(t);
        }
        fn transformation_inverse(&self) -> &Matrix {
            self.base.transformation_inverse()
        }
        // color value will allow client to test that world_point was transformed
        fn color_at_world(&self, world_point: Tuple) -> Color {
            color!(world_point.x, world_point.y, world_point.z)
        }
    }

    #[test]
    fn pattern_with_object_transformation() {
        let object = Sphere::build(scaling(2.0, 2.0, 2.0), default_material());
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
        let object = Sphere::build(scaling(2.0, 2.0, 2.0), default_material());
        let mut test_pattern = TestPattern::new();
        test_pattern.set_transformation(translation(0.5, 1.0, 1.5));
        let c = test_pattern.color_at_object(point!(2.5, 3, 3.5), &object);
        assert_eq!(c, color!(0.75, 0.5, 0.25));
    }
}
