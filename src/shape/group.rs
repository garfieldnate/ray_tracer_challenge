use crate::intersection::Intersection;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::shape::base_shape::BaseShape;
use crate::shape::shape::Shape;
use crate::tuple::Tuple;

#[derive(Debug)]
pub struct GroupShape {
    base: BaseShape,
    pub children: Vec<Box<dyn Shape>>,
}

impl GroupShape {
    pub fn new() -> Self {
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
    #[test]
    fn default_group_parent() {}
}
