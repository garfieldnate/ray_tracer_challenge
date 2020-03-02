use crate::shape::shape::Shape;
use std::cmp::Ordering::Equal;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Intersection<'a> {
    pub distance: f32,
    pub object: &'a dyn Shape,
    pub u: f32,
    pub v: f32,
}

impl Intersection<'_> {
    pub fn new(distance: f32, object: &dyn Shape) -> Intersection {
        Intersection {
            distance,
            object,
            u: 0.,
            v: 0.,
        }
    }
    pub fn new_with_uv(distance: f32, object: &dyn Shape, u: f32, v: f32) -> Intersection {
        Intersection {
            distance,
            object,
            u,
            v,
        }
    }
    // returns the a reference to the intersection with the lowest non-negative distance (or None if all are negative)
    pub fn hit<'a>(intersections: &'a [Intersection<'a>]) -> Option<&'a Intersection<'a>> {
        intersections
            .iter()
            .filter(|i| i.distance >= 0.0)
            .min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap_or(Equal))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shape::shape::Shape;
    use crate::shape::sphere::Sphere;
    use std::ptr;

    #[test]
    fn basic_intersection_creation() {
        let s = Sphere::new();
        let i = Intersection::new(1.0, &s);
        assert_eq!(i.distance, 1.0);
        assert!(ptr::eq(&s as &dyn Shape, i.object as &dyn Shape));
    }

    #[test]
    fn hit_all_intersections_have_positive_distance() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);

        let intersections = vec![i1, i2];
        let i = Intersection::hit(&intersections).unwrap();
        assert_eq!(i, &i1);
    }

    #[test]
    fn hit_some_interactions_have_negative_distance() {
        let s = Sphere::new();
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(1.0, &s);
        let i3 = Intersection::new(-0.5, &s);
        let interactions = vec![i1, i2, i3];
        let i = Intersection::hit(&interactions).unwrap();
        assert_eq!(&i2, i);
    }

    #[test]
    fn no_hit_when_all_interactions_negative() {
        let s = Sphere::new();
        let i1 = Intersection::new(-2.0, &s);
        let i2 = Intersection::new(-1.0, &s);
        let i3 = Intersection::new(-0.5, &s);
        let interactions = vec![i1, i2, i3];
        let i = Intersection::hit(&interactions);
        assert!(i.is_none());
    }

    #[test]
    fn hit_is_lowest_nonnegative_intersection() {
        let s = Sphere::new();
        let i1 = Intersection::new(5.0, &s);
        let i2 = Intersection::new(7.0, &s);
        let i3 = Intersection::new(-3.0, &s);
        let i4 = Intersection::new(2.0, &s);
        let interactions = vec![i1, i2, i3, i4];
        let i = Intersection::hit(&interactions).unwrap();
        assert_eq!(&i4, i);
    }

    #[test]
    fn create_intersection_with_uv() {
        let s = Sphere::new();
        let i = Intersection::new_with_uv(1.0, &s, 0.2, 0.4);
        assert_eq!(i.distance, 1.0);
        assert!(ptr::eq(&s as &dyn Shape, i.object as &dyn Shape));
        assert_eq!(i.u, 0.2);
        assert_eq!(i.v, 0.4);
    }
}
