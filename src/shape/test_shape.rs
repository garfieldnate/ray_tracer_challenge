use crate::intersection::Intersection;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::shape::base_shape::BaseShape;
use crate::shape::shape::Shape;
use crate::tuple::Tuple;
use std::cell::RefCell;
use std::fmt::Debug;

#[derive(Clone, Debug, PartialEq)]
pub struct TestShape {
    pub base: BaseShape,
    pub saved_ray: RefCell<Option<Ray>>,
}

impl TestShape {
    pub fn new() -> Self {
        TestShape {
            base: BaseShape::new(),
            saved_ray: RefCell::new(None),
        }
    }
}

impl Shape for TestShape {
    fn transformation(&self) -> &Matrix {
        &self.base.transformation()
    }
    fn set_transformation(&mut self, t: Matrix) {
        self.base.set_transformation(t);
    }
    fn material(&self) -> &Material {
        self.base.material()
    }
    fn set_material(&mut self, m: Material) {
        self.base.set_material(m)
    }
    fn casts_shadow(&self) -> bool {
        self.base.casts_shadow()
    }
    fn set_casts_shadow(&mut self, casts_shadow: bool) {
        self.base.set_casts_shadow(casts_shadow)
    }
    fn local_intersect(&self, _object_ray: Ray) -> Vec<Intersection> {
        // save the incoming ray for a comparison test
        self.saved_ray.borrow_mut().replace(_object_ray);
        vec![]
    }
    fn local_norm_at(&self, _object_point: Tuple) -> Tuple {
        // return something that will let us test both the input and output calculations
        vector!(
            2.0 * _object_point.x,
            3.0 * _object_point.y,
            4.0 * _object_point.z
        )
    }
    fn transformation_inverse(&self) -> &Matrix {
        self.base.transformation_inverse()
    }
    fn transformation_inverse_transpose(&self) -> &Matrix {
        self.base.transformation_inverse_transpose()
    }
}
