use ray_tracer_challenge::camera::Camera;
use ray_tracer_challenge::color::Color;
use ray_tracer_challenge::light::build_point_light;
use ray_tracer_challenge::material::default_material;
use ray_tracer_challenge::shape::sphere::Sphere;
use ray_tracer_challenge::transformations::rotation_x;
use ray_tracer_challenge::transformations::rotation_y;
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
    let mut room_material = default_material();
    room_material.color = color!(1, 0.9, 0.9);
    room_material.specular = 0.0;
    // The floor is an extremely flattened sphere with a matte texture.
    let floor = Sphere::build(scaling(10.0, 0.01, 10.0), room_material);

    // The wall on the left has the same scale and color as the floor, but is also rotated and translated into place.
    let left_wall = Sphere::build(
        &translation(0.0, 0.0, 5.0)
            * &(&rotation_y(-PI / 4.0) * &(&rotation_x(PI / 2.0) * &scaling(10.0, 0.01, 10.0))),
        room_material,
    );

    // The wall on the right is identical to the left wall, but is rotated the opposite direction in y.
    let right_wall = Sphere::build(
        &translation(0.0, 0.0, 5.0)
            * &(&rotation_y(PI / 4.0) * &(&rotation_x(PI / 2.0) * &scaling(10.0, 0.01, 10.0))),
        room_material,
    );

    // The large sphere in the middle is a unit sphere, translated upward slightly and colored green.

    let mut middle_sphere_material = default_material();
    middle_sphere_material.color = color!(0.1, 1, 0.5);
    middle_sphere_material.diffuse = 0.7;
    middle_sphere_material.specular = 0.3;
    let middle = Sphere::build(translation(-0.5, 1.0, 0.5), middle_sphere_material);

    // The smaller green sphere on the right is scaled in half

    let mut right_sphere_material = default_material();
    right_sphere_material.color = color!(0.5, 1, 0.1);
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
    left_sphere_material.diffuse = 0.7;
    left_sphere_material.specular = 0.3;
    let left = Sphere::build(
        &translation(-1.5, 0.33, -0.75) * &scaling(0.33, 0.33, 0.33),
        left_sphere_material,
    );

    let world = World {
        objects: vec![floor, left_wall, right_wall, left, middle, right],
        // The light source is white, shining from above and to the left
        light: Some(build_point_light(point!(-10, 10, -10), color!(1, 1, 1))),
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
