use approx::AbsDiffEq;
use std::ops::{Add, Div, Mul, Sub};
use std::str::FromStr;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Color {
	pub r: f32,
	pub g: f32,
	pub b: f32,
}

impl Color {
	pub fn new(r: f32, g: f32, b: f32) -> Color {
		Color { r, g, b }
	}
}

// use like this: color!(1,1,0.5)
#[macro_export]
macro_rules! color {
	($x:expr, $y:expr, $z:expr) => {{
		Color::new($x as f32, $y as f32, $z as f32)
		}};
}

impl Add for Color {
	type Output = Color;
	fn add(self, other: Color) -> Color {
		Color {
			r: self.r + other.r,
			g: self.g + other.g,
			b: self.b + other.b,
		}
	}
}

impl Sub for Color {
	type Output = Color;
	fn sub(self, other: Color) -> Color {
		Color {
			r: self.r - other.r,
			g: self.g - other.g,
			b: self.b - other.b,
		}
	}
}

// scalar multiplication (commutative)
impl Mul<f32> for Color {
	type Output = Color;
	fn mul(self, scalar: f32) -> Color {
		Color {
			r: self.r * scalar,
			g: self.g * scalar,
			b: self.b * scalar,
		}
	}
}

// scalar multiplication (commutative)
impl Mul<Color> for f32 {
	type Output = Color;
	fn mul(self, color: Color) -> Color {
		color * self
	}
}

// scalar division
impl Div<f32> for Color {
	type Output = Color;
	fn div(self, scalar: f32) -> Color {
		Color {
			r: self.r / scalar,
			g: self.g / scalar,
			b: self.b / scalar,
		}
	}
}

// Multiplying two Color objects produces a mix of the two colors using the Hadamard product
impl Mul<Color> for Color {
	type Output = Color;
	fn mul(self, other: Color) -> Color {
		Color {
			r: self.r * other.r,
			g: self.g * other.g,
			b: self.b * other.b,
		}
	}
}

// required for equality tests because floating point numbers must be compared approximately
impl AbsDiffEq for Color {
	type Epsilon = f32;

	fn default_epsilon() -> Self::Epsilon {
		f32::default_epsilon()
	}

	fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
		f32::abs_diff_eq(&self.r, &other.r, epsilon)
			&& f32::abs_diff_eq(&self.g, &other.g, epsilon)
			&& f32::abs_diff_eq(&self.b, &other.b, epsilon)
	}
}

impl FromStr for Color {
	type Err = std::num::ParseIntError;

	// Parses a color hex code of the form '#rRgGbB..'
	fn from_str(hex_code: &str) -> Result<Self, Self::Err> {
		let r: u8 = u8::from_str_radix(&hex_code[1..3], 16)?;
		let g: u8 = u8::from_str_radix(&hex_code[3..5], 16)?;
		let b: u8 = u8::from_str_radix(&hex_code[5..7], 16)?;

		Ok(Color::new(
			r as f32 / 255.0,
			g as f32 / 255.0,
			b as f32 / 255.0,
		))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_adding_colors() {
		let c1 = color!(0.9, 0.6, 0.75);
		let c2 = color!(0.7, 0.1, 0.25);
		assert_abs_diff_eq!(c1 + c2, color!(1.6, 0.7, 1));
	}

	#[test]
	fn test_subtracting_colors() {
		let c1 = color!(0.9, 0.6, 0.75);
		let c2 = color!(0.7, 0.1, 0.25);
		assert_abs_diff_eq!(c1 - c2, color!(0.2, 0.5, 0.5));
	}

	#[test]
	fn test_multiply_color_by_scalar() {
		let c = color!(0.2, 0.3, 0.4);
		assert_abs_diff_eq!(c * 2.0, 2.0 * c);
		assert_abs_diff_eq!(c * 2.0, color!(0.4, 0.6, 0.8));
	}

	#[test]
	fn test_mix_colors_by_multiplication() {
		let c1 = color!(1.0, 0.2, 0.4);
		let c2 = color!(0.9, 1.0, 0.1);
		assert_abs_diff_eq!(c1 * c2, color!(0.9, 0.2, 0.04));
	}

	#[test]
	fn test_parse_hex() {
		let c = Color::from_str("#0ab33f").unwrap();
		println!("{:?}", c);
		assert_abs_diff_eq!(c, color!(0.039_215_688, 0.701_960_8, 0.247_058_82));
	}
}
