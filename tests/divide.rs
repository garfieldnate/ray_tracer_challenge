// Show a teapot
use ray_tracer_challenge::camera::Camera;
use ray_tracer_challenge::canvas::Canvas;
use ray_tracer_challenge::color::Color;
use ray_tracer_challenge::constants::red;
use ray_tracer_challenge::constants::white;
use ray_tracer_challenge::light::point_light::PointLight;
use ray_tracer_challenge::material::Material;
use ray_tracer_challenge::matrix::identity_4x4;
use ray_tracer_challenge::obj_parser::parse_obj;
use ray_tracer_challenge::shape::cube::Cube;
use ray_tracer_challenge::shape::cylinder::Cylinder;
use ray_tracer_challenge::shape::group::GroupShape;
use ray_tracer_challenge::shape::plane::Plane;
use ray_tracer_challenge::shape::shape::Shape;
use ray_tracer_challenge::shape::sphere::Sphere;
use ray_tracer_challenge::transformations::rotation_x;
use ray_tracer_challenge::transformations::scaling;
use ray_tracer_challenge::transformations::translation;
use ray_tracer_challenge::transformations::view_transform;
use ray_tracer_challenge::tuple::Tuple;
use ray_tracer_challenge::world::World;
use ray_tracer_challenge::{color, point, vector};
use std::{env, fs::File, path::Path};

use std::f32::consts::PI;

// To render larger, be sure to use an optimized (release) build and give it up to a minute to finish
const CANVAS_WIDTH: u32 = 300;
const CANVAS_HEIGHT: u32 = 300;

#[test]
fn main() {
    let with_divide_canvas = get_canvas(true);
    let without_divide_canvas = get_canvas(false);

    assert_eq!(with_divide_canvas.to_ppm(), without_divide_canvas.to_ppm());
    assert!(false);
}

fn get_canvas(divide: bool) -> Canvas {
    let light = get_light();
    let world = World {
        objects: vec![Box::new(get_obj(
            Path::new("/Users/nathanglenn/Downloads/teapot-low.obj"),
            divide,
        ))],
        light: Some(Box::new(light)),
    };

    let camera = Camera::new(
        CANVAS_WIDTH,
        CANVAS_HEIGHT,
        1.,
        view_transform(point!(0, 0, -2), point!(0, 0, 0), vector!(0, 1, 0)),
    );

    let canvas = camera.render(world, 0);

    canvas
}
fn get_light() -> PointLight {
    PointLight::new(point!(-10, 100, -100), color!(1, 1, 1))
}

fn get_obj(obj_file_path: &Path, divide: bool) -> GroupShape {
    let file = File::open(obj_file_path).unwrap();
    let mut parse_results = parse_obj(file).unwrap();
    let mut teapot = parse_results.take_all_as_group().unwrap();
    eprintln!("Finished parsing obj");
    teapot.set_transformation(rotation_x(-PI / 2.));
    eprintln!("Finished transforming teapot");
    if divide {
        teapot.divide(2);
        eprintln!("Finished dividing teapot");
    }

    teapot
}
