use crate::color::Color;
use crate::constants::black;
use crate::constants::white;
use crate::pattern::pattern::BasePattern;
use crate::pattern::pattern::Pattern;
use crate::tuple::Tuple;

#[derive(Clone, Debug, PartialEq)]
pub struct UVCheckers {
    base: BasePattern,
    a: Color,
    b: Color,
    width: f32,
    height: f32,
}

impl UVCheckers {
    pub fn new(width: f32, height: f32, a: Color, b: Color) -> UVCheckers {
        UVCheckers {
            base: BasePattern::new(),
            a,
            b,
            width,
            height,
        }
    }
    pub fn uv_pattern_at(&self, u: f32, v: f32) -> Color {
        let u2 = (u * self.width).floor() as i32;
        let v2 = (v * self.height).floor() as i32;

        if (u2 + v2) % 2 == 0 {
            self.a
        } else {
            self.b
        }
    }
}

impl Pattern for UVCheckers {
    fn get_base(&self) -> &BasePattern {
        &self.base
    }
    fn get_base_mut(&mut self) -> &mut BasePattern {
        &mut self.base
    }
    fn color_at_world(&self, world_point: Tuple) -> Color {
        // TODO
        black()
    }
}

impl Default for UVCheckers {
    fn default() -> Self {
        Self::new(1., 1., white(), black())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pattern_creation() {
        let p = UVCheckers::new(2., 2., black(), white());
        let test_data = vec![
            ("1", 0.0, 0.0, black()),
            ("2", 0.5, 0.0, white()),
            ("3", 0.0, 0.5, white()),
            ("4", 0.5, 0.5, black()),
            ("5", 1.0, 1.0, black()),
        ];
        for (name, u, v, expected_color) in test_data {
            assert_eq!(p.uv_pattern_at(u, v), expected_color, "Case {}", name);
        }
    }
}
