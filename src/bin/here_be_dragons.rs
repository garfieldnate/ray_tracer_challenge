// Demo scene from the BVH bonus chapter

// TODO: implement YAML file reading

use ray_tracer_challenge::camera::Camera;
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
use ray_tracer_challenge::transformations::scaling;
use ray_tracer_challenge::transformations::translation;
use ray_tracer_challenge::transformations::view_transform;
use ray_tracer_challenge::tuple::Tuple;
use ray_tracer_challenge::world::World;
use ray_tracer_challenge::{color, point, vector};
use std::{env, fs::File, path::Path};

// To render larger, be sure to use an optimized (release) build and give it up to a minute to finish
// const CANVAS_WIDTH: u32 = 1000;
// const CANVAS_HEIGHT: u32 = 400;
const CANVAS_WIDTH: u32 = 500;
const CANVAS_HEIGHT: u32 = 200;
// const CANVAS_WIDTH: u32 = 250;
// const CANVAS_HEIGHT: u32 = 100;
// const CANVAS_WIDTH: u32 = 110;
// const CANVAS_HEIGHT: u32 = 50;

fn main() {
    let args: Vec<String> = env::args().collect();
    let dragon_file_path = Path::new(&args[1]);

    let light = get_light();

    let world = World {
        objects: vec![
            Box::new(get_scene_element(dragon_file_path)),
            // Box::new(get_floor()),
            // Box::new(get_sphere_1()),
            // Box::new(get_sphere_2()),
        ],
        light: Some(Box::new(light)),
    };

    // - add: camera
    //   width: 500
    //   height: 200
    //   field-of-view: 1.2
    //   from: [0, 2.5, -10]
    //   to: [0, 1, 0]
    //   up: [0, 1, 0]

    let camera = Camera::new(
        CANVAS_WIDTH,
        CANVAS_HEIGHT,
        1.2,
        view_transform(point!(0, 2.5, -10), point!(0, 1, 0), vector!(0, 1, 0)),
    );

    let canvas = camera.render(world, 5);
    println!("{}", canvas.to_ppm());
}

// TODO: support multiple lights; for now we just use the first one
// - add: light
//   at: [-10, 100, -100]
//   intensity: [1, 1, 1]

// - add: light
//   at: [0, 100, 0]
//   intensity: [0.1, 0.1, 0.1]

// - add: light
//   at: [100, 10, -25]
//   intensity: [0.2, 0.2, 0.2]

// - add: light
//   at: [-100, 10, -25]
//   intensity: [0.2, 0.2, 0.2]
fn get_light() -> PointLight {
    PointLight::new(point!(-10, 100, -100), color!(1, 1, 1))
}

// - define: raw-bbox
//   value:
//     add: cube
//     shadow: false
//     transform:
//       - [ translate, 1, 1, 1 ]
//       - [ scale, 3.73335, 2.5845, 1.6283 ]
//       - [ translate, -3.9863, -0.1217, -1.1820 ]
fn get_raw_bbox() -> Cube {
    let mut c = Cube::new();
    c.set_casts_shadow(false);
    c.set_transformation(
        &translation(-3.9863, -0.1217, -1.1820)
            * &(&scaling(3.73335, 2.5845, 1.6283) * &translation(1., 1., 1.)),
    );

    c
}

// - define: bbox
//   value:
//     add: raw-bbox
//     transform:
//       - [ translate, 0, 0.1217, 0]
//       - [ scale, 0.268, 0.268, 0.268 ]
fn get_bbox() -> GroupShape {
    let mut g = GroupShape::new();
    g.add_child(Box::new(get_raw_bbox()));
    g.set_transformation(&scaling(0.268, 0.268, 0.268) * &translation(0., 0.1217, 0.));

    g
}

// - define: pedestal
//   value:
//     add: cylinder
//     min: -0.15
//     max: 0
//     closed: true
//     material:
//       color: [ 0.2, 0.2, 0.2 ]
//       ambient: 0
//       diffuse: 0.8
//       specular: 0
//       reflective: 0.2
fn get_pedestal() -> Cylinder {
    let mut c = Cylinder::new();
    c.maximum_y = 0.;
    c.minimum_y = -0.15;
    c.closed = true;

    let mut m = Material::default();
    m.color = color!(0.2, 0.2, 0.2);
    m.ambient = 0.;
    m.diffuse = 0.8;
    m.specular = 0.;
    m.reflective = 0.2;
    c.set_material(m);

    c
}

// - define: dragon
//   value:
//     add: obj
//     file: dragon.obj
//     transform:
//       - [ translate, 0, 0.1217, 0]
//       - [ scale, 0.268, 0.268, 0.268 ]
fn get_dragon(dragon_file_path: &Path) -> GroupShape {
    let file = File::open(dragon_file_path).unwrap();
    let mut parse_results = parse_obj(file).unwrap();
    let mut dragon = parse_results.take_all_as_group().unwrap();
    dragon.set_transformation(&scaling(0.268, 0.268, 0.268) * &translation(0., 0.1217, 0.));
    eprintln!("Finished parsing dragon");

    // dragon.divide(5);
    // eprintln!("Finished dividing dragon");

    dragon
}

// - add: group
//   transform:
//     - [ translate, 0, 2, 0 ]
//   children:
//     - add: pedestal
//     - add: group
//       children:
//         - add: dragon
//           material:
//             color: [ 1, 0, 0.1 ]
//             ambient: 0.1
//             diffuse: 0.6
//             specular: 0.3
//             shininess: 15
//         - add: bbox
//           material:
//             ambient: 0
//             diffuse: 0.4
//             specular: 0
//             transparency: 0.6
//             refractive-index: 1
fn get_scene_element(dragon_file_path: &Path) -> GroupShape {
    let mut element = GroupShape::new();
    element.set_transformation(translation(0., 2., 0.));

    let mut dragon = get_dragon(dragon_file_path);
    let mut dragon_material = Material::default();
    dragon_material.color = color!(1., 0., 0.1);
    dragon_material.ambient = 0.1;
    dragon_material.diffuse = 0.6;
    dragon_material.specular = 0.3;
    dragon_material.shininess = 15.;
    dragon.set_material(dragon_material);

    let mut display_case = get_bbox();
    let mut case_material = Material::default();
    case_material.ambient = 0.;
    case_material.diffuse = 0.4;
    case_material.specular = 0.;
    case_material.transparency = 0.6;
    case_material.refractive_index = 1.;
    display_case.set_material(case_material);

    let mut dragon_box = GroupShape::new();
    dragon_box.add_child(Box::new(dragon));
    dragon_box.add_child(Box::new(display_case));

    element.add_child(Box::new(dragon_box));
    element.add_child(Box::new(get_pedestal()));

    element
}
// - add: group
//   transform:
//     - [ translate, 2, 1, -1 ]
//   children:
//     - add: pedestal
//     - add: group
//       transform:
//         - [ rotate-y, 4 ]
//         - [ scale, 0.75, 0.75, 0.75 ]
//       children:
//         - add: dragon
//           material:
//             color: [ 1, 0.5, 0.1 ]
//             ambient: 0.1
//             diffuse: 0.6
//             specular: 0.3
//             shininess: 15
//         - add: bbox
//           material:
//             ambient: 0
//             diffuse: 0.2
//             specular: 0
//             transparency: 0.8
//             refractive-index: 1

// - add: group
//   transform:
//     - [ translate, -2, .75, -1 ]
//   children:
//     - add: pedestal
//     - add: group
//       transform:
//         - [ rotate-y, -0.4 ]
//         - [ scale, 0.75, 0.75, 0.75 ]
//       children:
//         - add: dragon
//           material:
//             color: [ 0.9, 0.5, 0.1 ]
//             ambient: 0.1
//             diffuse: 0.6
//             specular: 0.3
//             shininess: 15
//         - add: bbox
//           material:
//             ambient: 0
//             diffuse: 0.2
//             specular: 0
//             transparency: 0.8
//             refractive-index: 1

// - add: group
//   transform:
//     - [ translate, -4, 0, -2 ]
//   children:
//     - add: pedestal
//     - add: group
//       transform:
//         - [ rotate-y, -0.2 ]
//         - [ scale, 0.5, 0.5, 0.5 ]
//       children:
//         - add: dragon
//           material:
//             color: [ 1, 0.9, 0.1 ]
//             ambient: 0.1
//             diffuse: 0.6
//             specular: 0.3
//             shininess: 15
//         - add: bbox
//           material:
//             ambient: 0
//             diffuse: 0.1
//             specular: 0
//             transparency: 0.9
//             refractive-index: 1

// - add: group
//   transform:
//     - [ translate, 4, 0, -2 ]
//   children:
//     - add: pedestal
//     - add: group
//       transform:
//         - [ rotate-y, 3.3 ]
//         - [ scale, 0.5, 0.5, 0.5 ]
//       children:
//         - add: dragon
//           material:
//             color: [ 0.9, 1, 0.1 ]
//             ambient: 0.1
//             diffuse: 0.6
//             specular: 0.3
//             shininess: 15
//         - add: bbox
//           material:
//             ambient: 0
//             diffuse: 0.1
//             specular: 0
//             transparency: 0.9
//             refractive-index: 1

// - add: group
//   transform:
//     - [ translate, 0, 0.5, -4 ]
//   children:
//     - add: pedestal
//     - add: dragon
//       material:
//         color: [ 1, 1, 1 ]
//         ambient: 0.1
//         diffuse: 0.6
//         specular: 0.3
//         shininess: 15
//       transform:
//         - [ rotate-y, 3.1415 ]
