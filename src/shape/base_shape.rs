use crate::intersection::Intersection;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::shape::group::GroupShape;
use crate::shape::shape::Shape;
use crate::tuple::Tuple;
use std::fmt::Debug;
use std::sync::atomic::{AtomicPtr, Ordering};

// Other shape implementations should delegate to this one where these defaults are acceptable.
#[derive(Default, Debug)]
pub struct BaseShape {
    t: Matrix,
    t_inverse: Matrix,
    t_inverse_transpose: Matrix,
    m: Material,
    casts_shadow: bool,
    parent: AtomicPtr<GroupShape>,
}

impl BaseShape {
    pub fn new() -> Self {
        Self {
            casts_shadow: true,
            ..Default::default()
        }
    }
}

impl Shape for BaseShape {
    // these two are unimplemented because BaseShape is not meant to be instantiated by itself
    fn get_base(&self) -> &BaseShape {
        unimplemented!()
    }
    fn get_base_mut(&mut self) -> &mut BaseShape {
        unimplemented!()
    }
    fn transformation(&self) -> &Matrix {
        &self.t
    }
    fn set_transformation(&mut self, t: Matrix) {
        self.t = t;
        self.t_inverse = self.t.inverse();
        self.t_inverse_transpose = self.t.inverse().transpose();
    }
    fn material(&self) -> &Material {
        &self.m
    }
    fn set_material(&mut self, m: Material) {
        self.m = m;
    }
    fn casts_shadow(&self) -> bool {
        self.casts_shadow
    }
    fn set_casts_shadow(&mut self, casts_shadow: bool) {
        self.casts_shadow = casts_shadow;
    }

    fn transformation_inverse(&self) -> &Matrix {
        &self.t_inverse
    }
    fn transformation_inverse_transpose(&self) -> &Matrix {
        &self.t_inverse_transpose
    }

    fn set_parent(&mut self, group: &mut GroupShape) {
        // TODO: add programmatic check that parent can only be set once?
        self.parent = AtomicPtr::new(group);
    }
    fn get_parent(&self) -> Option<&GroupShape> {
        unsafe { self.parent.load(Ordering::Relaxed).as_ref() }
    }

    // These two methods *must* be implemented by wrapping implementations
    fn local_intersect(&self, _object_ray: Ray) -> Vec<Intersection> {
        unimplemented!()
    }
    fn local_norm_at(&self, _object_point: Tuple) -> Tuple {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matrix::identity_4x4;
    use crate::shape::base_shape::BaseShape;
    use crate::transformations::translation;
    use std::ptr;

    #[test]
    fn shape_transformation() {
        let mut shape = BaseShape::new();
        assert_eq!(
            shape.transformation(),
            &identity_4x4(),
            "Default transform should be identity"
        );

        shape.set_transformation(translation(2.0, 3.0, 4.0));
        assert_eq!(
            shape.transformation(),
            &translation(2.0, 3.0, 4.0),
            "transformation should be settable"
        );
    }

    #[test]
    fn shape_material() {
        let mut shape = BaseShape::new();
        assert_eq!(shape.material(), &Material::default(), "Default material");

        let mut override_material = Material::default();
        override_material.ambient = 1.0;
        shape.set_material(override_material.clone());
        assert_eq!(
            shape.material(),
            &override_material,
            "material should be settable"
        );
    }

    #[test]
    fn shape_parent() {
        let mut shape = BaseShape::new();
        assert!(shape.get_parent().is_none(), "No parent group by default");

        let mut parent = GroupShape::new();
        shape.set_parent(&mut parent);
        assert!(
            ptr::eq(shape.get_parent().unwrap(), &parent),
            "Parent group should be settable"
        );
    }

    #[test]
    fn shape_casts_shadow() {
        let mut shape = BaseShape::new();
        assert_eq!(shape.casts_shadow(), true, "casts shadow by default");

        shape.set_casts_shadow(false);
        assert!(!shape.casts_shadow(), "casts_shadow should be settable");
    }
}
