use crate::color::Color;
use crate::constants::black;
use crate::constants::white;
use crate::pattern::pattern::BasePattern;
use crate::pattern::pattern::Pattern;
use crate::tuple::Tuple;

#[derive(Clone, Debug, PartialEq)]
pub struct Rings {
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

impl Default for Rings {
    fn default() -> Self {
        Self::new(white(), black())
    }
}

impl Pattern for Rings {
    fn get_base(&self) -> &BasePattern {
        &self.base
    }
    fn get_base_mut(&mut self) -> &mut BasePattern {
        &mut self.base
    }
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rings_extend_in_both_x_and_z() {
        let pattern = Rings::new(white(), black());
        assert_eq!(pattern.color_at_world(point!(0, 0, 0)), white());
        assert_eq!(pattern.color_at_world(point!(1, 0, 0)), black());
        assert_eq!(pattern.color_at_world(point!(0, 0, 1)), black());
        // 0.708 = just slightly more than √2/2
        assert_eq!(pattern.color_at_world(point!(0.708, 0, 0.708)), black());
    }
}
