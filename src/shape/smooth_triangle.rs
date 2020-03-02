use crate::intersection::Intersection;
use crate::ray::Ray;
use crate::shape::base_shape::BaseShape;
use crate::shape::shape::Shape;
use crate::shape::triangle::Triangle;
use crate::tuple::Tuple;

#[derive(Debug)]
pub struct SmoothTriangle {
    base: Triangle,
    // normal vectors at each corner
    pub n1: Tuple,
    pub n2: Tuple,
    pub n3: Tuple,
}

impl SmoothTriangle {
    pub fn new(p1: Tuple, p2: Tuple, p3: Tuple, n1: Tuple, n2: Tuple, n3: Tuple) -> Self {
        SmoothTriangle {
            base: Triangle::new(p1, p2, p3),
            n1,
            n2,
            n3,
        }
    }
}

impl Shape for SmoothTriangle {
    fn get_base(&self) -> &BaseShape {
        &self.base.get_base()
    }

    fn get_base_mut(&mut self) -> &mut BaseShape {
        self.base.get_base_mut()
    }

    fn local_intersect(&self, object_ray: Ray) -> Vec<Intersection> {
        self.base.local_intersect(object_ray)
    }

    fn local_norm_at(&self, object_point: Tuple) -> Tuple {
        vector!(0, 0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_smooth_triangle() -> SmoothTriangle {
        SmoothTriangle::new(
            point!(0, 1, 0),
            point!(-1, 0, 0),
            point!(1, 0, 0),
            vector!(0, 1, 0),
            vector!(-1, 0, 0),
            vector!(1, 0, 0),
        )
    }

    #[test]
    fn smooth_triangle_construction() {
        let t = default_smooth_triangle();

        assert_eq!(t.base.p1, point!(0, 1, 0));
        assert_eq!(t.base.p2, point!(-1, 0, 0));
        assert_eq!(t.base.p3, point!(1, 0, 0));

        assert_eq!(t.n1, vector!(0, 1, 0));
        assert_eq!(t.n2, vector!(-1, 0, 0));
        assert_eq!(t.n3, vector!(1, 0, 0));
    }

    #[test]
    fn intersection_stores_u_and_v() {
        let t = default_smooth_triangle();
        let r = Ray::new(point!(-0.2, 0.3, -2), vector!(0, 0, 1));
        let xs = t.local_intersect(r);
        assert_eq!(xs[0].u, 0.45);
        assert_eq!(xs[0].v, 0.25);
    }
}
