use crate::color::build_color;
use crate::color::Color;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
}

pub fn build_material() -> Material {
    Material {
        color: build_color(1.0, 1.0, 1.0),
        ambient: 0.1,
        diffuse: 0.9,
        specular: 0.9,
        shininess: 200.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn default_material_attributes() {
        let m = build_material();
        assert_eq!(m.color, build_color(1.0, 1.0, 1.0));
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
    }
}
