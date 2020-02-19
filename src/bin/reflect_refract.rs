use ray_tracer_challenge::camera::Camera;
use ray_tracer_challenge::color::Color;
use ray_tracer_challenge::light::PointLight;
use ray_tracer_challenge::material::default_material;
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
const CANVAS_WIDTH: u32 = 1000;
const CANVAS_HEIGHT: u32 = 500;
// const CANVAS_WIDTH: u32 = 100;
// const CANVAS_HEIGHT: u32 = 50;

fn main() {
    let mut stripes = Stripes::new(color!(1.0, 0.2, 0.4), color!(0.1, 0.1, 0.1));
    stripes.set_transformation(&scaling(0.3, 0.3, 0.3) * &rotation_z(3.0 * PI / 4.0));
    let mut sine2d = Sine2D::new(color!(0.1, 1, 0.5), color!(0.9, 0.2, 0.6));
    sine2d.set_transformation(&scaling(0.005, 1.0, 0.005) * &translation(-5.0, 1.0, 0.5));
    let mut room_material = default_material();
    room_material.color = color!(1, 0.9, 0.9);
    room_material.pattern = Some(Box::new(sine2d.clone()));
    room_material.specular = 0.0;
    room_material.reflective = 0.5;
    // The floor is a plane
    let floor = Plane::build(scaling(10.0, 0.01, 10.0), room_material);

    // The large sphere in the middle is a unit sphere, translated upward slightly and colored green.

    let mut middle_sphere_material = default_material();
    middle_sphere_material.color = color!(0.1, 1, 0.5);
    middle_sphere_material.pattern = Some(Box::new(stripes.clone()));
    middle_sphere_material.diffuse = 0.7;
    middle_sphere_material.specular = 0.3;
    let middle = Sphere::build(translation(-0.5, 1.0, 0.5), middle_sphere_material);

    // The smaller green sphere on the right is scaled in half

    let mut right_sphere_material = default_material();
    right_sphere_material.color = color!(0.5, 1, 0.1);
    right_sphere_material.pattern = Some(Box::new(stripes.clone()));
    right_sphere_material.diffuse = 0.7;
    right_sphere_material.specular = 0.3;
    let right = Sphere::build(
        &shearing(0.0, 1.0, 0.0, 0.0, 0.0, 1.0)
            * &(&translation(1.5, 0.5, -0.5) * &scaling(0.5, 0.5, 0.5)),
        right_sphere_material,
    );

    // The smallest sphere is scaled by a third before being translated
    let mut left_sphere_material = default_material();
    left_sphere_material.color = color!(1, 0.8, 0.1);
    left_sphere_material.pattern = Some(Box::new(stripes.clone()));
    left_sphere_material.diffuse = 0.7;
    left_sphere_material.specular = 0.3;
    left_sphere_material.reflective = 0.8;
    let left = Sphere::build(
        &translation(-1.5, 0.33, -0.75) * &scaling(0.33, 0.33, 0.33),
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
        light: Some(PointLight::new(point!(-10, 10, -10), color!(1, 1, 1))),
    };

    let camera = Camera::new(
        CANVAS_WIDTH,
        CANVAS_HEIGHT,
        PI / 3.0,
        view_transform(point!(0, 1.5, -5), point!(0, 1, 0), vector!(0, 1, 0)),
    );

    let canvas = camera.render(world);
    println!("{}", canvas.to_ppm());
}
