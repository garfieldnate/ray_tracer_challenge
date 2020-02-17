use crate::color::Color;
use crate::matrix::Matrix;
use crate::pattern::pattern::BasePattern;
use crate::pattern::pattern::Pattern;
use crate::tuple::Tuple;

#[derive(Clone, Debug, PartialEq)]
struct Rings {
    base: BasePattern,
    a: Color,
    b: Color,
}

impl Rings {
    pub fn new(a: Color, b: Color) -> Rings {
        Rings {
            base: BasePattern::new(),
            a,
            b,
        }
    }
}

impl Pattern for Rings {
    fn color_at_world(&self, world_point: Tuple) -> Color {
        // TODO: is any kind of overflow possible here?
        if (world_point.x.powi(2) + world_point.z.powi(2))
            .sqrt()
            .floor() as i32
            % 2
            == 0
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
    fn rings_extend_in_both_x_and_z() {
        let pattern = Rings::new(white(), black());
        assert_eq!(pattern.color_at_world(point!(0, 0, 0)), white());
        assert_eq!(pattern.color_at_world(point!(1, 0, 0)), black());
        assert_eq!(pattern.color_at_world(point!(0, 0, 1)), black());
        // 0.708 = just slightly more than √2/2​
        assert_eq!(pattern.color_at_world(point!(0.708, 0, 0.708)), black());
    }
}
