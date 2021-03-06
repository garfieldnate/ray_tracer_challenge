use crate::color::Color;
use crate::material::Material;

pub const DEFAULT_RAY_RECURSION_DEPTH: i16 = 5;

pub const REFRACTION_VACCUM: f32 = 1.0;
pub const REFRACTION_AIR: f32 = 1.00029;
pub const REFRACTION_WATER: f32 = 1.333;
pub const REFRACTION_GLASS: f32 = 1.52;
pub const REFRACTION_DIAMOND: f32 = 2.417;

pub fn glass() -> Material {
    Material::builder()
        .transparency(1.)
        .refractive_index(REFRACTION_GLASS)
        .build()
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
pub fn cyan() -> Color {
    color!(0, 1, 1)
}
pub fn purple() -> Color {
    color!(1, 0, 1)
}
pub fn gray() -> Color {
    color!(0.5, 0.5, 0.5)
}
pub fn brown() -> Color {
    color!(1, 0.5, 0)
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
