// Demo scene from the BVH bonus chapter

// TODO: implement YAML file reading

use ray_tracer_challenge::camera::Camera;
use ray_tracer_challenge::color::Color;
use ray_tracer_challenge::light::point_light::PointLight;
use ray_tracer_challenge::material::Material;
use ray_tracer_challenge::matrix::Matrix;
use ray_tracer_challenge::obj_parser::parse_obj;
use ray_tracer_challenge::shape::cube::Cube;
use ray_tracer_challenge::shape::cylinder::Cylinder;
use ray_tracer_challenge::shape::group::GroupShape;
use ray_tracer_challenge::shape::shape::Shape;
use ray_tracer_challenge::transformations::rotation_y;
use ray_tracer_challenge::transformations::scaling;
use ray_tracer_challenge::transformations::translation;
use ray_tracer_challenge::transformations::view_transform;
use ray_tracer_challenge::tuple::Tuple;
use ray_tracer_challenge::world::World;
use ray_tracer_challenge::{color, point, vector};
use std::f32::consts::PI;
use std::{env, fs::File, path::Path};

// To render larger, be sure to use an optimized (release) build and give it up to a minute to finish
const CANVAS_WIDTH: u32 = 1000;
const CANVAS_HEIGHT: u32 = 400;
// const CANVAS_WIDTH: u32 = 500;
// const CANVAS_HEIGHT: u32 = 200;
// const CANVAS_WIDTH: u32 = 250;
// const CANVAS_HEIGHT: u32 = 100;
// const CANVAS_WIDTH: u32 = 110;
// const CANVAS_HEIGHT: u32 = 50;

fn main() {
    let args: Vec<String> = env::args().collect();
    let dragon_file_path = Path::new(&args[1]);

    let light = get_light();

    let center_front_transform = &translation(0., 0.5, -4.) * &rotation_y(PI);
    let center_front_dragon_material = {
        let mut m = Material::default();
        m.color = color!(1, 1, 1);
        m.ambient = 0.1;
        m.diffuse = 0.6;
        m.specular = 0.3;
        m.shininess = 15.;
        m
    };
    let center_front_case_material = None;

    let center_back_transform = translation(0., 2., 2.);
    let center_back_dragon_material = {
        let mut m = Material::default();
        m.color = color!(1, 0, 0.1);
        m.ambient = 0.1;
        m.diffuse = 0.6;
        m.specular = 0.3;
        m.shininess = 15.;
        m
    };
    let center_back_case_material = {
        let mut m = Material::default();
        m.ambient = 0.;
        m.diffuse = 0.4;
        m.specular = 0.;
        m.transparency = 0.6;

        Some(m)
    };
    let center_left_transform =
        &translation(-2., 0.75, -1.) * &(&rotation_y(-PI / 8.) * &scaling(0.75, 0.75, 0.75));
    let center_left_dragon_material = {
        let mut m = Material::default();
        m.color = color!(0.9, 0.5, 0.1);
        m.ambient = 0.1;
        m.diffuse = 0.6;
        m.specular = 0.3;
        m.shininess = 15.;
        m
    };
    let center_left_case_material = {
        let mut m = Material::default();
        m.ambient = 0.;
        m.diffuse = 0.2;
        m.specular = 0.;
        m.transparency = 0.8;

        Some(m)
    };
    let left_transform =
        &translation(-4., 0., -2.) * &(&rotation_y(-PI / 16.) * &scaling(0.5, 0.5, 0.5));
    let left_dragon_material = {
        let mut m = Material::default();
        m.color = color!(1, 0.9, 0.1);
        m.ambient = 0.1;
        m.diffuse = 0.6;
        m.specular = 0.3;
        m.shininess = 15.;

        m
    };
    let left_case_material = {
        let mut m = Material::default();
        m.ambient = 0.;
        m.diffuse = 0.1;
        m.specular = 0.;
        m.transparency = 0.9;

        Some(m)
    };
    let right_transform =
        &translation(4., 0., -2.) * &(&rotation_y(21. * PI / 20.) * &scaling(0.5, 0.5, 0.5));
    let right_dragon_material = {
        let mut m = Material::default();
        m.color = color!(0.9, 1, 0.1);
        m.ambient = 0.1;
        m.diffuse = 0.6;
        m.specular = 0.3;
        m.shininess = 15.;

        m
    };
    let right_case_material = {
        let mut m = Material::default();
        m.ambient = 0.;
        m.diffuse = 0.1;
        m.specular = 0.;
        m.transparency = 0.9;

        Some(m)
    };
    let center_right_transform =
        &translation(2., 1., -1.) * &(&rotation_y(5. * PI / 4.) * &scaling(0.75, 0.75, 0.75));
    let center_right_dragon_material = {
        let mut m = Material::default();
        m.color = color!(1, 0.5, 0.1);
        m.ambient = 0.1;
        m.diffuse = 0.6;
        m.specular = 0.3;
        m.shininess = 15.;

        m
    };
    let center_right_case_material = {
        let mut m = Material::default();
        m.ambient = 0.;
        m.diffuse = 0.2;
        m.specular = 0.;
        m.transparency = 0.8;

        Some(m)
    };

    let mut element_data = vec![
        (
            center_front_transform,
            center_front_dragon_material,
            center_front_case_material,
        ),
        (
            center_back_transform,
            center_back_dragon_material,
            center_back_case_material,
        ),
        (
            center_left_transform,
            center_left_dragon_material,
            center_left_case_material,
        ),
        (left_transform, left_dragon_material, left_case_material),
        (
            center_right_transform,
            center_right_dragon_material,
            center_right_case_material,
        ),
        (right_transform, right_dragon_material, right_case_material),
    ];

    let objects: Vec<Box<dyn Shape>> = element_data
        .drain(0..)
        .map(|(element_transform, dragon_material, case_material)| {
            get_scene_element(
                dragon_file_path,
                element_transform,
                dragon_material,
                case_material,
            )
        })
        .map(|el| Box::new(el) as _)
        .collect();

    let world = World {
        objects,
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

fn get_display_case() -> Cube {
    let mut c = Cube::new();
    c.set_casts_shadow(false);
    // scale cube to fit the dragon model inside comfortably; translate up to sit on the pedestal
    // extra 0.001 is to avoid salt and pepper noise on shared boundary with pedestal
    c.set_transformation(&scaling(1.1, 0.77, 0.49) * &translation(0., 1.001, 0.));

    c
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

fn get_dragon(dragon_file_path: &Path) -> GroupShape {
    let file = File::open(dragon_file_path).unwrap();
    let mut parse_results = parse_obj(file).unwrap();
    let mut dragon = parse_results.take_all_as_group().unwrap();
    // lift dragon so that it sits on the pedestal
    // raw normalized OBJ bounds were:
    //     min: Tuple { x: -1.0, y: -0.6922653, z: -0.4361561, w: 1.0 }
    //     max: Tuple { x: 0.99999994, y: 0.6922653, z: 0.4361561, w: 1.0 } }
    dragon.set_transformation(translation(0., 0.69, 0.));

    eprintln!("Finished parsing dragon");

    dragon
}

fn get_scene_element(
    dragon_file_path: &Path,
    element_transform: Matrix,
    dragon_material: Material,
    display_case_material: Option<Material>,
) -> GroupShape {
    let mut element = GroupShape::new();
    element.set_transformation(element_transform);

    let mut dragon = get_dragon(dragon_file_path);
    eprintln!("Setting dragon material...");
    dragon.set_material(dragon_material);

    let dragon_box = {
        match display_case_material {
            Some(m) => {
                let mut display_case = get_display_case();
                display_case.set_material(m);

                let mut dragon_box = GroupShape::new();
                dragon_box.add_child(Box::new(dragon));
                dragon_box.add_child(Box::new(display_case));

                dragon_box
            }
            None => dragon,
        }
    };

    element.add_child(Box::new(dragon_box));
    element.add_child(Box::new(get_pedestal()));

    eprintln!("Dividing element...");
    element.divide(4);
    eprintln!("Finished dividing element");

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
