use crate::intersection::Intersection;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::shape::shape::BaseShape;
use crate::shape::shape::Shape;
use crate::tuple::Tuple;
use std::f32;

// Base shape is parallel to the Y-axis and infinitely long, centered on world origin

#[derive(Clone, Debug, PartialEq)]
pub struct Cylinder {
    base: BaseShape,
    pub minimum_y: f32,
    pub maximum_y: f32,
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
        }
    }
}

impl Shape for Cylinder {
    fn local_intersect(&self, object_ray: Ray) -> Vec<Intersection> {
        let mut intersections: Vec<Intersection> = Vec::with_capacity(2);
        self.intersect_sides(&object_ray, &mut intersections);
        intersections
    }

    // norms at the corners are the norms of one of the adjacent sides
    fn local_norm_at(&self, object_point: Tuple) -> Tuple {
        vector!(object_point.x, 0, object_point.z)
    }

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

impl Cylinder {
    fn intersect_sides<'a>(&'a self, object_ray: &Ray, intersections: &mut Vec<Intersection<'a>>) {
        // println!("ray: {:?}", object_ray);
        let two_a = 2.0 * (object_ray.direction.x.powi(2) + object_ray.direction.z.powi(2));
        // println!("two_a: {:?}", two_a);
        // TODO: turn this into shared constant somewhere?
        if two_a < 0.00000001 {
            // ray is parallel to y axis, so it won't intersect the cyllinder
            return;
        }
        let b = 2.0
            * (object_ray.origin.x * object_ray.direction.x
                + object_ray.origin.z * object_ray.direction.z);
        // println!("b: {:?}", b);
        let c = object_ray.origin.x.powi(2) + object_ray.origin.z.powi(2) - 1.0;
        // println!("c: {:?}", c);
        let discriminant = b.powi(2) - 2.0 * two_a * c;
        // println!("discriminant: {:?}", discriminant);

        if discriminant < 0.0 {
            //ray does not intersect cylinder
            return;
        }

        // Jingle all the way!
        let discriminant_sqrt = discriminant.sqrt();
        // println!("disc sqrt: {:?}", discriminant_sqrt);
        let distance1 = (-b - discriminant_sqrt) / two_a;
        // println!("d1: {:?}", distance1);
        let distance2 = (-b + discriminant_sqrt) / two_a;
        // println!("d2: {:?}", distance2);

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
    fn ray_intersects_cylinder() {
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
                6.808006,
                7.0886984,
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
    fn cylinder_normal_vector() {
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
}
