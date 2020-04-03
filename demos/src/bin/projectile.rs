// Create image of projectile's path given initial velocity, gravity and wind
// TODO: take command line args to tweak environment and projectile parameters
// TODO: display resulting PPM file instead of just printing the text
use ray_tracer_challenge::canvas::Canvas;
use ray_tracer_challenge::color::Color;
use ray_tracer_challenge::tuple::Tuple;
use ray_tracer_challenge::{color, point, vector};

struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

fn tick(env: &Environment, proj: &Projectile) -> Projectile {
    let position = proj.position + proj.velocity;
    let velocity = proj.velocity + env.gravity + env.wind;
    Projectile { position, velocity }
}

fn main() {
    let start = point!(0, 1, 0);
    let velocity = vector!(1, 1.8, 0).norm() * 11.25;
    let mut proj = Projectile {
        position: start,
        velocity,
    };
    let gravity = vector!(0, -0.1, 0);
    let wind = vector!(-0.02, 0, 0);
    let environment = Environment { gravity, wind };
    let mut canvas = Canvas::new(700, 550);
    let trace_color = color!(0.53, 0.39, 0.074);

    while proj.position.y >= 0.0 && proj.position.x >= 0.0 {
        canvas.write_pixel(
            proj.position.x as usize,
            canvas.height - (proj.position.y as usize),
            trace_color,
        );
        proj = tick(&environment, &proj);
    }

    println!("{}", canvas.to_ppm());
}
