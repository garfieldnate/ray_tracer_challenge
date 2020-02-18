use crate::color::Color;
use crate::matrix::Matrix;
use crate::pattern::pattern::BasePattern;
use crate::pattern::pattern::Pattern;
use crate::tuple::Tuple;

#[derive(Clone, Debug, PartialEq)]
pub struct Checkers {
    base: BasePattern,
    a: Color,
    b: Color,
}

impl Checkers {
    pub fn new(a: Color, b: Color) -> Checkers {
        Checkers {
            base: BasePattern::new(),
            a,
            b,
        }
    }
}

impl Pattern for Checkers {
    fn color_at_world(&self, world_point: Tuple) -> Color {
        // TODO: is any kind of overflow possible here?
        if (world_point.x.abs() + world_point.y.abs() + world_point.z.abs()).floor() as i32 % 2 == 0
        {
            self.a
        } else {
            self.b
        }
    }
    fn set_transformation(&mut self, t: Matrix) {
        self.base.set_transformation(t)
    }
    fn transformation_inverse(&self) -> &Matrix {
        self.base.transformation_inverse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn black() -> Color {
        color!(0, 0, 0)
    }

    fn white() -> Color {
        color!(1, 1, 1)
    }

    #[test]
    fn checkers_repeat_in_x() {
        let pattern = Checkers::new(white(), black());
        assert_eq!(pattern.color_at_world(point!(0, 0, 0)), white());
        assert_eq!(pattern.color_at_world(point!(0.99, 0, 0)), white());
        assert_eq!(pattern.color_at_world(point!(1.01, 0, 0)), black());
    }

    #[test]
    fn checkers_repeat_in_y() {
        let pattern = Checkers::new(white(), black());
        assert_eq!(pattern.color_at_world(point!(0, 0, 0)), white());
        assert_eq!(pattern.color_at_world(point!(0, 0.99, 0)), white());
        assert_eq!(pattern.color_at_world(point!(0, 1.01, 0)), black());
    }

    #[test]
    fn checkers_repeat_in_z() {
        let pattern = Checkers::new(white(), black());
        assert_eq!(pattern.color_at_world(point!(0, 0, 0)), white());
        assert_eq!(pattern.color_at_world(point!(0, 0, 0.99)), white());
        assert_eq!(pattern.color_at_world(point!(0, 0, 1.01)), black());
    }
}
