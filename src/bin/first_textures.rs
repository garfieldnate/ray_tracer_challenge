// 1 argument: path to PPM texture for center sphere
// In the demo, we use an earth texture downloaded from http://planetpixelemporium.com/download/download.php?earthmap1k.jpg
use ray_tracer_challenge::camera::Camera;
use ray_tracer_challenge::canvas::canvas_from_ppm;
use ray_tracer_challenge::color::Color;
use ray_tracer_challenge::constants::DEFAULT_RAY_RECURSION_DEPTH;
use ray_tracer_challenge::constants::{black, white};
use ray_tracer_challenge::light::light::Light;
use ray_tracer_challenge::light::rectangle_light::RectangleLight;
use ray_tracer_challenge::material::Material;
use ray_tracer_challenge::pattern::uv::get_align_check_cubic_map_pattern;
use ray_tracer_challenge::pattern::uv::{
    CylindricalMap, PlanarMap, SphericalMap, TextureMap, UVCheckers, UVImage,
};
use ray_tracer_challenge::shape::cube::Cube;
use ray_tracer_challenge::shape::cylinder::Cylinder;
use ray_tracer_challenge::shape::group::GroupShape;
use ray_tracer_challenge::shape::plane::Plane;
use ray_tracer_challenge::shape::shape::Shape;
use ray_tracer_challenge::shape::sphere::Sphere;
use ray_tracer_challenge::transformations::view_transform;
use ray_tracer_challenge::transformations::{rotation_x, rotation_y, scaling, translation};
use ray_tracer_challenge::tuple::Tuple;
use ray_tracer_challenge::world::World;
use ray_tracer_challenge::{color, point, vector};
use std::f32::consts::PI;
use std::{env, fs::File, path::Path};

const CANVAS_WIDTH: u32 = 1000;
const CANVAS_HEIGHT: u32 = 500;

fn main() {
    let args: Vec<String> = env::args().collect();
    let earth_image_file_path = Path::new(&args[1]);

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

    let sphere = {
        let mut material = Material::default();
        material.color = color!(0.1, 1, 0.5);
        material.pattern = Some(Box::new(TextureMap::new(
            Box::new(UVCheckers::new(16., 8., black(), white())),
            Box::new(SphericalMap),
        )));
        material.diffuse = 0.7;
        material.specular = 0.3;
        Sphere::build(translation(-2.5, 1.3, 3.), material)
    };

    let earth_display = {
        let file = File::open(earth_image_file_path).unwrap();
        let canvas = canvas_from_ppm(file).unwrap();

        let mut material = Material::default();
        material.pattern = Some(Box::new(TextureMap::new(
            Box::new(UVImage::new(canvas)),
            Box::new(SphericalMap),
        )));
        material.diffuse = 0.9;
        material.specular = 0.1;
        material.shininess = 10.;
        material.ambient = 0.1;
        // lift the model so that it sits on the pedestal
        let earth = Sphere::build(
            &translation(0., 1., 0.) * &(&rotation_x(-0.5) * &rotation_y(-1.5)),
            material,
        );

        let pedestal = get_pedestal();

        let mut g = GroupShape::with_children(vec![Box::new(earth), Box::new(pedestal)]);
        g.set_transformation(translation(-0.2, 0.15, 0.5));

        g
    };

    let cylinder = {
        let mut material = Material::default();
        material.ambient = 0.1;
        material.specular = 0.6;
        material.shininess = 15.;
        material.diffuse = 0.8;
        material.pattern = Some(Box::new(TextureMap::new(
            Box::new(UVCheckers::new(16., 16., color!(0, 0.5, 0), white())),
            Box::new(CylindricalMap),
        )));

        // y scaling by PI is always required for cylinders to get proper-looking squares
        let mut c = Cylinder::build(translation(2., 2., 2.), material);
        c.maximum_y = 3.;
        c.minimum_y = -3.;
        c
    };

    let cube = {
        let mut material = Material::default();
        let pattern = get_align_check_cubic_map_pattern();
        material.pattern = Some(Box::new(pattern));

        let mut c = Cube::new();
        c.set_transformation(&translation(5., 2., 2.) * &rotation_x(-PI / 4.));
        c.set_material(material);

        c
    };

    let world = World {
        objects: vec![
            Box::new(floor),
            Box::new(sphere),
            Box::new(cylinder),
            Box::new(cube),
            Box::new(earth_display),
        ],
        // The light source is white, shining from above and to the left
        light: Some(get_light()),
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

fn get_light() -> Box<dyn Light> {
    Box::new(RectangleLight::new(
        color!(1.5, 1.5, 1.5),
        point!(-10, 10, -10),
        vector!(2, 0, 0),
        10,
        vector!(0, 2, 0),
        10,
        None,
    ))
}
