// 1 argument: path to directory of PPM textures for sky box
// In the demo, we use an earth texture downloaded from http://www.humus.name/Textures/LancellottiChapel.zip
// Each image should be converted to PPM format. This can be done withe ImageMagick:
// convert x.jpg -compress none x.ppm
use ray_tracer_challenge::camera::Camera;
use ray_tracer_challenge::canvas::canvas_from_ppm;
use ray_tracer_challenge::color::Color;
use ray_tracer_challenge::constants::DEFAULT_RAY_RECURSION_DEPTH;
use ray_tracer_challenge::light::light::Light;
use ray_tracer_challenge::light::point_light::PointLight;
use ray_tracer_challenge::material::Material;
use ray_tracer_challenge::pattern::uv::CubicMap;
use ray_tracer_challenge::pattern::uv::UVImage;
use ray_tracer_challenge::shape::cube::Cube;
use ray_tracer_challenge::shape::sphere::Sphere;
use ray_tracer_challenge::transformations::view_transform;
use ray_tracer_challenge::transformations::{scaling, translation};
use ray_tracer_challenge::tuple::Tuple;
use ray_tracer_challenge::world::World;
use ray_tracer_challenge::{color, point, vector};
use std::{env, fs::File, path::Path};

const CANVAS_WIDTH: u32 = 800;
const CANVAS_HEIGHT: u32 = 400;

fn main() {
    let args: Vec<String> = env::args().collect();
    let skybox_image_directory = Path::new(&args[1]);

    //     - add: sphere
    //   transform:
    //     - [ scale, 0.75, 0.75, 0.75 ]
    //     - [ translate, 0, 0, 5 ]
    //   material:
    //     diffuse: 0.4
    //     specular: 0.6
    //     shininess: 20
    //     reflective: 0.6
    //     ambient: 0
    let sphere = {
        let mut material = Material::default();
        material.diffuse = 0.4;
        material.specular = 0.6;
        material.shininess = 20.;
        material.reflective = 0.6;
        material.ambient = 0.;
        Sphere::build(
            &scaling(0.75, 0.75, 0.75) * &translation(0., 0., 5.),
            material,
        )
    };

    //   transform:
    //     - [ scale, 1000, 1000, 1000 ]
    //   material:
    //     pattern:
    //       type: map
    //       mapping: cube
    //       left:
    //         type: image
    //         file: negx.ppm
    //       right:
    //         type: image
    //         file: posx.ppm
    //       front:
    //         type: image
    //         file: posz.ppm
    //       back:
    //         type: image
    //         file: negz.ppm
    //       up:
    //         type: image
    //         file: posy.ppm
    //       down:
    //         type: image
    //         file: negy.ppm
    //     diffuse: 0
    //     specular: 0
    //     ambient: 1
    let skybox = {
        let mut material = Material::default();
        material.diffuse = 0.;
        material.specular = 0.;
        material.ambient = 1.;

        eprintln!("Loading front...");
        let front = get_uv_from_path(&skybox_image_directory.join("posz.ppm"));
        eprintln!("Loading back...");
        let back = get_uv_from_path(&skybox_image_directory.join("negz.ppm"));
        eprintln!("Loading left...");
        let left = get_uv_from_path(&skybox_image_directory.join("posx.ppm"));
        eprintln!("Loading right...");
        let right = get_uv_from_path(&skybox_image_directory.join("negx.ppm"));
        eprintln!("Loading up...");
        let up = get_uv_from_path(&skybox_image_directory.join("posy.ppm"));
        eprintln!("Loading down...");
        let down = get_uv_from_path(&skybox_image_directory.join("negy.ppm"));

        material.pattern = Some(Box::new(CubicMap::new(front, back, left, right, up, down)));

        Cube::build(scaling(1000., 1000., 1000.), material)
    };

    let world = World {
        objects: vec![Box::new(sphere), Box::new(skybox)],
        light: Some(get_light()),
    };

    let camera = Camera::new(
        CANVAS_WIDTH,
        CANVAS_HEIGHT,
        1.2,
        view_transform(point!(0, 0, 0), point!(0, 0, 5), vector!(0, 1, 0)),
    );

    let canvas = camera.render(world, DEFAULT_RAY_RECURSION_DEPTH);
    println!("{}", canvas.to_ppm());
}

fn get_light() -> Box<dyn Light> {
    Box::new(PointLight::new(point!(0, 100, 0), color!(1, 1, 1)))
}

fn get_uv_from_path(path: &Path) -> Box<UVImage> {
    let file = File::open(path).unwrap();
    let canvas = canvas_from_ppm(file).unwrap();
    Box::new(UVImage::new(canvas))
}
