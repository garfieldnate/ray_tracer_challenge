use crate::matrix::Matrix;
use crate::tuple::Tuple;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
    // 1/x, 1/y, 1/z cached for faster intersection calculations later
    pub direction_inverses: Tuple,
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Self {
        debug_assert!(origin.is_point());
        debug_assert!(direction.is_vector());
        let direction_inverses = vector!(1. / direction.x, 1. / direction.y, 1. / direction.z);
        Ray {
            origin,
            direction,
            direction_inverses,
        }
    }
    pub fn position(&self, distance: f32) -> Tuple {
        self.origin + self.direction * distance
    }
    pub fn transform(&self, transform_matrix: &Matrix) -> Ray {
        Self::new(
            transform_matrix * &self.origin,
            transform_matrix * &self.direction,
        )
    }
    // derivation: think of a rhombus shape sitting on point on the surface, with the
    // bottom left and right sides being the incoming and reflected vectors and
    // the surface normal pointing to the middle of the rhombus.
    // To find the reflected vector from the incoming vector, project
    // the incoming vector onto the surface normal, then double the resulting vector's height to get the
    // the top point of the rhombus. Finally, subtract the incoming vector from this top
    // point to get the left side of the rhombus, or the reflected vector.
    // This gives us 2 * projection * normal - incoming. The sign needs to be flipped
    // to get the reflection direction right, though, so we have
    // incoming - 2 * projection * normal.
    pub fn reflect(in_vector: Tuple, normal_vector: Tuple) -> Tuple {
        -(normal_vector * 2.0 * in_vector.dot(normal_vector) - in_vector)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shape::shape::Shape;
    use crate::shape::sphere::Sphere;
    use crate::transformations::scaling;
    use crate::transformations::translation;
    use std::f32::consts::FRAC_1_SQRT_2;

    #[test]
    fn basic_ray_creation() {
        let origin = point!(1, 2, 3);
        let direction = vector!(4, 5, 6);
        let r = Ray::new(origin, direction);
        assert_eq!(r.origin, origin);
        assert_eq!(r.direction, direction);
    }

    #[test]
    fn compute_point_from_distance() {
        let r = Ray::new(point!(2, 3, 4), vector!(1, 0, 0));
        assert_eq!(r.position(0.0), point!(2, 3, 4));
        assert_eq!(r.position(1.0), point!(3, 3, 4));
        assert_eq!(r.position(-1.0), point!(1, 3, 4));
        assert_eq!(r.position(2.5), point!(4.5, 3, 4));
    }

    #[test]
    fn ray_intersects_sphere_at_two_points() {
        let r = Ray::new(point!(0, 0, -5), vector!(0, 0, 1));
        let s = Sphere::new();
        let intersections = s.intersect(r);
        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[0].distance, 4.0);
        assert_eq!(intersections[1].distance, 6.0);
    }

    #[test]
    fn ray_intersects_sphere_at_tangent() {
        let r = Ray::new(point!(0, 1, -5), vector!(0, 0, 1));
        let s = Sphere::new();
        let intersections = s.intersect(r);
        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[0].distance, 5.0);
        assert_eq!(intersections[1].distance, 5.0);
    }

    #[test]
    fn ray_misses_sphere() {
        let r = Ray::new(point!(0, 2, -5), vector!(0, 0, 1));
        let s = Sphere::new();
        let intersections = s.intersect(r);
        assert!(intersections.is_empty());
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let r = Ray::new(point!(0, 0, 0), vector!(0, 0, 1));
        let s = Sphere::new();
        let intersections = s.intersect(r);
        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[0].distance, -1.0);
        assert_eq!(intersections[1].distance, 1.0);
    }

    #[test]
    fn sphere_is_behind_ray() {
        let r = Ray::new(point!(0, 0, 5), vector!(0, 0, 1));
        let s = Sphere::new();
        let intersections = s.intersect(r);
        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[0].distance, -6.0);
        assert_eq!(intersections[1].distance, -4.0);
    }

    #[test]
    fn ray_translation() {
        let r = Ray::new(point!(1, 2, 3), vector!(0, 1, 0));
        let m = translation(3.0, 4.0, 5.0);
        let r2 = r.transform(&m);
        assert_eq!(r2.origin, point!(4, 6, 8));
        assert_eq!(r2.direction, vector!(0, 1, 0));
    }

    #[test]
    fn ray_scaling() {
        let r = Ray::new(point!(1, 2, 3), vector!(0, 1, 0));
        let m = scaling(2.0, 3.0, 4.0);
        let r2 = r.transform(&m);
        assert_eq!(r2.origin, point!(2, 6, 12));
        assert_eq!(r2.direction, vector!(0, 3, 0));
    }

    #[test]
    fn reflect_vector_approaching_at_45_degrees() {
        let v = vector!(1, -1, 0);
        let n = vector!(0, 1, 0);
        let r = Ray::reflect(v, n);
        assert_eq!(r, vector!(1, 1, 0));
    }

    #[test]
    fn reflect_vector_off_slanted_surface() {
        let v = vector!(0, -1, 0);
        let n = vector!(FRAC_1_SQRT_2, FRAC_1_SQRT_2, 0);
        let r = Ray::reflect(v, n);
        assert_abs_diff_eq!(r, vector!(1, 0, 0));
    }
}
