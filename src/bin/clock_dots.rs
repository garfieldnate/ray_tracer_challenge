// Create image of 12 points of an analog clock face
use ray_tracer_challenge::canvas::build_canvas;
use ray_tracer_challenge::color::build_color;
use ray_tracer_challenge::point;
use ray_tracer_challenge::transformations::*;
use ray_tracer_challenge::tuple::build_tuple;
use std::f32::consts::PI;

const CANVAS_SIZE: usize = 300;
fn main() {
    let mut canvas = build_canvas(CANVAS_SIZE, CANVAS_SIZE);
    let white = build_color(1.0, 1.0, 1.0);
    let translate_to_center =
        translation((canvas.height / 2) as f32, (canvas.height / 2) as f32, 0.0);
    let adjust_for_reversed_canvas_y =
        &translation(0.0, canvas.height as f32, 0.0) * &scaling(1.0, -1.0, 1.0);
    let display_transform = &adjust_for_reversed_canvas_y * &translate_to_center;

    let twelve_o_clock = point!(0, 100, 0);
    let rotate_one_hour = rotation_z(PI / 6.0);

    let mut hour = twelve_o_clock;
    for _num in 1..=12 {
        hour = &rotate_one_hour * &hour;
        // println!("{} o'clock: before display transform: {:?}", _num, hour);
        let display_point = &display_transform * &hour;
        // println!("{} o'clock: after display transform: {:?}", _num, display_hour);
        canvas.write_pixel(display_point.x as usize, display_point.y as usize, white);
    }

    println!("{}", canvas.to_ppm());
}
