use crate::color::Color;
use crate::constants::black;
use crate::constants::white;
use crate::pattern::pattern::BasePattern;
use crate::pattern::pattern::Pattern;
use crate::tuple::Tuple;
use std::fmt::Debug;

#[derive(Clone, Debug, PartialEq)]
pub struct Stripes {
    pub a: Color,
    pub b: Color,
    base: BasePattern,
}

impl Stripes {
    pub fn new(a: Color, b: Color) -> Stripes {
        Stripes {
            a,
            b,
            base: BasePattern::new(),
        }
    }
}

impl Default for Stripes {
    fn default() -> Self {
        Self::new(white(), black())
    }
}

impl Pattern for Stripes {
    fn get_base(&self) -> &BasePattern {
        &self.base
    }
    fn get_base_mut(&mut self) -> &mut BasePattern {
        &mut self.base
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

    #[test]
    fn stripe_pattern_constructor() {
        let pattern = Stripes::default();
        assert_eq!(pattern.a, white());
        assert_eq!(pattern.b, black());
    }

    #[test]
    fn stripe_pattern_is_constant_in_y() {
        let pattern = Stripes::default();
        assert_eq!(pattern.color_at_world(point!(0, 0, 0)), white());
        assert_eq!(pattern.color_at_world(point!(0, 1, 0)), white());
        assert_eq!(pattern.color_at_world(point!(0, 2, 0)), white());
    }

    #[test]
    fn stripe_pattern_is_constant_in_z() {
        let pattern = Stripes::default();
        assert_eq!(pattern.color_at_world(point!(0, 0, 0)), white());
        assert_eq!(pattern.color_at_world(point!(0, 0, 1)), white());
        assert_eq!(pattern.color_at_world(point!(0, 0, 2)), white());
    }

    #[test]
    fn stripe_pattern_alternates_in_x() {
        let pattern = Stripes::default();
        assert_eq!(pattern.color_at_world(point!(0, 0, 0)), white());
        assert_eq!(pattern.color_at_world(point!(0.9, 0, 0)), white());
        assert_eq!(pattern.color_at_world(point!(1, 0, 0)), black());
        assert_eq!(pattern.color_at_world(point!(-0.1, 0, 0)), black());
        assert_eq!(pattern.color_at_world(point!(-1, 0, 0)), black());
        assert_eq!(pattern.color_at_world(point!(-1.1, 0, 0)), white());
    }
}
