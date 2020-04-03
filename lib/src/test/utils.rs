use crate::intersection::Intersection;
use crate::shape::shape::Shape;
use crate::shape::sphere::Sphere;
use std::cell::RefCell;

pub fn dummy_intersection(s: &dyn Shape) -> Intersection {
    Intersection::new(1., s)
}

pub fn any_shape() -> Box<dyn Shape> {
    Box::new(Sphere::new())
}

// "Jitter" referring to point sampling for area lights
pub fn constant_jitter() -> Option<Box<dyn Fn() -> f32>> {
    Some(Box::new(|| 0.5))
}

pub fn hardcoded_jitter(sequence: Vec<f32>) -> Option<Box<dyn Fn() -> f32>> {
    let hardcoded_sequence = RefCell::new(sequence.into_iter().cycle());
    Some(Box::new(move || {
        hardcoded_sequence.borrow_mut().next().unwrap()
    }))
}
