use crate::canvas::Canvas;
use crate::color::Color;
use crate::constants::black;
use crate::constants::{blue, brown, cyan, green, purple, red, white, yellow};
use crate::pattern::pattern::BasePattern;
use crate::pattern::pattern::Pattern;
use crate::tuple::Tuple;
use dyn_clone::DynClone;
use std::f32::consts::{FRAC_1_PI, PI};
use std::fmt::{Debug, Formatter, Result};

const FRAC_1_2PI: f32 = 1. / (2. * PI);

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
    pub fn new(width: f32, height: f32, a: Color, b: Color) -> Self {
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
        let u = calculate_u_from_azimuth(p);

        let origin_to_p = vector!(p.x, p.y, p.z);
        let radius = origin_to_p.magnitude();

        // compute the polar angle
        // 0 <= φ <= π
        let phi = (p.y / radius).acos();

        // we want v to be 0 at the south pole of the sphere,
        // and 1 at the north pole, so we have to "flip it over"
        // by subtracting it from 1.
        let v = 1. - phi * FRAC_1_PI;

        (u, v)
    }
}

fn calculate_u_from_azimuth(p: Tuple) -> f32 {
    // compute the azimuthal angle -π < θ <= π
    // angle increases clockwise as viewed from above,
    // which is opposite of what we want, but we'll fix it later.
    let theta = p.x.atan2(p.z);

    // -0.5 < raw_u <= 0.5
    let raw_u = theta * FRAC_1_2PI;

    // 0 <= u < 1
    // here's also where we fix the direction of u. Subtract it from 1,
    // so that it increases counterclockwise as viewed from above.
    let u = 1. - (raw_u + 0.5);

    u
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AlignCheck {
    main: Color,
    ul: Color,
    ur: Color,
    bl: Color,
    br: Color,
}

impl AlignCheck {
    pub fn new(main: Color, ul: Color, ur: Color, bl: Color, br: Color) -> Self {
        AlignCheck {
            main,
            ul,
            ur,
            bl,
            br,
        }
    }
}

impl UVPattern for AlignCheck {
    fn color_at(&self, u: f32, v: f32) -> Color {
        // remember: v = 0 at the bottom, v = 1 at the top
        if v > 0.8 {
            if u < 0.2 {
                return self.ul;
            }
            if u > 0.8 {
                return self.ur;
            }
        } else if v < 0.2 {
            if u < 0.2 {
                return self.bl;
            }
            if u > 0.8 {
                return self.br;
            }
        }

        self.main
    }
}

impl Default for AlignCheck {
    fn default() -> Self {
        Self::new(white(), red(), yellow(), green(), cyan())
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Face {
    Front = 0,
    Back = 1,
    Left = 2,
    Right = 3,
    Up = 4,
    Down = 5,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct PlanarMap;
impl UVMapping for PlanarMap {
    fn point_to_uv(&self, p: Tuple) -> (f32, f32) {
        (p.x.rem_euclid(1.), p.z.rem_euclid(1.))
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CylindricalMap;
impl UVMapping for CylindricalMap {
    fn point_to_uv(&self, p: Tuple) -> (f32, f32) {
        let u = calculate_u_from_azimuth(p);
        // let v go from 0 to 1 between 2*pi units of y
        let v = p.y.rem_euclid(2. * PI) * FRAC_1_2PI;

        return (u, v);
    }
}

#[derive(Clone, Debug)]
pub struct CubicMap {
    base: BasePattern,
    uv_patterns: Vec<Box<dyn UVPattern>>,
}

impl CubicMap {
    pub fn new(
        front: Box<dyn UVPattern>,
        back: Box<dyn UVPattern>,
        left: Box<dyn UVPattern>,
        right: Box<dyn UVPattern>,
        up: Box<dyn UVPattern>,
        down: Box<dyn UVPattern>,
    ) -> Self {
        // TODO: using a proper EnumMap class would be great, but there isn't one currently.
        let mut uv_patterns: Vec<Box<dyn UVPattern>> = Vec::with_capacity(6);
        // it's important that these are inserted in the same order as they are declared in
        // the enum
        uv_patterns.push(front);
        uv_patterns.push(back);
        uv_patterns.push(left);
        uv_patterns.push(right);
        uv_patterns.push(up);
        uv_patterns.push(down);

        Self {
            base: BasePattern::new(),
            uv_patterns,
        }
    }
}

impl Pattern for CubicMap {
    fn get_base(&self) -> &BasePattern {
        &self.base
    }
    fn get_base_mut(&mut self) -> &mut BasePattern {
        &mut self.base
    }

    // color value will allow client to test that world_point was transformed
    fn color_at_world(&self, world_point: Tuple) -> Color {
        let face = face_from_point(world_point);
        let (u, v) = match face {
            Face::Left => cube_uv_left(world_point),
            Face::Right => cube_uv_right(world_point),
            Face::Front => cube_uv_front(world_point),
            Face::Back => cube_uv_back(world_point),
            Face::Up => cube_uv_up(world_point),
            Face::Down => cube_uv_down(world_point),
        };

        self.uv_patterns[face as usize].color_at(u, v)
    }
}

fn face_from_point(p: Tuple) -> Face {
    let abs_x = p.x.abs();
    let abs_y = p.y.abs();
    let abs_z = p.z.abs();
    let coord = abs_x.max(abs_y).max(abs_z);

    if coord == p.x {
        Face::Right
    } else if coord == -p.x {
        Face::Left
    } else if coord == p.y {
        Face::Up
    } else if coord == -p.y {
        Face::Down
    } else if coord == p.z {
        Face::Front
    } else {
        Face::Back
    }
}

fn cube_uv_front(p: Tuple) -> (f32, f32) {
    let u = ((p.x + 1.) % 2.) / 2.;
    let v = ((p.y + 1.) % 2.) / 2.;
    (u, v)
}

fn cube_uv_back(p: Tuple) -> (f32, f32) {
    let u = ((1. - p.x) % 2.) / 2.;
    let v = ((p.y + 1.) % 2.) / 2.;
    (u, v)
}

fn cube_uv_left(p: Tuple) -> (f32, f32) {
    let u = ((p.z + 1.) % 2.) / 2.;
    let v = ((p.y + 1.) % 2.) / 2.;
    (u, v)
}

fn cube_uv_right(p: Tuple) -> (f32, f32) {
    let u = ((1. - p.z) % 2.) / 2.;
    let v = ((p.y + 1.) % 2.) / 2.;
    (u, v)
}

fn cube_uv_up(p: Tuple) -> (f32, f32) {
    let u = ((p.x + 1.) % 2.) / 2.;
    let v = ((1. - p.z) % 2.) / 2.;
    (u, v)
}

fn cube_uv_down(p: Tuple) -> (f32, f32) {
    let u = ((p.x + 1.) % 2.) / 2.;
    let v = ((p.z + 1.) % 2.) / 2.;
    (u, v)
}

pub fn get_align_check_cubic_map_pattern() -> CubicMap {
    let left = AlignCheck::new(yellow(), cyan(), red(), blue(), brown());
    let front = AlignCheck::new(cyan(), red(), yellow(), brown(), green());
    let right = AlignCheck::new(red(), yellow(), purple(), green(), white());
    let back = AlignCheck::new(green(), purple(), cyan(), white(), blue());
    let up = AlignCheck::new(brown(), cyan(), purple(), red(), yellow());
    let down = AlignCheck::new(purple(), brown(), green(), blue(), white());
    let pattern = CubicMap::new(
        Box::new(front),
        Box::new(back),
        Box::new(left),
        Box::new(right),
        Box::new(up),
        Box::new(down),
    );
    pattern
}

#[derive(Clone)]
pub struct UVImage {
    canvas: Canvas,
}
impl UVImage {
    pub fn new(canvas: Canvas) -> Self {
        Self { canvas }
    }
}

impl Debug for UVImage {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        // TODO: print abbreviate canvas representation (too big to print all of it as debug representation)
        f.debug_struct("UVImage")
            .field("width", &self.canvas.width)
            .field("height", &self.canvas.height)
            .finish()
    }
}

impl UVPattern for UVImage {
    fn color_at(&self, u: f32, v: f32) -> Color {
        // flip v over so it matches the image layout, with y at the top
        let v = 1. - v;

        let x = u * (self.canvas.width - 1) as f32;
        let y = v * (self.canvas.height - 1) as f32;

        // be sure and round x and y to the nearest whole number
        self.canvas.pixel_at(x.round() as usize, y.round() as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canvas::canvas_from_ppm;
    use std::f32::consts::FRAC_1_SQRT_2;

    use crate::constants::{black, blue, brown, cyan, purple};
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
            ("1", point!(0.4315, 0.4670, 0.7719), white()),
            ("2", point!(-0.9654, 0.2552, -0.0534), black()),
            ("3", point!(0.1039, 0.7090, 0.6975), white()),
            ("4", point!(-0.4986, -0.7856, -0.3663), black()),
            ("5", point!(-0.0317, -0.9395, 0.3411), black()),
            ("6", point!(0.4809, -0.7721, 0.4154), black()),
            ("7", point!(0.0285, -0.9612, -0.2745), black()),
            ("8", point!(-0.5734, -0.2162, -0.7903), white()),
            ("9", point!(0.7688, -0.1470, 0.6223), black()),
            ("10", point!(-0.7652, 0.2175, 0.6060), black()),
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

    #[test]
    fn using_planar_mapping_on_3d_point() {
        let test_data = vec![
            ("1", point!(0.25, 0, 0.5), 0.25, 0.5),
            ("2", point!(0.25, 0, -0.25), 0.25, 0.75),
            ("3", point!(0.25, 0.5, -0.25), 0.25, 0.75),
            ("4", point!(1.25, 0, 0.5), 0.25, 0.5),
            ("5", point!(0.25, 0, -1.75), 0.25, 0.25),
            ("6", point!(1, 0, -1), 0.0, 0.0),
            ("7", point!(0, 0, 0), 0.0, 0.0),
        ];
        for (name, p, expected_u, expected_v) in test_data {
            let (u, v) = PlanarMap.point_to_uv(p);
            assert_eq!((u, v), (expected_u, expected_v), "Case {}", name);
        }
    }

    #[test]
    fn using_cylindrical_mapping_on_3d_point() {
        let test_data = vec![
            ("1", point!(0, 0, -1), 0.0, 0.0),
            ("2", point!(0, 0.5, -1), 0.0, 0.07957747),
            ("3", point!(0, 1, -1), 0.0, 0.15915494),
            ("4", point!(0.70711, 0.5, -0.70711), 0.125, 0.07957747),
            ("5", point!(1, 0.5, 0), 0.25, 0.07957747),
            ("6", point!(0.70711, 0.5, 0.70711), 0.375, 0.07957747),
            ("7", point!(0, -0.25, 1), 0.5, 0.9602113),
            ("8", point!(-0.70711, 0.5, 0.70711), 0.625, 0.07957747),
            ("9", point!(-1, 1.25, 0), 0.75, 0.19894367),
            ("10", point!(-0.70711, 0.5, -0.70711), 0.875, 0.07957747),
        ];
        for (name, p, expected_u, expected_v) in test_data {
            let (u, v) = CylindricalMap.point_to_uv(p);
            println!(
                "Case {}: expected {:?}, got {:?}",
                name,
                (expected_u, expected_v),
                (u, v)
            );
            assert_abs_diff_eq!(u, expected_u);
            assert_abs_diff_eq!(v, expected_v);
        }
    }

    #[test]
    fn layout_of_align_check_pattern() {
        let main = color!(1, 1, 1);
        let ul = color!(1, 0, 0);
        let ur = color!(1, 1, 0);
        let bl = color!(0, 1, 0);
        let br = color!(0, 1, 1);
        let align_check = AlignCheck::new(main, ul, ur, bl, br);
        let test_data = vec![
            ("1", 0.5, 0.5, main),
            ("2", 0.1, 0.9, ul),
            ("3", 0.9, 0.9, ur),
            ("4", 0.1, 0.1, bl),
            ("5", 0.9, 0.1, br),
        ];
        for (name, u, v, expected_color) in test_data {
            let c = align_check.color_at(u, v);
            assert_eq!(c, expected_color, "Case {}", name);
        }
    }

    #[test]
    fn identifying_face_of_cube_from_point() {
        let test_data = vec![
            ("1", point!(-1, 0.5, -0.25), Face::Left),
            ("2", point!(1.1, -0.75, 0.8), Face::Right),
            ("3", point!(0.1, 0.6, 0.9), Face::Front),
            ("4", point!(-0.7, 0, -2), Face::Back),
            ("5", point!(0.5, 1, 0.9), Face::Up),
            ("6", point!(-0.2, -1.3, 1.1), Face::Down),
        ];
        for (name, p, expected_face) in test_data {
            let face = face_from_point(p);
            assert_eq!(face, expected_face, "Case {}", name);
        }
    }

    #[test]
    fn uv_mapping_the_front_face_of_a_cube() {
        let test_data = vec![
            ("1", point!(-0.5, 0.5, 1), 0.25, 0.75),
            ("2", point!(0.5, -0.5, 1), 0.75, 0.25),
        ];
        for (name, p, expected_u, expected_v) in test_data {
            let (u, v) = cube_uv_front(p);
            assert_eq!((u, v), (expected_u, expected_v), "Case {}", name);
        }
    }

    #[test]
    fn uv_mapping_the_back_face_of_a_cube() {
        let test_data = vec![
            ("1", point!(0.5, 0.5, -1), 0.25, 0.75),
            ("2", point!(-0.5, -0.5, -1), 0.75, 0.25),
        ];
        for (name, p, expected_u, expected_v) in test_data {
            let (u, v) = cube_uv_back(p);
            assert_eq!((u, v), (expected_u, expected_v), "Case {}", name);
        }
    }

    #[test]
    fn uv_mapping_the_left_face_of_a_cube() {
        let test_data = vec![
            ("1", point!(-1, 0.5, -0.5), 0.25, 0.75),
            ("2", point!(-1, -0.5, 0.5), 0.75, 0.25),
        ];
        for (name, p, expected_u, expected_v) in test_data {
            let (u, v) = cube_uv_left(p);
            assert_eq!((u, v), (expected_u, expected_v), "Case {}", name);
        }
    }

    #[test]
    fn uv_mapping_the_right_face_of_a_cube() {
        let test_data = vec![
            ("1", point!(1, 0.5, 0.5), 0.25, 0.75),
            ("2", point!(1, -0.5, -0.5), 0.75, 0.25),
        ];
        for (name, p, expected_u, expected_v) in test_data {
            let (u, v) = cube_uv_right(p);
            assert_eq!((u, v), (expected_u, expected_v), "Case {}", name);
        }
    }

    #[test]
    fn uv_mapping_the_upper_face_of_a_cube() {
        let test_data = vec![
            ("1", point!(-0.5, 1, -0.5), 0.25, 0.75),
            ("2", point!(0.5, 1, 0.5), 0.75, 0.25),
        ];
        for (name, p, expected_u, expected_v) in test_data {
            let (u, v) = cube_uv_up(p);
            assert_eq!((u, v), (expected_u, expected_v), "Case {}", name);
        }
    }

    #[test]
    fn uv_mapping_the_lower_face_of_a_cube() {
        let test_data = vec![
            ("1", point!(-0.5, -1, 0.5), 0.25, 0.75),
            ("2", point!(0.5, -1, -0.5), 0.75, 0.25),
        ];
        for (name, p, expected_u, expected_v) in test_data {
            let (u, v) = cube_uv_down(p);
            assert_eq!((u, v), (expected_u, expected_v), "Case {}", name);
        }
    }

    #[test]
    fn finding_colors_on_mapped_cube() {
        let pattern = get_align_check_cubic_map_pattern();

        let test_data = vec![
            ("L1", point!(-1, 0, 0), yellow()),
            ("L2", point!(-1, 0.9, -0.9), cyan()),
            ("L3", point!(-1, 0.9, 0.9), red()),
            ("L4", point!(-1, -0.9, -0.9), blue()),
            ("L5", point!(-1, -0.9, 0.9), brown()),
            ("F1", point!(0, 0, 1), cyan()),
            ("F2", point!(-0.9, 0.9, 1), red()),
            ("F3", point!(0.9, 0.9, 1), yellow()),
            ("F4", point!(-0.9, -0.9, 1), brown()),
            ("F5", point!(0.9, -0.9, 1), green()),
            ("R1", point!(1, 0, 0), red()),
            ("R2", point!(1, 0.9, 0.9), yellow()),
            ("R3", point!(1, 0.9, -0.9), purple()),
            ("R4", point!(1, -0.9, 0.9), green()),
            ("R5", point!(1, -0.9, -0.9), white()),
            ("B1", point!(0, 0, -1), green()),
            ("B2", point!(0.9, 0.9, -1), purple()),
            ("B3", point!(-0.9, 0.9, -1), cyan()),
            ("B4", point!(0.9, -0.9, -1), white()),
            ("B5", point!(-0.9, -0.9, -1), blue()),
            ("U1", point!(0, 1, 0), brown()),
            ("U2", point!(-0.9, 1, -0.9), cyan()),
            ("U3", point!(0.9, 1, -0.9), purple()),
            ("U4", point!(-0.9, 1, 0.9), red()),
            ("U5", point!(0.9, 1, 0.9), yellow()),
            ("D1", point!(0, -1, 0), purple()),
            ("D2", point!(-0.9, -1, 0.9), brown()),
            ("D3", point!(0.9, -1, 0.9), green()),
            ("D4", point!(-0.9, -1, -0.9), blue()),
            ("D5", point!(0.9, -1, -0.9), white()),
        ];

        for (name, p, expected_color) in test_data {
            assert_eq!(pattern.color_at_world(p), expected_color, "Case {}", name);
        }
    }

    #[test]
    fn uv_mapping_an_image() {
        let ppm = "P3
        10 10
        10
        0 0 0  1 1 1  2 2 2  3 3 3  4 4 4  5 5 5  6 6 6  7 7 7  8 8 8  9 9 9
        1 1 1  2 2 2  3 3 3  4 4 4  5 5 5  6 6 6  7 7 7  8 8 8  9 9 9  0 0 0
        2 2 2  3 3 3  4 4 4  5 5 5  6 6 6  7 7 7  8 8 8  9 9 9  0 0 0  1 1 1
        3 3 3  4 4 4  5 5 5  6 6 6  7 7 7  8 8 8  9 9 9  0 0 0  1 1 1  2 2 2
        4 4 4  5 5 5  6 6 6  7 7 7  8 8 8  9 9 9  0 0 0  1 1 1  2 2 2  3 3 3
        5 5 5  6 6 6  7 7 7  8 8 8  9 9 9  0 0 0  1 1 1  2 2 2  3 3 3  4 4 4
        6 6 6  7 7 7  8 8 8  9 9 9  0 0 0  1 1 1  2 2 2  3 3 3  4 4 4  5 5 5
        7 7 7  8 8 8  9 9 9  0 0 0  1 1 1  2 2 2  3 3 3  4 4 4  5 5 5  6 6 6
        8 8 8  9 9 9  0 0 0  1 1 1  2 2 2  3 3 3  4 4 4  5 5 5  6 6 6  7 7 7
        9 9 9  0 0 0  1 1 1  2 2 2  3 3 3  4 4 4  5 5 5  6 6 6  7 7 7  8 8 8
        ";
        let canvas = canvas_from_ppm(ppm.as_bytes()).unwrap();
        let pattern = UVImage::new(canvas);

        let test_data = vec![
            ("1", 0., 0., color!(0.9, 0.9, 0.9)),
            ("2", 0.3, 0., color!(0.2, 0.2, 0.2)),
            ("3", 0.6, 0.3, color!(0.1, 0.1, 0.1)),
            ("4", 1., 1., color!(0.9, 0.9, 0.9)),
        ];
        for (name, u, v, expected_color) in test_data {
            let color = pattern.color_at(u, v);
            assert_eq!(color, expected_color, "Case {}", name);
        }
    }
}
