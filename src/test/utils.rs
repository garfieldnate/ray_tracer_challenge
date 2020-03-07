use crate::intersection::Intersection;
use crate::shape::shape::Shape;
use crate::shape::sphere::Sphere;

pub fn dummy_intersection(s: &dyn Shape) -> Intersection {
    Intersection::new(1., s)
}

pub fn any_shape() -> Box<dyn Shape> {
    Box::new(Sphere::new())
}
