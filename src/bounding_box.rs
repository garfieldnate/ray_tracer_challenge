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
}
