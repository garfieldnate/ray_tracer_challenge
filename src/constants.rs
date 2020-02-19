use crate::color::Color;

pub const REFACTION_VACCUM: f32 = 1.0;
pub const REFACTION_AIR: f32 = 1.00029;
pub const REFACTION_WATER: f32 = 1.333;
pub const REFACTION_GLASS: f32 = 1.52;
pub const REFACTION_DIAMOND: f32 = 2.417;

pub fn white() -> Color {
    color!(1, 1, 1)
}
pub fn black() -> Color {
    color!(0, 0, 0)
}
pub fn red() -> Color {
    color!(1, 0, 0)
}
pub fn green() -> Color {
    color!(0, 1, 0)
}
pub fn blue() -> Color {
    color!(0, 0, 1)
}
