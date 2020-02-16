use crate::color::Color;
use crate::tuple::Tuple;
use dyn_clone::DynClone;
use std::fmt::Debug;

pub trait Pattern: Debug + DynClone {
    fn color_at(&self, p: Tuple) -> Color;
}
dyn_clone::clone_trait_object!(Pattern);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Stripes {
    a: Color,
    b: Color,
}

impl Stripes {
    pub fn new(a: Color, b: Color) -> Stripes {
        Stripes { a, b }
    }
}

impl Pattern for Stripes {
    fn color_at(&self, p: Tuple) -> Color {
        if p.x.floor() as i32 % 2 == 0 {
            self.a
        } else {
            self.b
        }
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
    fn stripe_pattern_constructor() {
        let pattern = Stripes::new(white(), black());
        assert_eq!(pattern.a, white());
        assert_eq!(pattern.b, black());
    }

    #[test]
    fn stripe_pattern_is_constant_in_y() {
        let pattern = Stripes::new(white(), black());
        assert_eq!(pattern.color_at(point!(0, 0, 0)), white());
        assert_eq!(pattern.color_at(point!(0, 1, 0)), white());
        assert_eq!(pattern.color_at(point!(0, 2, 0)), white());
    }

    #[test]
    fn stripe_pattern_is_constant_in_z() {
        let pattern = Stripes::new(white(), black());
        assert_eq!(pattern.color_at(point!(0, 0, 0)), white());
        assert_eq!(pattern.color_at(point!(0, 0, 1)), white());
        assert_eq!(pattern.color_at(point!(0, 0, 2)), white());
    }

    #[test]
    fn stripe_pattern_alternates_in_x() {
        let pattern = Stripes::new(white(), black());
        assert_eq!(pattern.color_at(point!(0, 0, 0)), white());
        assert_eq!(pattern.color_at(point!(0.9, 0, 0)), white());
        assert_eq!(pattern.color_at(point!(1, 0, 0)), black());
        assert_eq!(pattern.color_at(point!(-0.1, 0, 0)), black());
        assert_eq!(pattern.color_at(point!(-1, 0, 0)), black());
        assert_eq!(pattern.color_at(point!(-1.1, 0, 0)), white());
    }
}
