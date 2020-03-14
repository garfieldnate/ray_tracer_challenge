use crate::bounding_box::BoundingBox;
use crate::intersection::Intersection;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::shape::base_shape::BaseShape;
use crate::shape::shape::Shape;
use crate::tuple::Tuple;
use std::f32;

// Base shape has a dimension of two and straddles the world origin

#[derive(Debug)]
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
    fn get_base(&self) -> &BaseShape {
        &self.base
    }
    fn get_base_mut(&mut self) -> &mut BaseShape {
        &mut self.base
    }
    // uses AABB. TODO: more documentation
    fn local_intersect(&self, object_ray: Ray) -> Vec<Intersection> {
        match aabb_intersection(object_ray) {
            Some((min_distance, max_distance)) => vec![
                Intersection::new(min_distance, self),
                Intersection::new(max_distance, self),
            ],
            None => vec![],
        }
    }

    // norms at the corners are the norms of one of the adjacent sides
    fn local_norm_at(&self, object_point: Tuple, _hit: &Intersection) -> Tuple {
        let (x_abs, y_abs, z_abs) = (
            object_point.x.abs(),
            object_point.y.abs(),
            object_point.z.abs(),
        );
        let max_c = x_abs.max(y_abs.max(z_abs));
        if x_abs == max_c {
            vector!(object_point.x, 0, 0)
        } else if y_abs == max_c {
            vector!(0, object_point.y, 0)
        } else {
            vector!(0, 0, object_point.z)
        }
    }

    fn bounding_box(&self) -> BoundingBox {
        BoundingBox {
            min: point!(-1, -1, -1),
            max: point!(1, 1, 1),
        }
    }
}

pub fn aabb_intersection(object_ray: Ray) -> Option<(f32, f32)> {
    // TODO: book says it's possible to return early sometimes
    // TODO: make it faster by replacing with this implementation: https://tavianator.com/fast-branchless-raybounding-box-intersections/
    let (min_x_distance, max_x_distance) = check_axis(object_ray.origin.x, object_ray.direction.x);
    let (min_y_distance, max_y_distance) = check_axis(object_ray.origin.y, object_ray.direction.y);
    let (min_z_distance, max_z_distance) = check_axis(object_ray.origin.z, object_ray.direction.z);

    // max of minimum and min of maximum plane intersections are
    // the actual cube intersections
    let min_distance = min_x_distance.max(min_y_distance.max(min_z_distance));
    let max_distance = max_x_distance.min(max_y_distance.min(max_z_distance));

    if min_distance > max_distance {
        // the min/max values get reversed only when the ray misses the cube
        None
    } else {
        Some((min_distance, max_distance))
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
    use crate::test::utils::dummy_intersection;
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

    #[test]
    fn cube_surface_normal() {
        let c = Cube::new();
        let test_data = vec![
            ("right side", point!(1, 0.5, -0.8), vector!(1, 0, 0)),
            ("left side", point!(-1, -0.2, 0.9), vector!(-1, 0, 0)),
            ("top side", point!(-0.4, 1, -0.1), vector!(0, 1, 0)),
            ("bottom side", point!(0.3, -1, -0.7), vector!(0, -1, 0)),
            ("front side", point!(-0.6, 0.3, 1), vector!(0, 0, 1)),
            ("back side", point!(0.4, 0.4, -1), vector!(0, 0, -1)),
            ("Top right front corner", point!(1, 1, 1), vector!(1, 0, 0)),
            (
                "Bottom back left corner",
                point!(-1, -1, -1),
                vector!(-1, 0, 0),
            ),
        ];
        for (name, point, expected_normal) in test_data {
            assert_eq!(
                c.local_norm_at(point, &dummy_intersection(&c)),
                expected_normal,
                "{}",
                name
            );
        }
    }
}
