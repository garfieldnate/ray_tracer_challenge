use crate::intersection::Intersection;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::shape::base_shape::BaseShape;
use crate::shape::shape::Shape;
use crate::tuple::Tuple;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct GroupShape {
    base: BaseShape,
    pub children: Vec<Box<dyn Shape>>,
    self_rc: Option<Weak<GroupShape>>,
}

impl GroupShape {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn is_empty(&self) -> bool {
        self.children.is_empty()
    }
    pub fn add_child(&mut self, mut child: Box<dyn Shape>) {
        child.as_mut().set_parent(Some(Rc::new(*self)));
        if self.self_rc.is_none() {
            self.self_rc = Some(Rc::downgrade(&Rc::new(*self)));
        }
        self.children.push(child);
    }
}

impl Default for GroupShape {
    fn default() -> GroupShape {
        GroupShape {
            base: BaseShape::new(),
            children: vec![],
            self_rc: None,
        }
    }
}

impl Shape for GroupShape {
    fn local_intersect(&self, _object_ray: Ray) -> Vec<Intersection> {
        vec![]
    }
    fn local_norm_at(&self, _object_point: Tuple) -> Tuple {
        vector!(0, 0, 0)
    }

    // Forward these to the wrapped BaseShape instance
    fn transformation(&self) -> &Matrix {
        &self.base.transformation()
    }
    fn set_transformation(&mut self, t: Matrix) {
        self.base.set_transformation(t);
    }
    fn transformation_inverse(&self) -> &Matrix {
        self.base.transformation_inverse()
    }
    fn transformation_inverse_transpose(&self) -> &Matrix {
        self.base.transformation_inverse_transpose()
    }
    fn material(&self) -> &Material {
        self.base.material()
    }
    fn set_material(&mut self, m: Material) {
        self.base.set_material(m)
    }
    fn get_parent(&self) -> &Option<Rc<GroupShape>> {
        self.base.get_parent()
    }
    fn set_parent(&mut self, shape: Option<Rc<GroupShape>>) {
        self.base.set_parent(shape)
    }
    fn casts_shadow(&self) -> bool {
        self.base.casts_shadow()
    }
    fn set_casts_shadow(&mut self, casts_shadow: bool) {
        self.base.set_casts_shadow(casts_shadow)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shape::test_shape::TestShape;
    #[test]
    fn add_child_to_group() {
        let mut g = GroupShape::new();
        let s = TestShape::new();
        g.add_child(Box::new(s));
        assert!(!g.is_empty());
        // assert!(ptr::eq(g.children[0].as_ref(), &s));
        // assert_eq!(g.children[0].get_parent().as_ref().unwrap().as_ref(), &g);
    }
    // ​ 	    ​And​ g includes s
    // ​ 	    ​And​ s.parent = g
}
