use crate::canvas::build_canvas;
use crate::canvas::Canvas;
use crate::matrix::Matrix;
use crate::ray::build_ray;
use crate::ray::Ray;
use crate::tuple::{build_tuple, Tuple};
use crate::world::World;

pub struct Camera {
    // in pixels
    width_pixels: u32,
    height_pixels: u32,
    // in radians
    field_of_view: f32,
    // world space units
    half_width_world: f32,
    half_height_world: f32,
    pixel_size: f32,

    transform: Matrix,
}

pub fn build_camera(
    width_pixels: u32,
    height_pixels: u32,
    field_of_view: f32,
    transform: Matrix,
) -> Camera {
    // calculate size of a pixel on the canvas using the fact that the canvas is 1 unit in front of the eye.
    // Half of the canvas width can be found using trig: cut the canvas in half, forming a right triangle between
    // the eye, the half-width point of the canvas, and one horizontal edge of the canvas. The field of view
    // angle is bisected, and the eye-canvas corner is a right angle. Use sohcahtoa:
    // tangent is opposite/adjacent, or (half canvas width)/(distance to canvas). The distance to the canvas is 1,
    // so tangent of half of the field of view angle will equal half the width of the canvas.
    let half_view = (field_of_view / 2.0).tan();

    // TODO: I don't get what this is for. It seems like we pick the longer dimension to be the width
    // and the shorter to be the height. But wouldn't that turn the image sideways?
    let aspect_ratio = (width_pixels as f32) / (height_pixels as f32);
    let half_width_world: f32;
    let half_height_world: f32;
    if aspect_ratio >= 1.0 {
        half_width_world = half_view;
        half_height_world = half_view / aspect_ratio;
    } else {
        half_width_world = half_view * aspect_ratio;
        half_height_world = half_view;
    }
    let pixel_size = (half_width_world * 2.0) / width_pixels as f32;

    Camera {
        width_pixels,
        height_pixels,
        field_of_view,
        transform,
        half_width_world,
        half_height_world,
        pixel_size,
    }
}

impl Camera {
    pub fn ray_for_pixel(&self, x: u32, y: u32) -> Ray {
        // offset from edge of canvas to pixel's center
        let x_offset = (x as f32 + 0.5) * self.pixel_size;
        let y_offset = (y as f32 + 0.5) * self.pixel_size;
        // untransformed coordinates of the pixel in world space
        // camera looks toward -z, so +x is to the left
        let world_x = self.half_width_world - x_offset;
        let world_y = self.half_height_world - y_offset;
        // use camera matrix to transform the canvas point and the origin, then get ray's direction vector
        // canvas is located at z=-1
        let pixel: Tuple = &self.transform.inverse() * &point!(world_x, world_y, -1);
        let origin: Tuple = &self.transform.inverse() * &point!(0, 0, 0);
        let direction = (pixel - origin).norm();
        build_ray(origin, direction)
    }

    pub fn render(&self, world: World) -> Canvas {
        let mut canvas = build_canvas(self.width_pixels as usize, self.height_pixels as usize);
        for y in 0..self.height_pixels - 1 {
            for x in 0..self.width_pixels - 1 {
                let ray = self.ray_for_pixel(x, y);
                let color = world.color_at(ray);
                canvas.write_pixel(x as usize, y as usize, color);
            }
        }
        canvas
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::build_color;
    use crate::matrix::identity_4x4;
    use crate::transformations::rotation_y;
    use crate::transformations::translation;
    use crate::transformations::view_transform;
    use crate::world::default_world;
    use approx::AbsDiffEq;
    use std::f32::consts::FRAC_1_SQRT_2;
    use std::f32::consts::PI;

    #[test]
    fn horizontal_canvas_pixel_size() {
        let c = build_camera(200, 125, PI / 2.0, identity_4x4());
        assert_eq!(c.pixel_size, 0.01);
    }

    #[test]
    fn vertical_canvas_pixel_size() {
        let c = build_camera(125, 200, PI / 2.0, identity_4x4());
        assert_eq!(c.pixel_size, 0.01);
    }

    #[test]
    fn construct_ray_through_canvas_center() {
        let c = build_camera(201, 101, PI / 2.0, identity_4x4());
        let r = c.ray_for_pixel(100, 50);
        assert_eq!(r.origin, point!(0, 0, 0));
        assert_abs_diff_eq!(r.direction, vector!(0, 0, -1));
    }

    #[test]
    fn construct_ray_through_canvas_corner() {
        let c = build_camera(201, 101, PI / 2.0, identity_4x4());
        let r = c.ray_for_pixel(0, 0);
        assert_eq!(r.origin, point!(0, 0, 0));
        assert_abs_diff_eq!(r.direction, vector!(0.6651864, 0.33259323, -0.66851234));
    }

    #[test]
    fn construct_ray_with_transformed_camera() {
        let c = build_camera(
            201,
            101,
            PI / 2.0,
            &rotation_y(PI / 4.0) * &translation(0.0, -2.0, 5.0),
        );
        let r = c.ray_for_pixel(100, 50);
        // higher epsilon for more floating point calculations
        assert!(r
            .origin
            .abs_diff_eq(&point!(0, 2, -5), 10.0 * f32::default_epsilon()));
        assert_abs_diff_eq!(r.direction, vector!(FRAC_1_SQRT_2, 0, -FRAC_1_SQRT_2));
    }

    #[test]
    fn render_world() {
        let w = default_world();
        let from = point!(0, 0, -5);
        let to = point!(0, 0, 0);
        let up = vector!(0, 1, 0);
        let c = build_camera(11, 11, PI / 2.0, view_transform(from, to, up));
        let image = c.render(w);
        assert_abs_diff_eq!(
            image.pixel_at(5, 5),
            build_color(0.38066125, 0.4758265, 0.28549594)
        );
    }
}
