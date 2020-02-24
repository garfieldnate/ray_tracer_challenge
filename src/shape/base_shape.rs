use crate::intersection::Intersection;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::shape::shape::Shape;
use crate::tuple::Tuple;
use std::fmt::Debug;

// Other shape implementations are meant to delegate to this one where these defaults are acceptable.
// TODO: Maybe someday Rust will support delegation: https://github.com/rust-lang/rfcs/pull/2393
// like Kotlin does. Could also use ambassador crate, if it adds partial delegation support.
#[derive(Default, Clone, Debug, PartialEq)]
pub struct BaseShape {
    t: Matrix,
    t_inverse: Matrix,
    t_inverse_transpose: Matrix,
    m: Material,
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
