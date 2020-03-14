use crate::tuple::Tuple;
use std::f32;

#[derive(Copy, Clone, PartialEq, Debug)]
struct BoundingBox {
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
}
