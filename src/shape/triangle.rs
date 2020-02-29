use crate::intersection::Intersection;
use crate::ray::Ray;
use crate::shape::base_shape::BaseShape;
use crate::shape::shape::Shape;
use crate::tuple::Tuple;

// Base shape has radius of 1 and straddles world origin

#[derive(Debug)]
pub struct Triangle {
    base: BaseShape,
    p1: Tuple,
    p2: Tuple,
    p3: Tuple,
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
        vec![]
    }
    fn local_norm_at(&self, object_point: Tuple) -> Tuple {
        self.normal
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn triangle_construction() {
        let p1 = point!(0, 1, 0);
        let p2 = point!(-1, 0, 0);
        let p3 = point!(1, 0, 0);
        let t = Triangle::new(p1, p2, p3);

        assert_eq!(t.p1, p1);
        assert_eq!(t.p2, p2);
        assert_eq!(t.p3, p3);
        assert_eq!(t.e1, vector!(-1, -1, 0));
        assert_eq!(t.e2, vector!(1, -1, 0));
        assert_eq!(t.normal, vector!(0, 0, -1));
    }

    #[test]
    fn triangle_normal() {
        let p1 = point!(0, 1, 0);
        let p2 = point!(-1, 0, 0);
        let p3 = point!(1, 0, 0);
        let t = Triangle::new(p1, p2, p3);

        let n1 = t.local_norm_at(point!(0, 0.5, 0));
        let n2 = t.local_norm_at(point!(-0.5, 0.75, 0));
        let n3 = t.local_norm_at(point!(0.5, 0.25, 0));

        // t.normal should always be used for triangle's normal
        assert_eq!(n1, t.normal);
        assert_eq!(n2, t.normal);
        assert_eq!(n3, t.normal);
    }
}
