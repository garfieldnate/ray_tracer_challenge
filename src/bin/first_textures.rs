use ray_tracer_challenge::camera::Camera;
use ray_tracer_challenge::color::Color;
use ray_tracer_challenge::constants::DEFAULT_RAY_RECURSION_DEPTH;
use ray_tracer_challenge::constants::{black, white};
use ray_tracer_challenge::light::point_light::PointLight;
use ray_tracer_challenge::material::Material;
use ray_tracer_challenge::pattern::uv::{
    CylindricalMap, PlanarMap, SphericalMap, TextureMap, UVCheckers,
};
use ray_tracer_challenge::shape::cylinder::Cylinder;
use ray_tracer_challenge::shape::plane::Plane;
use ray_tracer_challenge::shape::shape::Shape;
use ray_tracer_challenge::shape::sphere::Sphere;
use ray_tracer_challenge::transformations::view_transform;
use ray_tracer_challenge::transformations::{scaling, translation};
use ray_tracer_challenge::tuple::Tuple;
use ray_tracer_challenge::world::World;
use ray_tracer_challenge::{color, point, vector};
use std::f32::consts::PI;

const CANVAS_WIDTH: u32 = 1000;
const CANVAS_HEIGHT: u32 = 500;

fn main() {
    let floor = {
        let mut material = Material::default();
        material.color = color!(1, 0.9, 0.9);
        material.pattern = Some(Box::new(TextureMap::new(
            Box::new(UVCheckers::new(16., 8., black(), white())),
            Box::new(PlanarMap),
        )));
        material.specular = 0.0;
        Plane::build(scaling(10.0, 0.01, 10.0), material)
    };

    let middle = {
        let mut material = Material::default();
        material.color = color!(0.1, 1, 0.5);
        material.pattern = Some(Box::new(TextureMap::new(
            Box::new(UVCheckers::new(16., 8., black(), white())),
            Box::new(SphericalMap),
        )));
        material.diffuse = 0.7;
        material.specular = 0.3;
        Sphere::build(translation(-0.5, 1.0, 0.5), material)
    };

    let right = {
        let mut material = Material::default();
        material.ambient = 0.1;
        material.specular = 0.6;
        material.shininess = 15.;
        material.diffuse = 0.8;
        material.pattern = Some(Box::new(TextureMap::new(
            Box::new(UVCheckers::new(16., 8., color!(0, 0.5, 0), white())),
            Box::new(CylindricalMap),
        )));

        // y scaling by PI is always required for cylinders to get proper-looking squares
        let mut c = Cylinder::build(&translation(2., 0., 0.) * &scaling(1., PI, 1.), material);
        c.maximum_y = 1.;
        c.minimum_y = 0.;
        c
    };

    let world = World {
        objects: vec![Box::new(floor), Box::new(middle), Box::new(right)],
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
