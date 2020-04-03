use crate::bounding_box::BoundingBox;
use crate::intersection::Intersection;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::object_id::ObjectId;
use crate::ray::Ray;
use crate::shape::shape::Shape;
use crate::tuple::Tuple;
use std::fmt::Debug;

// Other shape implementations should delegate to this one where these defaults are acceptable.
#[derive(Debug, Clone)]
pub struct BaseShape {
    casts_shadow: bool,
    id: ObjectId,
    t: Matrix,
    t_inverse: Matrix,
    t_inverse_transpose: Matrix,
    m: Material,
}

impl BaseShape {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Default for BaseShape {
    fn default() -> Self {
        Self {
            casts_shadow: true,
            // the rest are just defaults; TODO: can we automatically use defaults for remaining fields with a macro or something? Perhaps https://github.com/nrc/derive-new
            id: ObjectId::default(),
            t: Matrix::default(),
            t_inverse: Matrix::default(),
            t_inverse_transpose: Matrix::default(),
            m: Material::default(),
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
    fn get_unique_id(&self) -> usize {
        self.id.get_id()
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

    // These two methods *must* be implemented by wrapping implementations
    fn local_intersect(&self, _object_ray: Ray) -> Vec<Intersection> {
        unimplemented!()
    }
    fn local_norm_at(&self, _object_point: Tuple, _hit: &Intersection) -> Tuple {
        unimplemented!()
    }

    fn bounding_box(&self) -> BoundingBox {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matrix::identity_4x4;
    use crate::shape::base_shape::BaseShape;
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
    fn shape_material() {
        let mut shape = BaseShape::new();
        assert_eq!(shape.material(), &Material::default(), "Default material");

        let override_material = Material::builder().ambient(1.).build();
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

    #[test]
    fn cloned_baseshapes_have_different_ids() {
        let shape1 = BaseShape::new();
        let shape2 = shape1.clone();
        assert_ne!(shape1.id, shape2.id);
    }
}
