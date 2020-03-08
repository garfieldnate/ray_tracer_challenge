use crate::intersection::Intersection;
use crate::shape::shape::Shape;
use crate::shape::sphere::Sphere;
use rand::distributions::Distribution;
use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;

pub fn dummy_intersection(s: &dyn Shape) -> Intersection {
    Intersection::new(1., s)
}

pub fn any_shape() -> Box<dyn Shape> {
    Box::new(Sphere::new())
}

// A random distribution that always returns 0.5
#[derive(Clone, Copy, Debug)]
pub struct ConstantDistribution;
impl Distribution<f32> for ConstantDistribution {
    fn sample<R: Rng + ?Sized>(&self, _rng: &mut R) -> f32 {
        0.5
    }
}

pub struct HardcodedDistribution {
    sequence: Rc<RefCell<dyn Iterator<Item = f32>>>,
}
impl HardcodedDistribution {
    pub fn new(sequence: Vec<f32>) -> Self {
        HardcodedDistribution {
            sequence: Rc::new(RefCell::new(sequence.into_iter())),
        }
    }
}
impl Distribution<f32> for HardcodedDistribution {
    fn sample<R: Rng + ?Sized>(&self, _rng: &mut R) -> f32 {
        self.sequence.borrow_mut().next().unwrap()
    }
}
