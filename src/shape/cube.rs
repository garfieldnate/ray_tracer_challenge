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
        let (min_x_distance, max_x_distance) =
            check_axis(object_ray.origin.x, object_ray.direction.x);
        let (min_y_distance, max_y_distance) =
            check_axis(object_ray.origin.y, object_ray.direction.y);
        let (min_z_distance, max_z_distance) =
            check_axis(object_ray.origin.z, object_ray.direction.z);

        // max of minimum and min of maximum plane intersections are
        // the actual cube intersections
        let min_distance = min_x_distance.max(min_y_distance.max(min_z_distance));
        let max_distance = max_x_distance.min(max_y_distance.min(max_z_distance));

        if min_distance > max_distance {
            // the min/max values get reversed only when the ray misses the cube
            vec![]
        } else {
            vec![
                Intersection::new(min_distance, self),
                Intersection::new(max_distance, self),
            ]
        }
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

// return pair of distance values for intersecting two parallel planes of the cube;
// note that values can also be + and - infinity
fn check_axis(origin: f32, direction: f32) -> (f32, f32) {
    // the planes are offset at origin + and - 1
    let tmin_numerator = -1.0 - origin;
    let tmax_numerator = 1.0 - origin;

    let (tmin, tmax) = (tmin_numerator / direction, tmax_numerator / direction);

    if tmin > tmax {
        (tmax, tmin)
    } else {
        (tmin, tmax)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ray_intersects_cube() {
        let c = Cube::new();
        let test_data = vec![
            ("+x", point!(5, 0.5, 0), vector!(-1, 0, 0), 4.0, 6.0),
            ("-x", point!(-5, 0.5, 0), vector!(1, 0, 0), 4.0, 6.0),
            ("+y", point!(0.5, 5, 0), vector!(0, -1, 0), 4.0, 6.0),
            ("-y", point!(0.5, -5, 0), vector!(0, 1, 0), 4.0, 6.0),
            ("+z", point!(0.5, 0, 5), vector!(0, 0, -1), 4.0, 6.0),
            ("-z", point!(0.5, 0.5, -5), vector!(0, 0, 1), 4.0, 6.0),
            ("inside", point!(0, 0.5, 0), vector!(0, 0, 1), -1.0, 1.0),
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

    #[test]
    fn ray_misses_cube() {
        let c = Cube::new();
        let test_data = vec![
            (
                "diagonal 1",
                point!(-2, 0, 0),
                vector!(0.2673, 0.5345, 0.8018),
            ),
            (
                "diagonal 2",
                point!(0, -2, 0),
                vector!(0.8018, 0.2673, 0.5345),
            ),
            (
                "diagonal 3",
                point!(0, 0, -2),
                vector!(0.5345, 0.8018, 0.2673),
            ),
            ("parallel to z", point!(2, 0, 2), vector!(0, 0, -1)),
            ("parallel to y", point!(0, 2, 2), vector!(0, -1, 0)),
            ("parallel to x", point!(2, 2, 0), vector!(-1, 0, 0)),
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
}
