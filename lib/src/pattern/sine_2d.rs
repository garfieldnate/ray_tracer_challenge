use crate::color::Color;
use crate::constants::black;
use crate::constants::white;
use crate::pattern::pattern::BasePattern;
use crate::pattern::pattern::Pattern;
use crate::tuple::Tuple;

#[derive(Clone, Debug, PartialEq)]
pub struct Sine2D {
    base: BasePattern,
    a: Color,
    distance: Color,
}

impl Sine2D {
    pub fn new(a: Color, b: Color) -> Sine2D {
        let distance = b - a;
        Sine2D {
            base: BasePattern::new(),
            a,
            distance,
        }
    }
}

impl Default for Sine2D {
    fn default() -> Self {
        Self::new(white(), black())
    }
}

impl Pattern for Sine2D {
    fn get_base(&self) -> &BasePattern {
        &self.base
    }
    fn get_base_mut(&mut self) -> &mut BasePattern {
        &mut self.base
    }
    fn color_at_world(&self, world_point: Tuple) -> Color {
        let cosine = (world_point.x + world_point.z).cos();
        // cosine is in [1, -1], but we need a fraction in [0, 1]
        let fraction = (-cosine + 1.0) / 2.0;
        self.a + (self.distance * fraction)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::f32::consts::PI;

    #[test]
    fn sine_2d_pattern_is_constant_in_y() {
        let pattern = Sine2D::default();
        assert_eq!(pattern.color_at_world(point!(0, 0, 0)), white());
        assert_eq!(pattern.color_at_world(point!(0, 1, 0)), white());
        assert_eq!(pattern.color_at_world(point!(0, 2, 0)), white());
    }

    #[test]
    fn sine_2d_pattern_varies_in_z() {
        let pattern = Sine2D::default();
        assert_eq!(pattern.color_at_world(point!(0, 0, 0)), white());
        assert_abs_diff_eq!(
            pattern.color_at_world(point!(0, 0, 1)),
            color!(0.770_151_14, 0.770_151_14, 0.770_151_14)
        );
        assert_eq!(
            pattern.color_at_world(point!(0, 0, 2)),
            color!(0.291_926_56, 0.291_926_56, 0.291_926_56)
        );
        assert_eq!(pattern.color_at_world(point!(0, 0, PI)), color!(0, 0, 0));
    }
}
