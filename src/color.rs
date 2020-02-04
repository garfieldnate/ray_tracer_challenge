use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Sub;

#[derive(Copy, Clone, Debug, PartialEq)]
struct Color {
    r: f32,
    g: f32,
    b: f32,
}

fn build_color(r: f32, g: f32, b: f32) -> Color {
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
        Color {
            r: color.r * self,
            g: color.g * self,
            b: color.b * self,
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_mix_colors_by_multiplication() {
        let c1 = build_color(1.0, 0.2, 0.4);
        let c2 = build_color(0.9, 1.0, 0.1);
        // TODO: fails because of lack of approximate equality
        assert_eq!(c1 * c2, build_color(0.9, 0.2, 0.04));
    }
}
