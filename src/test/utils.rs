use crate::intersection::Intersection;
use crate::shape::shape::Shape;

pub fn dummy_intersection(s: &dyn Shape) -> Intersection {
    Intersection::new(1., s)
}
