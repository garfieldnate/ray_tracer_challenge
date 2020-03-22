extern crate ray_tracer_challenge;
// Show a teapot
use ray_tracer_challenge::camera::Camera;
use ray_tracer_challenge::color::Color;
use ray_tracer_challenge::light::point_light::PointLight;
use ray_tracer_challenge::obj_parser::parse_obj;
use ray_tracer_challenge::shape::group::GroupShape;
use ray_tracer_challenge::shape::shape::Shape;
use ray_tracer_challenge::shape::triangle::Triangle;
use ray_tracer_challenge::transformations::rotation_x;
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
fn divide_teapot() {
    let obj_file_path = Path::new("/Users/nathanglenn/Downloads/teapot-low.obj");

    let light = get_light();

    let world_without_divide = World {
        objects: vec![Box::new(get_obj(obj_file_path, false))],
        light: Some(Box::new(light)),
    };

    let world_with_divide = World {
        objects: vec![Box::new(get_obj(obj_file_path, true))],
        light: Some(Box::new(light)),
    };

    let camera = Camera::new(
        CANVAS_WIDTH,
        CANVAS_HEIGHT,
        1.,
        view_transform(point!(0, 0, -2), point!(0, 0, 0), vector!(0, 1, 0)),
    );
    // let x = 207;
    // let y = 82;
    for y in 0..CANVAS_HEIGHT - 1 {
        for x in 0..CANVAS_WIDTH - 1 {
            let ray = camera.ray_for_pixel(x, y);
            let color_with_divide = world_with_divide.color_at(ray, 0);
            let color_without_divide = world_without_divide.color_at(ray, 0);
            assert_eq!(color_with_divide, color_without_divide);
            // eprintln!("With divide...");
            // let intersections_with_divide: Vec<f32> = world_with_divide
            //     .intersect(ray)
            //     .iter()
            //     .map(|i| i.distance)
            //     .collect();
            // eprintln!("Without divide...");
            // let intersections_without_divide: Vec<f32> = world_without_divide
            //     .intersect(ray)
            //     .iter()
            //     .map(|i| i.distance)
            //     .collect();
            // assert_eq!(
            //     intersections_with_divide, intersections_without_divide,
            //     "Pixel at {},{}",
            //     x, y
            // );
        }
    }
}

fn get_light() -> PointLight {
    PointLight::new(point!(-10, 100, -100), color!(1, 1, 1))
}

fn get_obj(obj_file_path: &Path, divide: bool) -> GroupShape {
    // let file = File::open(obj_file_path).unwrap();
    // let mut parse_results = parse_obj(file).unwrap();
    // let mut shape = parse_results.take_all_as_group().unwrap();
    // eprintln!("Finished parsing obj");
    // shape.set_transformation(rotation_x(-PI / 2.));
    // eprintln!("Finished transforming teapot");

    let mut shape = GroupShape::new();
    shape.add_child(Box::new(Triangle::new(
        point!(7, 0, 12),
        point!(5, -5, 12),
        point!(5, 0, 12),
    )));
    shape.add_child(Box::new(Triangle::new(
        point!(0, 10, 5),
        point!(6, 6, 8),
        point!(7, 7, 5),
    )));
    if divide {
        eprintln!("Dividing teapot");
        shape.divide(2);
        eprintln!("Finished dividing teapot");
    }

    shape
}
