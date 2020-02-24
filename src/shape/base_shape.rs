use crate::intersection::Intersection;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::shape::group::GroupShape;
use crate::shape::shape::Shape;
use crate::tuple::Tuple;
use std::fmt::Debug;
use std::rc::Rc;

// Other shape implementations are meant to delegate to this one where these defaults are acceptable.
// TODO: Maybe someday Rust will support delegation: https://github.com/rust-lang/rfcs/pull/2393
// like Kotlin does. Could also use ambassador crate, if it adds partial delegation support.
#[derive(Default, Clone, Debug, PartialEq)]
pub struct BaseShape {
    t: Matrix,
    t_inverse: Matrix,
    t_inverse_transpose: Matrix,
    m: Material,
    parent: Option<Rc<GroupShape>>,
    casts_shadow: bool,
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
    fn set_parent(&mut self, shape: Option<Rc<GroupShape>>) {
        self.parent = shape;
    }
    fn get_parent(&self) -> &Option<Rc<GroupShape>> {
        &self.parent
    }

    fn transformation_inverse(&self) -> &Matrix {
        &self.t_inverse
    }
    fn transformation_inverse_transpose(&self) -> &Matrix {
        &self.t_inverse_transpose
    }

    // These two methods cannot be delegated to
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
    use crate::transformations::translation;

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
    fn shape_parent() {
        let mut shape = BaseShape::new();
        assert!(
            shape.get_parent().is_none(),
            "Shape is not assigned to group by default"
        );

        shape.set_parent(Some(Rc::new(GroupShape::new())));
        assert!(shape.get_parent().is_some(), "parent should be settable");
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
    fn shape_casts_shadow() {
        let mut shape = BaseShape::new();
        assert_eq!(shape.casts_shadow(), true, "casts shadow by default");

        shape.set_casts_shadow(false);
        assert!(!shape.casts_shadow(), "casts_shadow should be settable");
    }
}
