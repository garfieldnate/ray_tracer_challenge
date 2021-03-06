use crate::bounding_box::BoundingBox;
use crate::intersection::Intersection;
use crate::ray::Ray;
use crate::shape::base_shape::BaseShape;
use crate::shape::shape::Shape;
use crate::shape::triangle::Triangle;
use crate::tuple::Tuple;

#[derive(Debug, Clone)]
pub struct SmoothTriangle {
    // visible for testing
    pub(crate) base: Triangle,
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

    fn local_norm_at(&self, _object_point: Tuple, hit: &Intersection) -> Tuple {
        // TODO: explain the math here. And why is the normal the same everywhere?
        self.n2 * hit.u + self.n3 * hit.v + self.n1 * (1. - hit.u - hit.v)
    }

    fn bounding_box(&self) -> BoundingBox {
        // TODO: this is totally wrong, but the text doesn't give the code for the smooth triangle case
        self.base.bounding_box()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::precompute_values;

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

    #[test]
    fn uses_u_and_v_to_interpolate_normal() {
        let t = default_smooth_triangle();
        let i = Intersection::new_with_uv(1.0, &t, 0.45, 0.25);
        let n = t.normal_at(&point!(0, 0, 0), &i);
        assert_abs_diff_eq!(n, vector!(-0.5547002, 0.8320504, 0.0));
    }

    #[test]
    fn u_and_v_propagated_by_prepare_computations() {
        let t = default_smooth_triangle();
        let i = Intersection::new_with_uv(1.0, &t, 0.45, 0.25);
        let r = Ray::new(point!(-0.2, 0.3, -2), vector!(0, 0, 1));
        let xs = vec![i];
        let comps = precompute_values(r, &i, &xs);
        assert_abs_diff_eq!(comps.surface_normal, vector!(-0.5547002, 0.8320504, 0.0));
    }
}
