use ray_tracer_challenge::camera::Camera;
use ray_tracer_challenge::color::Color;
use ray_tracer_challenge::constants::glass;
use ray_tracer_challenge::constants::white;
use ray_tracer_challenge::constants::DEFAULT_RAY_RECURSION_DEPTH;
use ray_tracer_challenge::light::PointLight;
use ray_tracer_challenge::material::Material;
use ray_tracer_challenge::pattern::checkers::Checkers;
use ray_tracer_challenge::shape::cylinder::Cylinder;
use ray_tracer_challenge::shape::group::GroupShape;
use ray_tracer_challenge::shape::plane::Plane;
use ray_tracer_challenge::shape::shape::Shape;
use ray_tracer_challenge::shape::sphere::Sphere;
use ray_tracer_challenge::transformations::rotation_x;
use ray_tracer_challenge::transformations::rotation_y;
use ray_tracer_challenge::transformations::rotation_z;
use ray_tracer_challenge::transformations::scaling;
use ray_tracer_challenge::transformations::translation;
use ray_tracer_challenge::transformations::view_transform;
use ray_tracer_challenge::tuple::Tuple;
use ray_tracer_challenge::world::World;
use ray_tracer_challenge::{point, vector};
use std::f32::consts::PI;
use std::str::FromStr;

// To render larger, be sure to use an optimized (release) build and give it several minutes to finish
const CANVAS_WIDTH: u32 = 1000;
const CANVAS_HEIGHT: u32 = 500;
// const CANVAS_WIDTH: u32 = 100;
// const CANVAS_HEIGHT: u32 = 50;

fn main() {
    let floor = {
        let mut plane = Plane::new();
        plane.set_transformation(&translation(0.0, 0.0, 5.0) * &rotation_x(PI / 2.0));

        // TODO: looks like checkers doesn't work right. Creates square rings. Which are also cool!
        let checkers = Checkers::new(
            Color::from_str("#C5D86D").unwrap(),
            Color::from_str("#261C15").unwrap(),
        );
        // checkers.set_transformation(rotation_x(PI / 2.0));
        let mut m = Material::default();

        m.pattern = Some(Box::new(checkers));
        plane.set_material(m);
        Box::new(plane)
    };
    let mut hex1 = hexagon(&glass());
    hex1.set_transformation(&translation(0.0, 0.75, 0.0) * &rotation_x(PI / 2.0));
    let world = World {
        objects: vec![floor, Box::new(hex1)],
        // The light source is white, shining from above and to the left
        light: Some(PointLight::new(point!(-10, 10, -10), white())),
    };

    let camera = Camera::new(
        CANVAS_WIDTH,
        CANVAS_HEIGHT,
        PI / 3.0,
        view_transform(point!(0, 1.5, -5), point!(0, 1, 0), vector!(0, 1, 0)),
    );

    let canvas = camera.render(world, DEFAULT_RAY_RECURSION_DEPTH);
    println!("{}", canvas.to_ppm());
}

fn hexagon_corner(m: &Material) -> Sphere {
    Sphere::build(
        &translation(0.0, 0.0, -1.0) * &scaling(0.25, 0.25, 0.25),
        m.clone(),
    )
}

fn hexagon_edge(m: &Material) -> Cylinder {
    let mut edge = Cylinder::new();
    edge.minimum_y = 0.0;
    edge.maximum_y = 1.0;
    edge.set_transformation(
        &translation(0.0, 0.0, -1.0)
            * &(&rotation_y(-PI / 6.0) * &(&rotation_z(-PI / 2.0) * &scaling(0.25, 1.0, 0.25))),
    );
    edge.set_material(m.clone());
    edge
}

fn hexagon_side(m: &Material) -> GroupShape {
    let mut side = GroupShape::new();
    side.add_child(Box::new(hexagon_corner(m)));
    side.add_child(Box::new(hexagon_edge(m)));
    side
}

fn hexagon(m: &Material) -> GroupShape {
    let mut hex = GroupShape::new();
    for n in 0..=5 {
        let mut side = hexagon_side(m);
        side.set_transformation(rotation_y(n as f32 * PI / 3.0));
        hex.add_child(Box::new(side));
    }
    hex
}
