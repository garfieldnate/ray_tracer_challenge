use crate::color::Color;
use crate::constants::black;
use crate::constants::white;
use crate::pattern::pattern::BasePattern;
use crate::pattern::pattern::Pattern;
use crate::tuple::Tuple;

#[derive(Clone, Debug, PartialEq)]
pub struct Gradient {
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
	fn get_base(&self) -> &BasePattern {
		&self.base
	}
	fn get_base_mut(&mut self) -> &mut BasePattern {
		&mut self.base
	}
	fn color_at_world(&self, world_point: Tuple) -> Color {
		let fraction = world_point.x - world_point.x.floor();
		self.a + (self.distance * fraction)
	}
}

impl Default for Gradient {
	fn default() -> Self {
		Self::new(white(), black())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn gradient_linearly_interpolates_between_colors() {
		let pattern = Gradient::default();
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
