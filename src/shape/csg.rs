use crate::intersection::Intersection;
use crate::ray::Ray;
use crate::shape::base_shape::BaseShape;
use crate::shape::shape::Shape;
use crate::tuple::Tuple;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSGOperator {
    Union(),
    Intersection(),
    Difference(),
}

#[derive(Debug)]
pub struct CSG {
    base: BaseShape,
    op: CSGOperator,
    s1: Box<dyn Shape>,
    s2: Box<dyn Shape>,
}

impl CSG {
    pub fn new(op: CSGOperator, s1: Box<dyn Shape>, s2: Box<dyn Shape>) -> Self {
        CSG {
            base: BaseShape::new(),
            op,
            s1,
            s2,
        }
    }
}

impl Shape for CSG {
    fn get_base(&self) -> &BaseShape {
        &self.base
    }

    fn get_base_mut(&mut self) -> &mut BaseShape {
        &mut self.base
    }

    fn local_intersect(&self, _object_ray: Ray) -> Vec<Intersection> {
        vec![]
    }

    fn local_norm_at(&self, _object_point: Tuple, _hit: &Intersection) -> Tuple {
        vector!(0, 0, 0)
    }

    fn includes(&self, other: &dyn Shape) -> bool {
        if self.get_unique_id() == other.get_unique_id() {
            true
        } else {
            self.s1.includes(other) || self.s2.includes(other)
        }
    }
}

impl CSG {
    fn filter_intersections<'a>(
        &self,
        intersections: &Vec<Intersection<'a>>,
    ) -> Vec<Intersection<'a>> {
        // begin outside of both children
        let mut inside_s1 = false;
        let mut inside_s2 = false;
        let mut filtered: Vec<Intersection> = vec![];

        for i in intersections {
            let hit_s1 = self.s1.includes(i.object);
            if CSG::intersection_allowed(self.op, hit_s1, inside_s1, inside_s2) {
                filtered.push(*i);
            }
            if hit_s1 {
                inside_s1 = !inside_s1;
            } else {
                inside_s2 = !inside_s2;
            }
        }
        filtered
    }
    // hit_s1: true if intersection is with a CSG's s1, false if with the s2
    // inside_s1: true if intersection is inside CSG's s1, false otherwise
    // inside_s2: true if intersection is inside CSG's s2, false otherwise
    fn intersection_allowed(
        op: CSGOperator,
        hit_s1: bool,
        inside_s1: bool,
        inside_s2: bool,
    ) -> bool {
        match op {
            CSGOperator::Union() => (hit_s1 && !inside_s2) || (!hit_s1 && !inside_s1),
            CSGOperator::Intersection() => (hit_s1 && inside_s2) || (!hit_s1 && inside_s1),
            CSGOperator::Difference() => (hit_s1 && !inside_s2) || (!hit_s1 && inside_s1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shape::csg::CSGOperator::Difference;
    use crate::shape::csg::CSGOperator::Union;
    use crate::shape::cube::Cube;
    use crate::shape::sphere::Sphere;
    use std::ptr;

    #[test]
    fn csg_construction() {
        // TODO: possibly fragile test
        let s1 = Box::new(Sphere::new());
        let s1_address = s1.as_ref() as *const dyn Shape;
        let s2 = Box::new(Cube::new());
        let s2_address = s2.as_ref() as *const dyn Shape;

        let c = CSG::new(CSGOperator::Union(), s1, s2);
        assert_eq!(c.op, CSGOperator::Union());

        assert_eq!(c.s1.as_ref() as *const _, s1_address);
        assert_eq!(c.s2.as_ref() as *const _, s2_address);
    }

    #[test]
    fn csg_operation_rule_evaluation() {
        let test_data = vec![
            ("union1", Union(), true, true, true, false),
            ("union2", Union(), true, true, false, true),
            ("union3", Union(), true, false, true, false),
            ("union4", Union(), true, false, false, true),
            ("union5", Union(), false, true, true, false),
            ("union6", Union(), false, true, false, false),
            ("union7", Union(), false, false, true, true),
            ("union8", Union(), false, false, false, true),
            (
                "union9",
                CSGOperator::Intersection(),
                true,
                true,
                true,
                true,
            ),
            (
                "intersection1",
                CSGOperator::Intersection(),
                true,
                true,
                false,
                false,
            ),
            (
                "intersection2",
                CSGOperator::Intersection(),
                true,
                false,
                true,
                true,
            ),
            (
                "intersection3",
                CSGOperator::Intersection(),
                true,
                false,
                false,
                false,
            ),
            (
                "intersection4",
                CSGOperator::Intersection(),
                false,
                true,
                true,
                true,
            ),
            (
                "intersection5",
                CSGOperator::Intersection(),
                false,
                true,
                false,
                true,
            ),
            (
                "intersection6",
                CSGOperator::Intersection(),
                false,
                false,
                true,
                false,
            ),
            (
                "intersection7",
                CSGOperator::Intersection(),
                false,
                false,
                false,
                false,
            ),
            ("", Difference(), true, true, true, false),
            ("", Difference(), true, true, false, true),
            ("", Difference(), true, false, true, false),
            ("", Difference(), true, false, false, true),
            ("", Difference(), false, true, true, true),
            ("", Difference(), false, true, false, true),
            ("", Difference(), false, false, true, false),
            ("", Difference(), false, false, false, false),
        ];
        for (name, op, hit_s1, inside_s1, inside_s2, expected) in test_data {
            assert_eq!(
                expected,
                CSG::intersection_allowed(op, hit_s1, inside_s1, inside_s2),
                "Case {}",
                name
            );
        }
    }

    #[test]
    fn filter_intersections() {
        let test_data = vec![
            ("union", Union(), 0, 3),
            ("intersection", CSGOperator::Intersection(), 1, 2),
            ("difference", Difference(), 0, 1),
        ];
        for (name, op, x0, x1) in test_data {
            let s1 = Box::new(Sphere::new());
            let s2 = Box::new(Cube::new());
            let c = CSG::new(op, s1, s2);
            let xs = vec![
                Intersection::new(1., c.s1.as_ref()),
                Intersection::new(2., c.s2.as_ref()),
                Intersection::new(3., c.s1.as_ref()),
                Intersection::new(4., c.s2.as_ref()),
            ];
            let filtered = c.filter_intersections(&xs);
            println!("{:?}", filtered);
            assert_eq!(2, filtered.len(), "case: {}", name);
            assert_eq!(&filtered[0], &xs[x0], "case: {}", name);
            assert_eq!(&filtered[1], &xs[x1], "case: {}", name);
        }
    }
}
