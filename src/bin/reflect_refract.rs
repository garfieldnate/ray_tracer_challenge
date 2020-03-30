use ray_tracer_challenge::camera::Camera;
use ray_tracer_challenge::color::Color;
use ray_tracer_challenge::constants::metal;
use ray_tracer_challenge::constants::{white, yellow, REFRACTION_GLASS};
use ray_tracer_challenge::light::point_light::PointLight;
use ray_tracer_challenge::material::Material;
use ray_tracer_challenge::pattern::pattern::Pattern;
use ray_tracer_challenge::pattern::rings::Rings;
use ray_tracer_challenge::pattern::sine_2d::Sine2D;
use ray_tracer_challenge::pattern::stripes::Stripes;
use ray_tracer_challenge::shape::cone::Cone;
use ray_tracer_challenge::shape::csg::{CSGOperator, CSG};
use ray_tracer_challenge::shape::cylinder::Cylinder;
use ray_tracer_challenge::shape::plane::Plane;
use ray_tracer_challenge::shape::shape::Shape;
use ray_tracer_challenge::shape::sphere::Sphere;
use ray_tracer_challenge::transformations::scaling;
use ray_tracer_challenge::transformations::shearing;
use ray_tracer_challenge::transformations::translation;
use ray_tracer_challenge::transformations::view_transform;
use ray_tracer_challenge::transformations::{rotation_x, rotation_z};
use ray_tracer_challenge::tuple::Tuple;
use ray_tracer_challenge::world::World;
use ray_tracer_challenge::{color, point, vector};
use std::f32::consts::PI;

// To render larger, be sure to use an optimized (release) build and give it several seconds to finish
const CANVAS_WIDTH: u32 = 1000;
const CANVAS_HEIGHT: u32 = 500;
// const CANVAS_WIDTH: u32 = 200;
// const CANVAS_HEIGHT: u32 = 100;
// const CANVAS_WIDTH: u32 = 100;
// const CANVAS_HEIGHT: u32 = 50;

fn main() {
    let mut stripes = Stripes::new(color!(1., 0.2, 0.4), color!(0.1, 0.1, 0.1));
    stripes.set_transformation(&scaling(0.3, 0.3, 0.3) * &rotation_z(3. * PI / 4.));
    let mut sine2d = Sine2D::new(color!(0.1, 1, 0.5), color!(0.9, 0.2, 0.6));
    sine2d.set_transformation(&scaling(0.05, 1., 0.05) * &translation(-5., 1., 0.5));
    let room_material = Material::builder()
        .pattern(Box::new(sine2d))
        .specular(0.)
        .reflective(0.5)
        .build();
    // The floor is a plane
    let floor = Plane::build(scaling(10., 0.1, 10.), room_material);

    // The large sphere in the middle is a unit sphere, translated upward slightly and colored green.

    let middle = get_clear_sphere();

    // The smaller green sphere on the right is scaled in half

    let right_sphere_material = Material::builder()
        .pattern(Box::new(stripes.clone()))
        .diffuse(0.7)
        .specular(0.3)
        .build();
    let mut metal_rings = metal();
    let mut ring_pattern = Rings::new(yellow() / 2., white() / 2.);
    ring_pattern.set_transformation(scaling(0.1, 0.1, 0.1));
    metal_rings.pattern = Some(Box::new(ring_pattern));
    let right = Sphere::build(
        &shearing(0., 1., 0., 0., 0., 1.)
            * &(&translation(1.5, 0.5, -0.5) * &scaling(0.5, 0.5, 0.5)),
        metal_rings,
    );

    // The smallest sphere is scaled by a third before being translated

    // have to make this thing much darker since it will also be reflective
    let mut stripes2 = stripes;
    stripes2.a = stripes2.a / 4.;
    stripes2.b = stripes2.b / 4.;
    let left_sphere_material = Material::builder()
        .pattern(Box::new(stripes2))
        .diffuse(0.7)
        .specular(1.)
        .reflective(0.8)
        .shininess(300.)
        .build();
    let left = Sphere::build(
        &translation(-1.5, 0.33, -0.75) * &scaling(0.33, 0.33, 0.33),
        left_sphere_material,
    );

    let cylinder = get_cylinder();

    let cone = {
        let mut c = Cone::new();
        c.maximum_y = 1.5;
        c.minimum_y = 0.;
        let mut m = Material::builder().build();
        m.reflective = 0.5;
        m.color = color!(0.6, 0.3, 0.1);
        m.shininess = 10.;
        m.specular = 0.8;
        c.set_material(m);
        c.set_transformation(&translation(-3.5, 0., 4.) * &scaling(0.33, 1.8, 0.33));
        c
    };

    let world = World {
        objects: vec![
            Box::new(floor),
            Box::new(left),
            Box::new(middle),
            Box::new(right),
            Box::new(cylinder),
            Box::new(cone),
            // Box::new(get_csg()),
        ],
        // The light source is white, shining from above and to the left
        light: Some(Box::new(PointLight::new(point!(-10, 10, -10), white()))),
    };

    let camera = Camera::new(
        CANVAS_WIDTH,
        CANVAS_HEIGHT,
        PI / 3.,
        view_transform(point!(0, 1.5, -5), point!(0, 1, 0), vector!(0, 1, 0)),
    );

    let canvas = camera.render(world, 5);
    println!("{}", canvas.to_ppm());
}

fn get_clear_sphere() -> Sphere {
    let middle_sphere_material = Material::builder()
        .color(color!(0, 0, 0))
        .specular(1.)
        .shininess(300.)
        .transparency(1.)
        .refractive_index(REFRACTION_GLASS)
        .reflective(1.)
        .build();

    let mut sphere = Sphere::build(translation(-0.5, 1., 0.5), middle_sphere_material);
    sphere.set_casts_shadow(false);
    sphere
}

fn get_cylinder() -> Cylinder {
    let mut c = Cylinder::new();
    c.maximum_y = 1.5;
    c.minimum_y = 0.;
    c.set_material(
        Material::builder()
            .reflective(1.)
            .color(color!(0.5, 0.5, 0.5))
            .shininess(300.)
            .specular(0.8)
            .build(),
    );
    c.set_transformation(&translation(3.7, 0., 4.) * &scaling(0.33, 1.8, 0.33));
    c
}

fn get_csg() -> CSG {
    let mut s1 = get_clear_sphere();
    s1.set_transformation(translation(0., 1., 0.));
    // s1
    let mut s2 = get_cylinder();
    s2.set_transformation(scaling(0.2, 2.0, 0.2));
    s2.set_material(
        Material::builder()
            .reflective(0.)
            .refractive_index(1.)
            .transparency(1.)
            .build(),
    );
    s2.set_casts_shadow(false);
    // s2
    let mut c = CSG::new(CSGOperator::Difference(), Box::new(s1), Box::new(s2));
    c.set_transformation(translation(0., 0., 2.));
    c
}
