use approx::AbsDiffEq;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

// TODO: allow changing datatypes to f64?
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

pub fn build_color(r: f32, g: f32, b: f32) -> Color {
    Color { r: r, g: g, b: b }
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

impl Mul<Color> for f32 {
    type Output = Color;
    fn mul(self, color: Color) -> Color {
        color * self
    }
}

// Hadamard product for mixing two colors
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

// required for approximate comparisons due to use of floating point numbers
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

#[cfg(test)]
mod tests {
    use super::*;
    // TODO: lots of fails because of lack of approximate equality

    #[test]
    fn test_adding_colors() {
        let c1 = build_color(0.9, 0.6, 0.75);
        let c2 = build_color(0.7, 0.1, 0.25);
        abs_diff_eq!(c1 + c2, build_color(1.6, 0.7, 1.0));
    }

    #[test]
    fn test_subtracting_colors() {
        let c1 = build_color(0.9, 0.6, 0.75);
        let c2 = build_color(0.7, 0.1, 0.25);
        abs_diff_eq!(c1 - c2, build_color(0.2, 0.5, 0.5));
    }

    #[test]
    fn test_multiply_color_by_scalar() {
        let c = build_color(0.2, 0.3, 0.4);
        abs_diff_eq!(c * 2.0, 2.0 * c);
        abs_diff_eq!(c * 2.0, build_color(0.4, 0.6, 0.8));
    }
    #[test]
    fn test_mix_colors_by_multiplication() {
        let c1 = build_color(1.0, 0.2, 0.4);
        let c2 = build_color(0.9, 1.0, 0.1);
        abs_diff_eq!(c1 * c2, build_color(0.9, 0.2, 0.04));
    }
}
