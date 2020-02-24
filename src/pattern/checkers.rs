use crate::color::Color;
use crate::constants::black;
use crate::constants::white;
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

impl Default for Checkers {
	fn default() -> Self {
		Self::new(white(), black())
	}
}

impl Pattern for Checkers {
	fn get_base(&self) -> &BasePattern {
		&self.base
	}
	fn get_base_mut(&mut self) -> &mut BasePattern {
		&mut self.base
	}
	fn color_at_world(&self, world_point: Tuple) -> Color {
		// TODO: is any kind of overflow possible here?
		if (world_point.x.abs() + world_point.y.abs() + world_point.z.abs()).floor() as i32 % 2 == 0
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
	fn checkers_repeat_in_x() {
		let pattern = Checkers::default();
		assert_eq!(pattern.color_at_world(point!(0, 0, 0)), white());
		assert_eq!(pattern.color_at_world(point!(0.99, 0, 0)), white());
		assert_eq!(pattern.color_at_world(point!(1.01, 0, 0)), black());
	}

	#[test]
	fn checkers_repeat_in_y() {
		let pattern = Checkers::default();
		assert_eq!(pattern.color_at_world(point!(0, 0, 0)), white());
		assert_eq!(pattern.color_at_world(point!(0, 0.99, 0)), white());
		assert_eq!(pattern.color_at_world(point!(0, 1.01, 0)), black());
	}

	#[test]
	fn checkers_repeat_in_z() {
		let pattern = Checkers::default();
		assert_eq!(pattern.color_at_world(point!(0, 0, 0)), white());
		assert_eq!(pattern.color_at_world(point!(0, 0, 0.99)), white());
		assert_eq!(pattern.color_at_world(point!(0, 0, 1.01)), black());
	}
}
