use crate::bounding_box::BoundingBox;
use crate::intersection::Intersection;
use crate::ray::Ray;
use crate::shape::base_shape::BaseShape;
use crate::shape::shape::Shape;
use crate::tuple::Tuple;

#[derive(Debug, Clone)]
pub struct Triangle {
    base: BaseShape,
    pub p1: Tuple,
    pub p2: Tuple,
    pub p3: Tuple,
    e1: Tuple,
    e2: Tuple,
    normal: Tuple,
}

impl Triangle {
    pub fn new(p1: Tuple, p2: Tuple, p3: Tuple) -> Self {
        let e1 = p2 - p1;
        let e2 = p3 - p1;
        let normal = e2.cross(e1).norm();
        Triangle {
            base: BaseShape::new(),
            p1,
            p2,
            p3,
            e1,
            e2,
            normal,
        }
    }
}

impl Shape for Triangle {
    fn get_base(&self) -> &BaseShape {
        &self.base
    }

    fn get_base_mut(&mut self) -> &mut BaseShape {
        &mut self.base
    }

    fn local_intersect(&self, object_ray: Ray) -> Vec<Intersection> {
        // Get a vector that's orthogonal to both the incoming ray and one of the edges
        let dir_cross_e2 = object_ray.direction.cross(self.e2);
        // Check the cosine between the other edge and this orthogonal vector
        let determinant = self.e1.dot(dir_cross_e2);
        // If the other edge is not even kind of orthogonal, then the ray is parallel
        // to the triangle face and will miss the triangle
        // TODO: should probably be a constant somewhere
        if determinant.abs() < 0.0000001 {
            return vec![];
        }

        // TODO: explain u and v
        // Ray misses p1-p3 edge. TODO: explain math
        let f = 1.0 / determinant;
        let p1_to_origin = object_ray.origin - self.p1;
        let u = f * p1_to_origin.dot(dir_cross_e2);
        if u < 0.0 || u > 1.0 {
            return vec![];
        }

        // Ray misses p2-p3 and p1-p2 edges. TODO: explain math
        let origin_cross_e1 = p1_to_origin.cross(self.e1);
        let v = f * object_ray.direction.dot(origin_cross_e1);
        if v < 0.0 || (u + v) > 1.0 {
            return vec![];
        }

        // Ray intersects the triangle. TODO: explain math
        let distance = f * self.e2.dot(origin_cross_e1);
        vec![Intersection::new_with_uv(distance, self, u, v)]
    }

    fn local_norm_at(&self, _object_point: Tuple, _hit: &Intersection) -> Tuple {
        // Normal is always the same, regardless of point on triangle
        self.normal
    }

    fn bounding_box(&self) -> BoundingBox {
        let mut b = BoundingBox::empty();
        b.add_point(self.p1);
        b.add_point(self.p2);
        b.add_point(self.p3);
        b
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::utils::dummy_intersection;

    fn default_triangle() -> Triangle {
        Triangle::new(point!(0, 1, 0), point!(-1, 0, 0), point!(1, 0, 0))
    }
    #[test]
    fn triangle_construction() {
        let t = default_triangle();

        assert_eq!(t.p1, point!(0, 1, 0));
        assert_eq!(t.p2, point!(-1, 0, 0));
        assert_eq!(t.p3, point!(1, 0, 0));
        assert_eq!(t.e1, vector!(-1, -1, 0));
        assert_eq!(t.e2, vector!(1, -1, 0));
        assert_eq!(t.normal, vector!(0, 0, -1));
    }

    #[test]
    fn triangle_normal() {
        let t = default_triangle();

        let n1 = t.local_norm_at(point!(0, 0.5, 0), &dummy_intersection(&t));
        let n2 = t.local_norm_at(point!(-0.5, 0.75, 0), &dummy_intersection(&t));
        let n3 = t.local_norm_at(point!(0.5, 0.25, 0), &dummy_intersection(&t));

        // t.normal should always be used for triangle's normal
        assert_eq!(n1, t.normal);
        assert_eq!(n2, t.normal);
        assert_eq!(n3, t.normal);
    }

    #[test]
    fn intersect_ray_parallel_to_triangle() {
        let t = default_triangle();
        let r = Ray::new(point!(0, -1, -2), vector!(0, 1, 0));
        let xs = t.local_intersect(r);
        assert!(xs.is_empty());
    }

    #[test]
    fn ray_misses_p1_p3_edge() {
        let t = default_triangle();
        let r = Ray::new(point!(1, 1, -2), vector!(0, 0, 1));
        let xs = t.local_intersect(r);
        assert!(xs.is_empty());
    }

    #[test]
    fn ray_misses_p1_p2_edge() {
        let t = default_triangle();
        let r = Ray::new(point!(-1, 1, -2), vector!(0, 0, 1));
        let xs = t.local_intersect(r);
        assert!(xs.is_empty());
    }

    #[test]
    fn ray_misses_p2_p3_edge() {
        let t = default_triangle();
        let r = Ray::new(point!(0, -1, -2), vector!(0, 0, 1));
        let xs = t.local_intersect(r);
        assert!(xs.is_empty());
    }

    #[test]
    fn ray_strikes_triangle() {
        let t = default_triangle();
        let r = Ray::new(point!(0, 0.5, -2), vector!(0, 0, 1));
        let xs = t.local_intersect(r);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].distance, 2.0);
    }

    #[test]
    fn triangle_bounding_box() {
        let p1 = point!(-3, 7, 2);
        let p2 = point!(6, 2, -4);
        let p3 = point!(2, -1, -1);
        let t = Triangle::new(p1, p2, p3);
        let b = t.bounding_box();
        assert_eq!(b.min, point!(-3, -1, -4));
        assert_eq!(b.max, point!(6, 7, 2));
    }
}
