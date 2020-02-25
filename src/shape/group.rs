use crate::intersection::Intersection;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::shape::base_shape::BaseShape;
use crate::shape::shape::Shape;
use crate::tuple::Tuple;

#[derive(Debug)]
pub struct GroupShape {
    base: BaseShape,
    children: Vec<Box<dyn Shape>>,
}

impl GroupShape {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add_child(&mut self, mut child: Box<dyn Shape>) {
        child.as_mut().set_parent(self);
        self.children.push(child);
    }
}

impl Default for GroupShape {
    fn default() -> GroupShape {
        GroupShape {
            base: BaseShape::new(),
            children: vec![],
        }
    }
}

impl Shape for GroupShape {
    fn get_base(&self) -> &BaseShape {
        &self.base
    }
    fn get_base_mut(&mut self) -> &mut BaseShape {
        &mut self.base
    }
    fn local_intersect(&self, _object_ray: Ray) -> Vec<Intersection> {
        vec![]
    }
    fn local_norm_at(&self, _object_point: Tuple) -> Tuple {
        vector!(0, 0, 0)
    }
    fn transformation_inverse(&self) -> &Matrix {
        self.base.transformation_inverse()
    }
    fn transformation_inverse_transpose(&self) -> &Matrix {
        self.base.transformation_inverse_transpose()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shape::base_shape::BaseShape;
    use std::ptr;

    #[test]
    fn add_child_to_group() {
        let s = Box::new(BaseShape::new());
        let s_address = s.as_ref() as *const dyn Shape;
        let mut g = GroupShape::new();
        g.add_child(s);
        assert_eq!(g.children.len(), 1, "g should have 1 child,");
        assert_eq!(
            g.children[0].as_ref() as *const _,
            s_address,
            "the one child should be s,"
        );
        assert!(
            ptr::eq(g.children[0].get_parent().unwrap(), &g),
            "and s's parent should be g"
        );
    }
}
