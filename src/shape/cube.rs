use crate::bounding_box::BoundingBox;
use crate::intersection::Intersection;
use crate::intersection::IntersectionList;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::shape::base_shape::BaseShape;
use crate::shape::shape::Shape;
use crate::tuple::Tuple;
use std::f32;

// Base shape has a dimension of two and straddles the world origin

#[derive(Debug, Clone)]
pub struct Cube {
    base: BaseShape,
}

impl Cube {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn min_point() -> Tuple {
        point!(-1, -1, -1)
    }

    pub fn max_point() -> Tuple {
        point!(1, 1, 1)
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
    fn local_intersect(&self, object_ray: Ray) -> IntersectionList {
        match aabb_intersection(object_ray, Cube::min_point(), Cube::max_point()) {
            Some((min_distance, max_distance)) => IntersectionList::with_intersections(vec![
                Intersection::new(min_distance, self),
                Intersection::new(max_distance, self),
            ]),
            None => IntersectionList::empty(),
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

pub fn aabb_intersection(object_ray: Ray, min: Tuple, max: Tuple) -> Option<(f32, f32)> {
    // a branchless and divisionless implementation taken from tavianator:
    // https://tavianator.com/cgit/dimension.git/tree/libdimension/bvh/bvh.c

    // First calculate the distance the ray travels to hit the minimum and maximum bounds
    // of the box on the x axis. Note that if the ray's direction's x value is 0, the inverse
    // will be infinity and the max/min distances will be positive or negative infinity
    // (both represented nicely with IEEE floating point numbers).
    let min_x_distance = (min.x - object_ray.origin.x) * object_ray.direction_inverses.x;
    let max_x_distance = (max.x - object_ray.origin.x) * object_ray.direction_inverses.x;
    // calculate the min and max distances that the ray may travel; note that these values
    // may be reversed from the distances to the min and max box bounds. Using the min/max
    // logic here is apparently more efficient than an if statement with swap logic, since
    // CPU's can do comparison-conditional logic without the costs of regular branching.
    let mut min_distance = min_x_distance.min(max_x_distance);
    let mut max_distance = min_x_distance.max(max_x_distance);

    // repeat for distances to bounds in y axis
    let min_y_distance = (min.y - object_ray.origin.y) * object_ray.direction_inverses.y;
    let max_y_distance = (max.y - object_ray.origin.y) * object_ray.direction_inverses.y;
    min_distance = min_distance.max(min_y_distance.min(max_y_distance));
    max_distance = max_distance.min(min_y_distance.max(max_y_distance));

    // repeat for distances to bounds in z axis
    let min_z_distance = (min.z - object_ray.origin.z) * object_ray.direction_inverses.z;
    let max_z_distance = (max.z - object_ray.origin.z) * object_ray.direction_inverses.z;
    min_distance = min_distance.max(min_z_distance.min(max_z_distance));
    max_distance = max_distance.min(min_z_distance.max(max_z_distance));

    // The max distance has to be at least 0; if it's negative, then that means the ray
    // intersected in the direction that we are *not* casting it. In the case that the ray
    // was parallel to an axis and did not intersect the cube, either min_distance will be
    // infinity or max_distance will be negative infinity; both cases are automatically
    // handled with regular floating point number comparisons.
    if max_distance >= 0f32.max(min_distance) {
        Some((min_distance, max_distance))
    } else {
        None
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
            let xs = c.local_intersect(r).xs;
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
            (
                "ray is cast away from the cube",
                point!(0, 0, 2),
                vector!(0., 0., 1.),
            ),
            ("parallel to z", point!(2, 0, 2), vector!(0, 0, -1)),
            ("parallel to y", point!(0, 2, 2), vector!(0, -1, 0)),
            ("parallel to x", point!(2, 2, 0), vector!(-1, 0, 0)),
        ];
        for (name, origin, direction) in test_data {
            let r = Ray::new(origin, direction);
            let xs = c.local_intersect(r).xs;
            assert!(
                xs.is_empty(),
                "case {}: should find 0 intersections but found {}: {:?}",
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
