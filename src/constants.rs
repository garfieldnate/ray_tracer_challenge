use crate::color::Color;
use crate::material::Material;

pub const REFRACTION_VACCUM: f32 = 1.0;
pub const REFRACTION_AIR: f32 = 1.00029;
pub const REFRACTION_WATER: f32 = 1.333;
pub const REFRACTION_GLASS: f32 = 1.52;
pub const REFRACTION_DIAMOND: f32 = 2.417;

pub fn glass() -> Material {
    let mut m = Material::default();
    m.transparency = 1.0;
    m.refractive_index = REFRACTION_GLASS;
    m
}

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
pub fn yellow() -> Color {
    color!(1, 1, 0)
}
pub fn gray() -> Color {
    color!(0.5, 0.5, 0.5)
}

pub fn metal() -> Material {
    Material {
        color: gray(),
        ambient: 1.0,
        diffuse: 0.6,
        reflective: 0.1,
        specular: 0.4,
        shininess: 10.0,
        transparency: 0.0,
        refractive_index: 1.0,
        pattern: None,
    }
}
