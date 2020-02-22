use crate::intersection::Intersection;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::shape::shape::BaseShape;
use crate::shape::shape::Shape;
use crate::tuple::Tuple;
use std::f32;

#[derive(Clone, Debug, PartialEq)]
pub struct Cube {
    base: BaseShape,
}

impl Cube {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(transform: Matrix, material: Material) -> Self {
        let mut s = Cube::new();
        s.set_transformation(transform);
        s.set_material(material);
        s
    }
}

impl Default for Cube {
    fn default() -> Self {
        Cube {
            base: BaseShape::new(),
        }
    }
}

impl Shape for Cube {
    fn local_intersect(&self, object_ray: Ray) -> Vec<Intersection> {
        vec![]
    }
    fn local_norm_at(&self, _object_point: Tuple) -> Tuple {
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
    fn ray_intersection() {
        let c = Cube::new();
        // let r = Ray::new(point!(0,0,0), direction: Tuple)
        let test_data = vec![
            ("+x", point!(5, 0.5, 0), vector!(-1, 0, 0), 4.0, 6.0),
            ("-x", point!(-5, 0.5, 0), vector!(1, 0, 0), 4.0, 6.0),
            ("+y", point!(0.5, 5, 0), vector!(0, -1, 0), 4.0, 6.0),
            ("-y", point!(0.5, 0, 0), vector!(0, 1, 0), 4.0, 6.0),
            ("+z", point!(0.5, 0, 5), vector!(0, 0, -1), 4.0, 6.0),
            ("-z", point!(0.5, 0.5, -5), vector!(0, 0, 1), 4.0, 6.0),
            ("insce", point!(0, 0.5, 0), vector!(0, 0, 1), -1.0, 1.0),
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
