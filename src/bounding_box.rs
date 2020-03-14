use crate::tuple::Tuple;
use std::f32;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct BoundingBox {
    pub min: Tuple,
    pub max: Tuple,
}

impl Default for BoundingBox {
    fn default() -> Self {
        BoundingBox {
            min: point!(f32::INFINITY, f32::INFINITY, f32::INFINITY),
            max: point!(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY),
        }
    }
}

trait Between<T> {
    fn between_inclusive(self, min: T, max: T) -> bool;
}

impl<T: PartialOrd> Between<T> for T {
    fn between_inclusive(self, min: T, max: T) -> bool {
        self >= min && self <= max
    }
}

impl BoundingBox {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn with_bounds(min: Tuple, max: Tuple) -> Self {
        BoundingBox { min, max }
    }

    pub fn add_point(&mut self, p: Tuple) {
        self.min.x = self.min.x.min(p.x);
        self.min.y = self.min.y.min(p.y);
        self.min.z = self.min.z.min(p.z);

        self.max.x = self.max.x.max(p.x);
        self.max.y = self.max.y.max(p.y);
        self.max.z = self.max.z.max(p.z);
    }

    pub fn add_bounding_box(&mut self, other: BoundingBox) {
        self.add_point(other.min);
        self.add_point(other.max);
    }

    pub fn contains_point(&self, p: Tuple) -> bool {
        p.x.between_inclusive(self.min.x, self.max.x)
            && p.y.between_inclusive(self.min.y, self.max.y)
            && p.z.between_inclusive(self.min.z, self.max.z)
    }

    pub fn contains_bounding_box(&self, other: BoundingBox) -> bool {
        self.contains_point(other.min) && self.contains_point(other.max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adding_points_to_empty_bounding_box() {
        let mut bounding_box = BoundingBox::empty();
        let p1 = point!(-5, 2, 0);
        let p2 = point!(7, 0, -3);
        bounding_box.add_point(p1);
        bounding_box.add_point(p2);
        assert_eq!(bounding_box.min, point!(-5, 0, -3));
        assert_eq!(bounding_box.max, point!(7, 2, 0));
    }

    #[test]
    fn add_one_bounding_box_to_another() {
        let mut box1 = BoundingBox::with_bounds(point!(-5, -2, 0), point!(7, 4, 4));
        let box2 = BoundingBox::with_bounds(point!(8, -7, -2), point!(14, 2, 8));
        box1.add_bounding_box(box2);
        assert_eq!(box1.min, point!(-5, -7, -2));
        assert_eq!(box1.max, point!(14, 4, 8));
    }

    #[test]
    fn check_if_bounding_box_contains_given_point() {
        let b = BoundingBox::with_bounds(point!(5, -2, 0), point!(11, 4, 7));
        let test_data = vec![
            ("1", point!(5, -2, 0), true),
            ("2", point!(11, 4, 7), true),
            ("3", point!(8, 1, 3), true),
            ("4", point!(3, 0, 3), false),
            ("5", point!(8, -4, 3), false),
            ("6", point!(8, 1, -1), false),
            ("7", point!(13, 1, 3), false),
            ("8", point!(8, 5, 3), false),
            ("9", point!(8, 1, 8), false),
        ];
        for (name, p, expected) in test_data {
            assert_eq!(b.contains_point(p), expected, "Case {}", name);
        }
    }

    #[test]
    fn check_if_bounding_box_contains_other_box() {
        let box1 = BoundingBox::with_bounds(point!(5, -2, 0), point!(11, 4, 7));
        let test_data = vec![
            ("1", point!(5, -2, 0), point!(11, 4, 7), true),
            ("2", point!(6, -1, 1), point!(10, 3, 6), true),
            ("3", point!(4, -3, -1), point!(10, 3, 6), false),
            ("4", point!(6, -1, 1), point!(12, 5, 8), false),
        ];
        for (name, min, max, expected) in test_data {
            let box2 = BoundingBox::with_bounds(min, max);
            assert_eq!(box1.contains_bounding_box(box2), expected, "Case {}", name);
        }
    }
}
