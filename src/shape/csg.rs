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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shape::cube::Cube;
    use crate::shape::sphere::Sphere;
    use crate::test::utils::dummy_intersection;

    #[test]
    fn CSG_construction() {
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
}
