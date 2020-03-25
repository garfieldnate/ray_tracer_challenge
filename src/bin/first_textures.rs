use ray_tracer_challenge::camera::Camera;
use ray_tracer_challenge::color::Color;
use ray_tracer_challenge::constants::DEFAULT_RAY_RECURSION_DEPTH;
use ray_tracer_challenge::constants::{black, white};
use ray_tracer_challenge::light::point_light::PointLight;
use ray_tracer_challenge::material::Material;
use ray_tracer_challenge::pattern::checkers::Checkers;
use ray_tracer_challenge::pattern::pattern::Pattern;
use ray_tracer_challenge::pattern::uv::{SphericalMap, TextureMap, UVCheckers};
use ray_tracer_challenge::shape::plane::Plane;
use ray_tracer_challenge::shape::sphere::Sphere;
use ray_tracer_challenge::transformations::scaling;
use ray_tracer_challenge::transformations::translation;
use ray_tracer_challenge::transformations::view_transform;
use ray_tracer_challenge::tuple::Tuple;
use ray_tracer_challenge::world::World;
use ray_tracer_challenge::{color, point, vector};
use std::f32::consts::PI;

const CANVAS_WIDTH: u32 = 1000;
const CANVAS_HEIGHT: u32 = 500;

fn main() {
    let mut checkers = Checkers::new(color!(0.1, 1, 0.5), color!(0.9, 0.2, 0.6));
    checkers.set_transformation(scaling(0.1, 0.1, 0.1));
    let mut room_material = Material::default();
    room_material.color = color!(1, 0.9, 0.9);
    room_material.pattern = Some(Box::new(checkers));
    room_material.specular = 0.0;
    let floor = Plane::build(scaling(10.0, 0.01, 10.0), room_material);

    let mut middle_sphere_material = Material::default();
    middle_sphere_material.color = color!(0.1, 1, 0.5);
    middle_sphere_material.pattern = Some(Box::new(TextureMap::new(
        Box::new(UVCheckers::new(16., 8., black(), white())),
        Box::new(SphericalMap),
    )));
    middle_sphere_material.diffuse = 0.7;
    middle_sphere_material.specular = 0.3;
    let middle = Sphere::build(translation(-0.5, 1.0, 0.5), middle_sphere_material);

    let world = World {
        objects: vec![Box::new(floor), Box::new(middle)],
        // The light source is white, shining from above and to the left
        light: Some(Box::new(PointLight::new(point!(-10, 10, -10), white()))),
    };

    let camera = Camera::new(
        CANVAS_WIDTH,
        CANVAS_HEIGHT,
        PI / 3.0,
        view_transform(point!(0, 1.5, -10), point!(2, 2.8, 0), vector!(0, 1, 0)),
    );

    let canvas = camera.render(world, DEFAULT_RAY_RECURSION_DEPTH);
    println!("{}", canvas.to_ppm());
}
