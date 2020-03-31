use crate::color::Color;
use crate::matrix::Matrix;
use crate::pattern::pattern::BasePattern;
use crate::pattern::pattern::Pattern;
use crate::shape::shape::Shape;
use crate::tuple::Tuple;

// A solid color with no variation
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Solid {
    color: Color,
}

impl Solid {
    pub fn new(color: Color) -> Solid {
        Solid { color }
    }
}

impl Pattern for Solid {
    // Override these two methods to make processing fast
    fn color_at_object(&self, _world_point: Tuple, _object: &dyn Shape) -> Color {
        self.color
    }
    // these are not needed, and should never be called
    fn transformation_inverse(&self) -> &Matrix {
        unimplemented!()
    }
    fn get_base(&self) -> &BasePattern {
        unimplemented!()
    }
    fn get_base_mut(&mut self) -> &mut BasePattern {
        unimplemented!()
    }
    fn color_at_world(&self, _object_point: Tuple) -> Color {
        unimplemented!()
    }

    // this one can be called but will do nothing
    fn set_transformation(&mut self, _t: Matrix) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::{black, yellow};
    use crate::shape::sphere::Sphere;

    #[test]
    fn solid_pattern_returns_same_color_everywhere() {
        let dummy_shape = Sphere::new();
        let pattern = Solid::default();
        assert_eq!(
            pattern.color_at_object(point!(0, 0, 0), &dummy_shape),
            black()
        );
        assert_eq!(
            pattern.color_at_object(point!(100, -100, 20), &dummy_shape),
            black()
        );
        assert_eq!(
            pattern.color_at_object(point!(1, 1_000, 1_000_000), &dummy_shape),
            black()
        );

        let pattern = Solid::new(yellow());
        assert_eq!(
            pattern.color_at_object(point!(0, 0, 0), &dummy_shape),
            yellow()
        );
        assert_eq!(
            pattern.color_at_object(point!(100, -100, 20), &dummy_shape),
            yellow()
        );
        assert_eq!(
            pattern.color_at_object(point!(1, 1_000, 1_000_000), &dummy_shape),
            yellow()
        );
    }
}
