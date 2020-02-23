use crate::intersection::Intersection;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::shape::shape::BaseShape;
use crate::shape::shape::Shape;
use crate::tuple::Tuple;
use std::f32;

// Base shape is a double-napped cone with tips meeting at the origin and extending vertically along the y axis.

#[derive(Clone, Debug, PartialEq)]
pub struct Cone {
    base: BaseShape,
    pub minimum_y: f32,
    pub maximum_y: f32,
    pub closed: bool,
}

impl Cone {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(transform: Matrix, material: Material) -> Self {
        let mut s = Cone::new();
        s.set_transformation(transform);
        s.set_material(material);
        s
    }
}

impl Default for Cone {
    fn default() -> Self {
        Cone {
            base: BaseShape::new(),
            minimum_y: f32::NEG_INFINITY,
            maximum_y: f32::INFINITY,
            closed: false,
        }
    }
}

impl Shape for Cone {
    fn local_intersect(&self, object_ray: Ray) -> Vec<Intersection> {}

    fn local_norm_at(&self, object_point: Tuple) -> Tuple {}

    // forward these to BaseShape (TODO: need delegation RFC to be accepted!)
    fn transformation(&self) -> &Matrix {
        &self.base.transformation()
    }
    fn set_transformation(&mut self, t: Matrix) {
        self.base.set_transformation(t);
    }
    fn material(&self) -> &Material {
        self.base.material()
    }
    fn set_material(&mut self, m: Material) {
        self.base.set_material(m);
    }
    fn casts_shadow(&self) -> bool {
        self.base.casts_shadow()
    }
    fn set_casts_shadow(&mut self, casts_shadow: bool) {
        self.base.set_casts_shadow(casts_shadow)
    }
    fn transformation_inverse(&self) -> &Matrix {
        self.base.transformation_inverse()
    }
    fn transformation_inverse_transpose(&self) -> &Matrix {
        self.base.transformation_inverse_transpose()
    }
}

const CLOSE_TO_ZERO: f32 = 0.000001;
impl Cone {
    fn intersect_sides<'a>(&'a self, object_ray: &Ray, intersections: &mut Vec<Intersection<'a>>) {}

    fn check_cap(ray: &Ray, distance: f32) -> bool {}

    // add intersections with the end caps of the cone to intersections
    fn intersect_caps<'a>(&'a self, object_ray: &Ray, intersections: &mut Vec<Intersection<'a>>) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::AbsDiffEq;
    use std::f32::consts::SQRT_2;

    // #[test]
    // fn ray_misses_cone() {
    //     TODO
    // }

    #[test]
    fn ray_intersects_cone() {
        let c = Cone::new();
        let test_data = vec![
            ("", point!(0, 0, -5), vector!(0, 0, 1), 5., 5.),
            ("", point!(0, 0, -5), vector!(1, 1, 1), 8.66025, 8.66025),
            (
                "",
                point!(1, 1, -5),
                vector!(-0.5, -1, 1),
                4.55006,
                49.44994,
            ),
        ];
        for (name, origin, direction, distance1, distance2) in test_data {
            let r = Ray::new(origin, direction.norm());
            let xs = c.local_intersect(r);
            assert_eq!(xs.len(), 2, "{}: should find 2 intersections", name);
            debug_assert!(
                xs[0]
                    .distance
                    .abs_diff_eq(&distance1, f32::default_epsilon()),
                "{}: distance to first intersection (expected {}, got {})",
                name,
                distance1,
                xs[0].distance
            );
            debug_assert!(
                xs[1]
                    .distance
                    .abs_diff_eq(&distance2, f32::default_epsilon()),
                "{}: distance to second intersection (expected {}, got {})",
                name,
                distance2,
                xs[1].distance
            );
        }
    }

    #[test]
    fn intersect_cone_with_ray_parallel_to_one_half() {
        let s = Cone::new();
        let r = Ray::new(point!(0, 0, -1), vector!(0, 1, 1).norm());
        let intersections = s.local_intersect(r);
        assert_eq!(intersections.len(), 1);
        assert_abs_diff_eq!(intersections[0].distance, 0.35355);
    }

    #[test]
    fn ray_intersects_caps_of_closed_cone() {
        let c = {
            let mut c = Cone::new();
            c.minimum_y = 1.0;
            c.maximum_y = 2.0;
            c.closed = true;
            c
        };
        let test_data = vec![
            ("", point!(0, 0, -5), vector!(0, 1, 0), 0),
            ("", point!(0, 0, -0.25), vector!(0, 1, 1), 2),
            ("", point!(0, 0, -0.25), vector!(0, 1, 0), 4),
        ];
        for (name, origin, direction, expected_num_intersections) in test_data {
            let r = Ray::new(origin, direction.norm());
            let xs = c.local_intersect(r);
            assert_eq!(xs.len(), expected_num_intersections, "{}", name);
        }
    }

    #[test]
    fn cone_normal_vector() {
        let c = Cone::new();
        let test_data = vec![
            ("", point!(0, 0, 0), vector!(0, 0, 0)),
            ("", point!(1, 1, 1), vector!(1, -SQRT_2, 1)),
            ("", point!(-1, -1, 0), vector!(-1, 1, 0)),
        ];
        for (name, point, expected_normal) in test_data {
            let normal = c.local_norm_at(point);
            assert_eq!(normal, expected_normal, "{}", name);
        }
    }
}
