use crate::intersection::Intersection;
use crate::ray::Ray;
use crate::shape::base_shape::BaseShape;
use crate::shape::shape::Shape;
use crate::tuple::Tuple;

#[derive(Debug, PartialEq, Eq)]
pub enum CSGOperator {
    Union(),
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

    fn local_intersect(&self, object_ray: Ray) -> Vec<Intersection> {
        vec![]
    }

    fn local_norm_at(&self, _object_point: Tuple, _hit: &Intersection) -> Tuple {
        vector!(0, 0, 0)
    }
}

// hit_s1: true if intersection is with a CSG's s1, false if with the s2
// inside_s1: true if intersection is inside CSG's s1, false otherwise
// inside_s2: true if intersection is inside CSG's s2, false otherwise
fn intersection_allowed(op: CSGOperator, hit_s1: bool, inside_s1: bool, inside_s2: bool) -> bool {
    match op {
        CSGOperator::Union() => (hit_s1 && !inside_s2) || (!hit_s1 && !inside_s1),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shape::csg::CSGOperator::Union;
    use crate::shape::cube::Cube;
    use crate::shape::sphere::Sphere;

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
            ("1", Union(), true, true, true, false),
            ("2", Union(), true, true, false, true),
            ("3", Union(), true, false, true, false),
            ("4", Union(), true, false, false, true),
            ("5", Union(), false, true, true, false),
            ("6", Union(), false, true, false, false),
            ("7", Union(), false, false, true, true),
            ("8", Union(), false, false, false, true),
        ];
        for (name, op, hit_s1, inside_s1, inside_s2, expected) in test_data {
            assert_eq!(
                expected,
                intersection_allowed(op, hit_s1, inside_s1, inside_s2),
                "Case {}",
                name
            );
        }
    }
}
