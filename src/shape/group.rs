use crate::intersection::Intersection;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::shape::base_shape::BaseShape;
use crate::shape::shape::Shape;
use crate::tuple::Tuple;

#[derive(Debug, PartialEq)]
struct GroupShape {
    base: BaseShape,
    pub children: Vec<Box<dyn Shape>>,
}

impl GroupShape {
    fn new() -> Self {
        Self::default()
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

    // Forward these to the wrapped BaseShape instance
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
}

#[cfg(test)]
mod tests {
    #[test]
    fn default_group_parent() {}
}
