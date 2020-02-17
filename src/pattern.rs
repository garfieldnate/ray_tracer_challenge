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

#[derive(Clone, Debug, PartialEq)]
pub struct Stripes {
    a: Color,
    b: Color,
    ti: Matrix,
}

impl Stripes {
    pub fn new(a: Color, b: Color) -> Stripes {
        Stripes {
            a,
            b,
            ti: identity_4x4(),
        }
    }
}

impl Pattern for Stripes {
    fn set_transformation(&mut self, t: Matrix) {
        self.ti = t.inverse();
    }
    fn transformation_inverse(&self) -> &Matrix {
        &self.ti
    }
    fn color_at_world(&self, world_point: Tuple) -> Color {
        if world_point.x.floor() as i32 % 2 == 0 {
            self.a
        } else {
            self.b
        }
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

    #[test]
    fn stripe_pattern_constructor() {
        let pattern = Stripes::new(white(), black());
        assert_eq!(pattern.a, white());
        assert_eq!(pattern.b, black());
    }

    #[test]
    fn stripe_pattern_is_constant_in_y() {
        let pattern = Stripes::new(white(), black());
        assert_eq!(pattern.color_at_world(point!(0, 0, 0)), white());
        assert_eq!(pattern.color_at_world(point!(0, 1, 0)), white());
        assert_eq!(pattern.color_at_world(point!(0, 2, 0)), white());
    }

    #[test]
    fn stripe_pattern_is_constant_in_z() {
        let pattern = Stripes::new(white(), black());
        assert_eq!(pattern.color_at_world(point!(0, 0, 0)), white());
        assert_eq!(pattern.color_at_world(point!(0, 0, 1)), white());
        assert_eq!(pattern.color_at_world(point!(0, 0, 2)), white());
    }

    #[test]
    fn stripe_pattern_alternates_in_x() {
        let pattern = Stripes::new(white(), black());
        assert_eq!(pattern.color_at_world(point!(0, 0, 0)), white());
        assert_eq!(pattern.color_at_world(point!(0.9, 0, 0)), white());
        assert_eq!(pattern.color_at_world(point!(1, 0, 0)), black());
        assert_eq!(pattern.color_at_world(point!(-0.1, 0, 0)), black());
        assert_eq!(pattern.color_at_world(point!(-1, 0, 0)), black());
        assert_eq!(pattern.color_at_world(point!(-1.1, 0, 0)), white());
    }

    #[test]
    fn stripes_with_an_object_transformation() {
        let object = Sphere::build(scaling(2.0, 2.0, 2.0), default_material());
        let pattern = Stripes::new(white(), black());
        let c = pattern.color_at_object(point!(1.5, 0, 0), &object);
        assert_eq!(c, white());
    }

    #[test]
    fn stripes_with_a_pattern_transformation() {
        let object = Sphere::new();
        let mut pattern = Stripes::new(white(), black());
        pattern.set_transformation(scaling(2.0, 2.0, 2.0));
        let c = pattern.color_at_object(point!(1.5, 0, 0), &object);
        assert_eq!(c, white());
    }

    #[test]
    fn stripes_with_both_object_and_pattern_transformation() {
        let object = Sphere::build(scaling(2.0, 2.0, 2.0), default_material());
        let mut pattern = Stripes::new(white(), black());
        pattern.set_transformation(translation(0.5, 0.0, 0.0));
        let c = pattern.color_at_object(point!(1.5, 0, 0), &object);
        assert_eq!(c, white());
    }
}
