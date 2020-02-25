use crate::intersection::Intersection;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::shape::base_shape::BaseShape;
use crate::shape::shape::Shape;
use crate::tuple::Tuple;
use std::f32;

// Base shape is parallel to the Y-axis and infinitely long, centered on world origin

#[derive(Clone, Debug, PartialEq)]
pub struct Cylinder {
    base: BaseShape,
    pub minimum_y: f32,
    pub maximum_y: f32,
    pub closed: bool,
}

impl Cylinder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(transform: Matrix, material: Material) -> Self {
        let mut s = Cylinder::new();
        s.set_transformation(transform);
        s.set_material(material);
        s
    }
}

impl Default for Cylinder {
    fn default() -> Self {
        Cylinder {
            base: BaseShape::new(),
            minimum_y: f32::NEG_INFINITY,
            maximum_y: f32::INFINITY,
            closed: false,
        }
    }
}

impl Shape for Cylinder {
    fn get_base(&self) -> &BaseShape {
        &self.base
    }
    fn get_base_mut(&mut self) -> &mut BaseShape {
        &mut self.base
    }
    fn local_intersect(&self, object_ray: Ray) -> Vec<Intersection> {
        let mut intersections: Vec<Intersection> = Vec::with_capacity(2);
        self.intersect_sides(&object_ray, &mut intersections);
        if intersections.len() < 2 {
            self.intersect_caps(&object_ray, &mut intersections);
        }
        intersections
    }

    // norms at the corners are the norms of one of the adjacent sides
    fn local_norm_at(&self, object_point: Tuple) -> Tuple {
        let dist_square = object_point.x.powi(2) + object_point.z.powi(2);
        if dist_square < 1.0 {
            if object_point.y >= self.maximum_y - CLOSE_TO_ZERO {
                return vector!(0, 1, 0);
            } else if object_point.y <= self.minimum_y + CLOSE_TO_ZERO {
                return vector!(0, -1, 0);
            }
        }
        vector!(object_point.x, 0, object_point.z)
    }
}

const CLOSE_TO_ZERO: f32 = 0.000_001;
impl Cylinder {
    fn intersect_sides<'a>(&'a self, object_ray: &Ray, intersections: &mut Vec<Intersection<'a>>) {
        let two_a = 2.0 * (object_ray.direction.x.powi(2) + object_ray.direction.z.powi(2));
        // TODO: turn this into shared constant somewhere?
        // TODO: add test for negative small two_a value (forgot abs() before since book doesn't use this epsilon thingy)
        if two_a.abs() < CLOSE_TO_ZERO {
            // ray is parallel to y axis, so it won't intersect the cyllinder
            return;
        }
        let b = 2.0
            * (object_ray.origin.x * object_ray.direction.x
                + object_ray.origin.z * object_ray.direction.z);
        let c = object_ray.origin.x.powi(2) + object_ray.origin.z.powi(2) - 1.0;
        let discriminant = b.powi(2) - 2.0 * two_a * c;

        if discriminant < 0.0 {
            //ray does not intersect cylinder
            return;
        }

        // Jingle all the way!
        let discriminant_sqrt = discriminant.sqrt();
        let distance1 = (-b - discriminant_sqrt) / two_a;
        let distance2 = (-b + discriminant_sqrt) / two_a;

        let (distance1, distance2) = if distance1 > distance2 {
            (distance2, distance1)
        } else {
            (distance1, distance2)
        };

        let y1 = object_ray.origin.y + distance1 * object_ray.direction.y;
        if self.minimum_y < y1 && y1 < self.maximum_y {
            intersections.push(Intersection::new(distance1, self));
        }
        let y2 = object_ray.origin.y + distance2 * object_ray.direction.y;
        if self.minimum_y < y2 && y2 < self.maximum_y {
            intersections.push(Intersection::new(distance2, self));
        }
    }

    // check if the intersection at distance is within the radius (1) from the y axis
    fn check_cap(ray: &Ray, distance: f32) -> bool {
        let x = ray.origin.x + distance * ray.direction.x;
        let z = ray.origin.z + distance * ray.direction.z;
        // TODO: the book didn't use an epsilon. Maybe switching to f64 everywhere would fix this?
        (x.powi(2) + z.powi(2)) <= 1.0 + CLOSE_TO_ZERO
    }

    // add intersections with the end caps of the cylinder to intersections
    fn intersect_caps<'a>(&'a self, object_ray: &Ray, intersections: &mut Vec<Intersection<'a>>) {
        // don't bother checking for intersection if the cylinder isn't close
        // TODO: book says we should also have `|| object_ray.direction.y <= CLOSE_TO_ZERO` here.
        // That makes no sense, though, right? A vertical ray can intersect both caps. Maybe report as
        // error?
        if !self.closed {
            return;
        }

        // TODO: cache ray direction inverses
        let distance = (self.minimum_y - object_ray.origin.y) / object_ray.direction.y;
        if Cylinder::check_cap(&object_ray, distance) {
            intersections.push(Intersection::new(distance, self));
        }
        let distance = (self.maximum_y - object_ray.origin.y) / object_ray.direction.y;
        if Cylinder::check_cap(&object_ray, distance) {
            intersections.push(Intersection::new(distance, self));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::AbsDiffEq;

    #[test]
    fn ray_misses_cylinder() {
        let c = Cylinder::new();
        let test_data = vec![
            ("Parallel outside", point!(1, 0, 0), vector!(0, 1, 0)),
            ("Parallel inside", point!(0, 0, 0), vector!(0, 1, 0)),
            ("Skew", point!(0, 0, -5), vector!(1, 1, 1)),
        ];
        for (name, origin, direction) in test_data {
            let r = Ray::new(origin, direction.norm());
            let xs = c.local_intersect(r);
            assert!(
                xs.is_empty(),
                "{}: should find 0 intersections but found {}: {:?}",
                name,
                xs.len(),
                xs
            );
        }
    }

    #[test]
    fn ray_intersects_cylinder_sides() {
        let c = Cylinder::new();
        let test_data = vec![
            ("tangent", point!(1, 0, -5), vector!(0, 0, 1), 5.0, 5.0),
            (
                "perpendicular",
                point!(0, 0, -5),
                vector!(0, 0, 1),
                4.0,
                6.0,
            ),
            (
                "angle",
                point!(0.5, 0, -5),
                vector!(0.1, 1, 1),
                6.808_006,
                7.088_698_4,
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
    fn ray_intersects_constrained_cylinder() {
        let c = {
            let mut c = Cylinder::new();
            c.minimum_y = 1.0;
            c.maximum_y = 2.0;
            c
        };
        let test_data = vec![
            (
                "Diagonal ray inside that exits before hitting sides",
                point!(0, 1.5, 0),
                vector!(0.1, 1, 0),
                0,
            ),
            (
                "Perpendicular but above y max",
                point!(0, 3, -5),
                vector!(0, 0, 1),
                0,
            ),
            (
                "Perpendicular but below y min",
                point!(0, 0, -5),
                vector!(0, 0, 1),
                0,
            ),
            (
                "Max y should be outside bounds",
                point!(0, 2, -5),
                vector!(0, 0, 1),
                0,
            ),
            (
                "Min y should be outside bounds",
                point!(0, 1, -5),
                vector!(0, 0, 1),
                0,
            ),
            (
                "Ray through middle should intersect twice",
                point!(0, 1.5, -2),
                vector!(0, 0, 1),
                2,
            ),
        ];
        for (name, origin, direction, expected_num_intersections) in test_data {
            let r = Ray::new(origin, direction.norm());
            let xs = c.local_intersect(r);
            assert_eq!(xs.len(), expected_num_intersections, "{}", name);
        }
    }

    #[test]
    fn ray_intersects_caps_of_closed_cylinder() {
        let c = {
            let mut c = Cylinder::new();
            c.minimum_y = 1.0;
            c.maximum_y = 2.0;
            c.closed = true;
            c
        };
        let test_data = vec![
            (
                "Ray pointing down center",
                point!(0, 3, 0),
                vector!(0, -1, 0),
                2,
            ),
            (
                "Diagonal from above through center",
                point!(0, 3, -2),
                vector!(0, -1, 2),
                2,
            ),
            (
                "Diagonal from above through center, exit at border between side and cap",
                point!(0, 4, -2),
                vector!(0, -1, 1),
                2,
            ), // corner case
            (
                "Diagonal from below through center",
                point!(0, 0, -2),
                vector!(0, 1, 2),
                2,
            ),
            (
                "Diagonal from below through center, exit at border between side and cap",
                point!(0, -1, -2),
                vector!(0, 1, 1),
                2,
            ), // corner case
        ];
        for (name, origin, direction, expected_num_intersections) in test_data {
            let r = Ray::new(origin, direction.norm());
            let xs = c.local_intersect(r);
            assert_eq!(xs.len(), expected_num_intersections, "{}", name);
        }
    }

    #[test]
    fn normal_vector_cylinder_sides() {
        let c = Cylinder::new();
        let test_data = vec![
            ("+x", point!(1, 0, 0), vector!(1, 0, 0)),
            ("-z", point!(0, 5, -1), vector!(0, 0, -1)),
            ("+z", point!(0, -2, 1), vector!(0, 0, 1)),
            ("-x", point!(-1, 1, 0), vector!(-1, 0, 0)),
        ];
        for (name, point, expected_normal) in test_data {
            let normal = c.local_norm_at(point);
            assert_eq!(normal, expected_normal, "{}", name);
        }
    }

    #[test]
    fn normal_vector_cylinder_caps() {
        let c = {
            let mut c = Cylinder::new();
            c.minimum_y = 1.0;
            c.maximum_y = 2.0;
            c.closed = true;
            c
        };
        let test_data = vec![
            ("-y at bottom center", point!(0, 1, 0), vector!(0, -1, 0)),
            ("-y at bottom right", point!(0.5, 1, 0), vector!(0, -1, 0)),
            ("-y at bottom front", point!(0, 1, 0.5), vector!(0, -1, 0)),
            ("+y at top center", point!(0, 2, 0), vector!(0, 1, 0)),
            ("+y at top right", point!(0.5, 2, 0), vector!(0, 1, 0)),
            ("+y at top front", point!(0, 2, 0.5), vector!(0, 1, 0)),
        ];
        for (name, point, expected_normal) in test_data {
            let normal = c.local_norm_at(point);
            assert_eq!(normal, expected_normal, "{}", name);
        }
    }
}
