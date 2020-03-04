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
    left: Box<dyn Shape>,
    right: Box<dyn Shape>,
}

impl CSG {
    pub fn new(op: CSGOperator, left: Box<dyn Shape>, right: Box<dyn Shape>) -> Self {
        CSG {
            base: BaseShape::new(),
            op,
            left,
            right,
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

fn intersection_allowed(
    op: CSGOperator,
    hit_left: bool,
    inside_left: bool,
    inside_right: bool,
) -> bool {
    match op {
        CSGOperator::Union() => (hit_left && !inside_right) || (!hit_left && !inside_left),
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
        let left = Box::new(Sphere::new());
        let left_address = left.as_ref() as *const dyn Shape;
        let right = Box::new(Cube::new());
        let right_address = right.as_ref() as *const dyn Shape;

        let c = CSG::new(CSGOperator::Union(), left, right);
        assert_eq!(c.op, CSGOperator::Union());

        assert_eq!(c.left.as_ref() as *const _, left_address);
        assert_eq!(c.right.as_ref() as *const _, right_address);
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
        for (name, op, hit_left, inside_left, inside_right, expected) in test_data {
            assert_eq!(
                expected,
                intersection_allowed(op, hit_left, inside_left, inside_right),
                "Case {}",
                name
            );
        }
    }
}
