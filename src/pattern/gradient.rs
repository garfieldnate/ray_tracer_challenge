use crate::color::Color;
use crate::matrix::Matrix;
use crate::pattern::pattern::BasePattern;
use crate::pattern::pattern::Pattern;
use crate::tuple::Tuple;

#[derive(Clone, Debug, PartialEq)]
struct Gradient {
    base: BasePattern,
    a: Color,
    distance: Color,
}

impl Gradient {
    pub fn new(a: Color, b: Color) -> Gradient {
        let distance = b - a;
        Gradient {
            base: BasePattern::new(),
            a,
            distance,
        }
    }
}

impl Pattern for Gradient {
    fn color_at_world(&self, world_point: Tuple) -> Color {
        let fraction = world_point.x - world_point.x.floor();
        self.a + (self.distance * fraction)
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
    fn gradient_linearly_interpolates_between_colors() {
        let pattern = Gradient::new(white(), black());
        assert_eq!(pattern.color_at_world(point!(0, 0, 0)), white());
        assert_eq!(
            pattern.color_at_world(point!(0.25, 0, 0)),
            color!(0.75, 0.75, 0.75)
        );
        assert_eq!(
            pattern.color_at_world(point!(0.5, 0, 0)),
            color!(0.5, 0.5, 0.5)
        );
        assert_eq!(
            pattern.color_at_world(point!(0.75, 0, 0)),
            color!(0.25, 0.25, 0.25)
        );
    }
}
