use ray_tracer_challenge::camera::Camera;
use ray_tracer_challenge::color::Color;
use ray_tracer_challenge::constants::white;
use ray_tracer_challenge::constants::DEFAULT_RAY_RECURSION_DEPTH;
use ray_tracer_challenge::light::point_light::PointLight;
use ray_tracer_challenge::material::Material;
use ray_tracer_challenge::pattern::pattern::Pattern;
use ray_tracer_challenge::pattern::sine_2d::Sine2D;
use ray_tracer_challenge::pattern::stripes::Stripes;
use ray_tracer_challenge::shape::plane::Plane;
use ray_tracer_challenge::shape::sphere::Sphere;
use ray_tracer_challenge::transformations::rotation_z;
use ray_tracer_challenge::transformations::scaling;
use ray_tracer_challenge::transformations::shearing;
use ray_tracer_challenge::transformations::translation;
use ray_tracer_challenge::transformations::view_transform;
use ray_tracer_challenge::tuple::Tuple;
use ray_tracer_challenge::world::World;
use ray_tracer_challenge::{color, point, vector};
use std::f32::consts::PI;

// To render larger, be sure to use an optimized (release) build and give it several minutes to finish
// const CANVAS_WIDTH: u32 = 1000;
// const CANVAS_HEIGHT: u32 = 500;
const CANVAS_WIDTH: u32 = 100;
const CANVAS_HEIGHT: u32 = 50;

fn main() {
    let mut stripes = Stripes::new(color!(1.0, 0.2, 0.4), color!(0.1, 0.1, 0.1));
    stripes.set_transformation(scaling(0.3, 0.3, 0.3) * rotation_z(3.0 * PI / 4.0));
    let mut sine2d = Sine2D::new(color!(0.1, 1, 0.5), color!(0.9, 0.2, 0.6));
    sine2d.set_transformation(scaling(0.005, 1.0, 0.005) * translation(-5.0, 1.0, 0.5));
    let room_material = Material::builder()
        .pattern(Box::new(sine2d))
        .specular(0.)
        .build();
    // The floor is a plane
    let floor = Plane::build(scaling(10.0, 0.01, 10.0), room_material);

    // The large sphere in the middle is a unit sphere, translated upward slightly and colored green.
    let middle_sphere_material = Material::builder()
        .pattern(Box::new(stripes.clone()))
        .diffuse(0.7)
        .specular(0.3)
        .build();
    let middle = Sphere::build(translation(-0.5, 1.0, 0.5), middle_sphere_material);

    // The smaller green sphere on the right is scaled in half
    let right_sphere_material = Material::builder()
        .pattern(Box::new(stripes.clone()))
        .diffuse(0.7)
        .specular(0.3)
        .build();
    let right = Sphere::build(
        shearing(0.0, 1.0, 0.0, 0.0, 0.0, 1.0)
            * translation(1.5, 0.5, -0.5)
            * scaling(0.5, 0.5, 0.5),
        right_sphere_material,
    );

    // The smallest sphere is scaled by a third before being translated
    let left_sphere_material = Material::builder()
        .pattern(Box::new(stripes))
        .diffuse(0.7)
        .specular(0.3)
        .build();
    let left = Sphere::build(
        translation(-1.5, 0.33, -0.75) * scaling(0.33, 0.33, 0.33),
        left_sphere_material,
    );

    let world = World {
        objects: vec![
            Box::new(floor),
            Box::new(left),
            Box::new(middle),
            Box::new(right),
        ],
        // The light source is white, shining from above and to the left
        light: Some(Box::new(PointLight::new(point!(-10, 10, -10), white()))),
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
