use crate::shape::shape::Shape;
use std::cmp::Ordering::Equal;
use std::ptr;
#[derive(Copy, Clone, Debug)]
pub struct Intersection<'a> {
    pub distance: f32,
    pub object: &'a dyn Shape,
}

impl PartialEq for Intersection<'_> {
    fn eq(&self, other: &Intersection) -> bool {
        // TODO: why doesn't Rust allow self.object == other.object, and also auto deriving of this trait impl?
        self.distance.eq(&other.distance) && ptr::eq(self.object, other.object)
    }
}

impl Intersection<'_> {
    pub fn new<'a>(distance: f32, object: &'a dyn Shape) -> Intersection<'a> {
        Intersection { distance, object }
    }
    // returns the a reference to the intersection with the lowest non-negative distance (or None if all are negative)
    pub fn hit<'a>(intersections: &'a Vec<Intersection<'a>>) -> Option<&'a Intersection<'a>> {
        intersections
            .iter()
            .filter(|i| i.distance >= 0.0)
            .min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap_or(Equal))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::material::Material;
    use crate::matrix::identity_4x4;
    use crate::ray::Ray;
    use crate::shape::shape::Shape;
    use crate::shape::sphere::Sphere;
    use crate::transformations::scaling;
    use crate::transformations::translation;
    use crate::tuple::Tuple;
    use crate::world::precompute_values;

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
        let i1 = Intersection {
            distance: 1.0,
            object: &s,
        };
        let i2 = Intersection {
            distance: 2.0,
            object: &s,
        };

        let intersections = vec![i1, i2];
        let i = Intersection::hit(&intersections).unwrap();
        assert_eq!(i, &i1);
    }

    #[test]
    fn hit_some_interactions_have_negative_distance() {
        let s = Sphere::new();
        let i1 = Intersection {
            distance: -1.0,
            object: &s,
        };
        let i2 = Intersection {
            distance: 1.0,
            object: &s,
        };
        let i3 = Intersection {
            distance: -0.5,
            object: &s,
        };
        let interactions = vec![i1, i2, i3];
        let i = Intersection::hit(&interactions).unwrap();
        assert_eq!(&i2, i);
    }

    #[test]
    fn no_hit_when_all_interactions_negative() {
        let s = Sphere::new();
        let i1 = Intersection {
            distance: -2.0,
            object: &s,
        };
        let i2 = Intersection {
            distance: -1.0,
            object: &s,
        };
        let i3 = Intersection {
            distance: -0.5,
            object: &s,
        };
        let interactions = vec![i1, i2, i3];
        let i = Intersection::hit(&interactions);
        assert!(i.is_none());
    }

    #[test]
    fn hit_is_lowest_nonnegative_intersection() {
        let s = Sphere::new();
        let i1 = Intersection {
            distance: 5.0,
            object: &s,
        };
        let i2 = Intersection {
            distance: 7.0,
            object: &s,
        };
        let i3 = Intersection {
            distance: -3.0,
            object: &s,
        };
        let i4 = Intersection {
            distance: 2.0,
            object: &s,
        };
        let interactions = vec![i1, i2, i3, i4];
        let i = Intersection::hit(&interactions).unwrap();
        assert_eq!(&i4, i);
    }
}
