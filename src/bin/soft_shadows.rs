// Implements the glamor shot from the bonus sof shadows chapter
// TODO: implement YAML file reading

use ray_tracer_challenge::camera::Camera;
use ray_tracer_challenge::color::Color;
use ray_tracer_challenge::constants::red;
use ray_tracer_challenge::constants::white;
use ray_tracer_challenge::light::rectangle_light::RectangleLight;
use ray_tracer_challenge::material::Material;
use ray_tracer_challenge::matrix::identity_4x4;
use ray_tracer_challenge::shape::cube::Cube;
use ray_tracer_challenge::shape::plane::Plane;
use ray_tracer_challenge::shape::shape::Shape;
use ray_tracer_challenge::shape::sphere::Sphere;
use ray_tracer_challenge::transformations::scaling;
use ray_tracer_challenge::transformations::translation;
use ray_tracer_challenge::transformations::view_transform;
use ray_tracer_challenge::tuple::Tuple;
use ray_tracer_challenge::world::World;
use ray_tracer_challenge::{color, point, vector};
use std::f32::consts::PI;

// To render larger, be sure to use an optimized (release) build and give it up to a minute to finish
const CANVAS_WIDTH: u32 = 1000;
const CANVAS_HEIGHT: u32 = 400;
// const CANVAS_WIDTH: u32 = 400;
// const CANVAS_HEIGHT: u32 = 160;
// const CANVAS_WIDTH: u32 = 200;
// const CANVAS_HEIGHT: u32 = 100;
// const CANVAS_WIDTH: u32 = 100;
// const CANVAS_HEIGHT: u32 = 50;

fn main() {
    let light = get_light();

    let world = World {
        objects: vec![
            Box::new(get_lampshade()),
            Box::new(get_floor()),
            Box::new(get_sphere_1()),
            Box::new(get_sphere_2()),
        ],
        light: Some(Box::new(light)),
    };

    //     - add: camera
    //   width: 400
    //   height: 160
    //   field-of-view: 0.7854
    //   from: [-3, 1, 2.5]
    //   to: [0, 0.5, 0]
    //   up: [0, 1, 0]
    let camera = Camera::new(
        CANVAS_WIDTH,
        CANVAS_HEIGHT,
        PI / 4.,
        view_transform(point!(-3, 1, 2.5), point!(0, 0.5, 0), vector!(0, 1, 0)),
    );

    let canvas = camera.render(world, 5);
    println!("{}", canvas.to_ppm());
}

// - add: light
//   corner: [-1, 2, 4]
//   uvec: [2, 0, 0]
//   vvec: [0, 2, 0]
//   usteps: 10
//   vsteps: 10
//   jitter: true
//   intensity: [1.5, 1.5, 1.5]
fn get_light() -> RectangleLight<'static> {
    RectangleLight::new(
        color!(1.5, 1.5, 1.5),
        point!(-1, 2, 4),
        vector!(2, 0, 0),
        10,
        vector!(0, 2, 0),
        10,
        None,
    )
}

// Put the light in the middle of a "shining" cube so that it can show
// up in reflections as a physical thing. Naturally, the cube must
// opt out of shadow tests...
// - add: cube
//   material:
//     color: [1.5, 1.5, 1.5]
//     ambient: 1
//     diffuse: 0
//     specular: 0
//   transform:
//     - [ scale, 1, 1, 0.01 ]
//     - [ translate, 0, 3, 4 ]
//   shadow: false
fn get_lampshade() -> Cube {
    let m = Material::builder()
        // TODO: What? I thought 1 was the max!
        .color(color!(1.5, 1.5, 1.5))
        .ambient(1.)
        .diffuse(0.)
        .specular(0.)
        .build();
    let transform = &translation(0., 3., 4.) * &scaling(1., 1., 0.01);
    let mut c = Cube::build(transform, m);
    c.set_casts_shadow(false);
    c
}

// - add: plane
//   material:
//     color: [1, 1, 1]
//     ambient: 0.025
//     diffuse: 0.67
//     specular: 0
fn get_floor() -> Plane {
    let material = Material::builder()
        .color(white())
        .ambient(0.025)
        .diffuse(0.67)
        .specular(0.)
        .build();
    Plane::build(identity_4x4(), material)
}

// - add: sphere
//   transform:
//     - [ scale, 0.5, 0.5, 0.5 ]
//     - [ translate, 0.5, 0.5, 0 ]
//   material:
//     color: [1, 0, 0]
//     ambient: 0.1
//     specular: 0
//     diffuse: 0.6
//     reflective: 0.3
fn get_sphere_1() -> Sphere {
    let transform = &translation(0.5, 0.5, 0.) * &scaling(0.5, 0.5, 0.5);
    let material = Material::builder()
        .color(red())
        .ambient(0.1)
        .specular(0.)
        .diffuse(0.6)
        .reflective(0.3)
        .build();
    Sphere::build(transform, material)
}

// - add: sphere
//   transform:
//     - [ scale, 0.33, 0.33, 0.33 ]
//     - [ translate, -0.25, 0.33, 0 ]
//   material:
//     color: [0.5, 0.5, 1]
//     ambient: 0.1
//     specular: 0
//     diffuse: 0.6
//     reflective: 0.3
fn get_sphere_2() -> Sphere {
    let transform = &translation(-0.25, 0.33, 0.) * &scaling(0.33, 0.33, 0.33);
    let material = Material::builder()
        .color(color!(0.5, 0.5, 1))
        .ambient(0.1)
        .specular(0.)
        .diffuse(0.6)
        .reflective(0.3)
        .build();
    Sphere::build(transform, material)
}
