use crate::bounding_box::BoundingBox;
use crate::intersection::Intersection;
use crate::ray::Ray;
use crate::shape::base_shape::BaseShape;
use crate::shape::shape::Shape;
use crate::tuple::Tuple;
use std::cell::RefCell;
use std::fmt::Debug;

#[derive(Debug)]
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
    fn get_base(&self) -> &BaseShape {
        &self.base
    }
    fn get_base_mut(&mut self) -> &mut BaseShape {
        &mut self.base
    }
    fn local_intersect(&self, _object_ray: Ray) -> Vec<Intersection> {
        // save the incoming ray for a comparison test
        self.saved_ray.borrow_mut().replace(_object_ray);
        vec![]
    }
    fn local_norm_at(&self, _object_point: Tuple, _hit: &Intersection) -> Tuple {
        // return something that will let us test both the input and output calculations
        vector!(
            2.0 * _object_point.x,
            3.0 * _object_point.y,
            4.0 * _object_point.z
        )
    }

    fn bounding_box(&self) -> BoundingBox {
        // arbitrary but easy to work with
        BoundingBox {
            min: point!(-1, -1, -1),
            max: point!(1, 1, 1),
        }
    }
}
