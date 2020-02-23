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
        }
    }
}

impl Shape for Cylinder {
    fn local_intersect(&self, object_ray: Ray) -> Vec<Intersection> {
        println!("ray: {:?}", object_ray);
        let two_a = 2.0 * (object_ray.direction.x.powi(2) + object_ray.direction.z.powi(2));
        println!("two_a: {:?}", two_a);
        // TODO: turn this into shared constant somewhere?
        if two_a < 0.00000001 {
            // ray is parallel to y axis, so it won't intersect the cyllinder
            return vec![];
        }
        let b = 2.0
            * (object_ray.origin.x * object_ray.direction.x
                + object_ray.origin.z * object_ray.direction.z);
        println!("b: {:?}", b);
        let c = object_ray.origin.x.powi(2) + object_ray.origin.z.powi(2) - 1.0;
        println!("c: {:?}", c);
        let discriminant = b.powi(2) - 2.0 * two_a * c;
        println!("discriminant: {:?}", discriminant);

        if discriminant < 0.0 {
            //ray does not intersect cylinder
            return vec![];
        }

        // Jingle all the way!
        let discriminant_sqrt = discriminant.sqrt();
        println!("disc sqrt: {:?}", discriminant_sqrt);
        let distance1 = (-b - discriminant_sqrt) / two_a;
        println!("d1: {:?}", distance1);
        let distance2 = (-b + discriminant_sqrt) / two_a;
        println!("d2: {:?}", distance2);
        vec![
            Intersection::new(distance1, self),
            Intersection::new(distance2, self),
        ]
    }

    // norms at the corners are the norms of one of the adjacent sides
    fn local_norm_at(&self, object_point: Tuple) -> Tuple {
        vector!(0, 0, 0)
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ray_misses_cylinder() {
        let c = Cylinder::new();
        let test_data = vec![
            ("Parallel outside", point!(1, 0, 0), vector!(0, 1, 0)),
            ("Parallel inside", point!(0, 0, 0), vector!(0, 1, 0)),
            ("Skew", point!(0, 0, -5), vector!(1, 1, 1)),
        ];
        for (name, origin, direction) in test_data {
            let r = Ray::new(origin, direction);
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
            // TODO: this one doesn't pass and I don't know why. Driving me a little nuts. I'll come back once I have a visual.
            (
                "angle",
                point!(0.5, 0, -5),
                vector!(0.1, 1, 1),
                6.80798,
                7.08872,
            ),
        ];
        for (name, origin, direction, distance1, distance2) in test_data {
            let r = Ray::new(origin, direction);
            let xs = c.local_intersect(r);
            assert_eq!(xs.len(), 2, "{}: should find 2 intersections", name);
            assert_eq!(
                xs[0].distance, distance1,
                "{}: distance to first intersection",
                name
            );
            assert_eq!(
                xs[1].distance, distance2,
                "{}: distance to second intersection",
                name
            );
        }
    }
}
