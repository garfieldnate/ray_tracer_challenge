use crate::color::Color;
use crate::constants::black;
use crate::constants::white;
use crate::pattern::pattern::BasePattern;
use crate::pattern::pattern::Pattern;
use crate::tuple::Tuple;
use dyn_clone::DynClone;
use std::f32::consts::PI;
use std::fmt::Debug;

pub trait UVPattern: Debug + DynClone {
    fn color_at(&self, u: f32, v: f32) -> Color;
}

dyn_clone::clone_trait_object!(UVPattern);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UVCheckers {
    a: Color,
    b: Color,
    width: f32,
    height: f32,
}

impl UVCheckers {
    pub fn new(width: f32, height: f32, a: Color, b: Color) -> UVCheckers {
        UVCheckers {
            a,
            b,
            width,
            height,
        }
    }
}

impl Default for UVCheckers {
    fn default() -> Self {
        Self::new(1., 1., white(), black())
    }
}

impl UVPattern for UVCheckers {
    fn color_at(&self, u: f32, v: f32) -> Color {
        let u2 = (u * self.width).floor() as i32;
        let v2 = (v * self.height).floor() as i32;

        if (u2 + v2) % 2 == 0 {
            self.a
        } else {
            self.b
        }
    }
}

pub trait UVMapping: Debug + DynClone {
    fn point_to_uv(&self, p: Tuple) -> (f32, f32);
}

dyn_clone::clone_trait_object!(UVMapping);

#[derive(Clone, Debug)]
pub struct TextureMap {
    base: BasePattern,
    uv_pattern: Box<dyn UVPattern>,
    uv_mapping: Box<dyn UVMapping>,
}

impl TextureMap {
    pub fn new(uv_pattern: Box<dyn UVPattern>, uv_mapping: Box<dyn UVMapping>) -> Self {
        Self {
            base: BasePattern::new(),
            uv_pattern,
            uv_mapping,
        }
    }
}

impl Pattern for TextureMap {
    fn get_base(&self) -> &BasePattern {
        &self.base
    }
    fn get_base_mut(&mut self) -> &mut BasePattern {
        &mut self.base
    }
    // color value will allow client to test that world_point was transformed
    fn color_at_world(&self, world_point: Tuple) -> Color {
        let (u, v) = self.uv_mapping.point_to_uv(world_point);
        self.uv_pattern.color_at(u, v)
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SphericalMap;
impl UVMapping for SphericalMap {
    fn point_to_uv(&self, p: Tuple) -> (f32, f32) {
        // compute the azimuthal angle -π < θ <= π
        // angle increases clockwise as viewed from above,
        // which is opposite of what we want, but we'll fix it later.
        let theta = p.x.atan2(p.z);

        let origin_to_p = vector!(p.x, p.y, p.z);
        let radius = origin_to_p.magnitude();

        // compute the polar angle
        // 0 <= φ <= π
        let phi = (p.y / radius).acos();

        // -0.5 < raw_u <= 0.5
        let raw_u = theta / (2. * PI);

        // 0 <= u < 1
        // here's also where we fix the direction of u. Subtract it from 1,
        // so that it increases counterclockwise as viewed from above.
        let u = 1. - (raw_u + 0.5);

        // we want v to be 0 at the south pole of the sphere,
        // and 1 at the north pole, so we have to "flip it over"
        // by subtracting it from 1.
        let v = 1. - phi / PI;

        (u, v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::f32::consts::FRAC_1_SQRT_2;
    #[test]
    fn uv_checkers_pattern() {
        let p = UVCheckers::new(2., 2., black(), white());
        let test_data = vec![
            ("1", 0.0, 0.0, black()),
            ("2", 0.5, 0.0, white()),
            ("3", 0.0, 0.5, white()),
            ("4", 0.5, 0.5, black()),
            ("5", 1.0, 1.0, black()),
        ];
        for (name, u, v, expected_color) in test_data {
            assert_eq!(p.color_at(u, v), expected_color, "Case {}", name);
        }
    }

    #[test]
    fn spherical_map_on_3d_point() {
        let test_data = vec![
            ("1", point!(0, 0, -1), 0.0, 0.5),
            ("2", point!(1, 0, 0), 0.25, 0.5),
            ("3", point!(0, 0, 1), 0.5, 0.5),
            ("4", point!(-1, 0, 0), 0.75, 0.5),
            ("5", point!(0, 1, 0), 0.5, 1.0),
            ("6", point!(0, -1, 0), 0.5, 0.0),
            ("7", point!(FRAC_1_SQRT_2, FRAC_1_SQRT_2, 0), 0.25, 0.75),
        ];
        for (name, p, expected_u, expected_v) in test_data {
            let (u, v) = SphericalMap.point_to_uv(p);
            println!("Case {}", name);
            assert_abs_diff_eq!(u, expected_u);
            assert_abs_diff_eq!(v, expected_v);
        }
    }

    #[test]
    fn using_texture_map_pattern_with_spherical_map() {
        let checkers = UVCheckers::new(16., 8., black(), white());
        let texture_map = TextureMap::new(Box::new(checkers), Box::new(SphericalMap));
        let test_data = vec![
            ("", point!(0.4315, 0.4670, 0.7719), white()),
            ("", point!(-0.9654, 0.2552, -0.0534), black()),
            ("", point!(0.1039, 0.7090, 0.6975), white()),
            ("", point!(-0.4986, -0.7856, -0.3663), black()),
            ("", point!(-0.0317, -0.9395, 0.3411), black()),
            ("", point!(0.4809, -0.7721, 0.4154), black()),
            ("", point!(0.0285, -0.9612, -0.2745), black()),
            ("", point!(-0.5734, -0.2162, -0.7903), white()),
            ("", point!(0.7688, -0.1470, 0.6223), black()),
            ("", point!(-0.7652, 0.2175, 0.6060), black()),
        ];
        for (name, p, expected_color) in test_data {
            assert_eq!(
                texture_map.color_at_world(p),
                expected_color,
                "Case {}",
                name
            );
        }
    }
}
