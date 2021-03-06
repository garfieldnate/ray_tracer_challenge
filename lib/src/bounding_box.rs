use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::shape::cube::aabb_intersection;
use crate::tuple::Tuple;
use std::f32;

// TODO: wouldn't it be better to have a tighter, non-axis-aligned bounding box?
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

    pub fn transform(&self, m: &Matrix) -> BoundingBox {
        let mut new_box = BoundingBox::empty();
        // transform all 8 corners of self and add them to the new bounding box
        let p1 = self.min;
        let p2 = point!(self.min.x, self.min.y, self.max.z);
        let p3 = point!(self.min.x, self.max.y, self.min.z);
        let p4 = point!(self.min.x, self.max.y, self.max.z);
        let p5 = point!(self.max.x, self.min.y, self.min.z);
        let p6 = point!(self.max.x, self.min.y, self.max.z);
        let p7 = point!(self.max.x, self.max.y, self.min.z);
        let p8 = self.max;

        for p in vec![p1, p2, p3, p4, p5, p6, p7, p8] {
            new_box.add_point(m * &p);
        }

        new_box
    }

    pub fn intersects(&self, r: Ray) -> bool {
        aabb_intersection(r, self.min, self.max).is_some()
    }

    pub fn split(&self) -> (BoundingBox, BoundingBox) {
        // figure out the box's largest dimension
        let dx = self.max.x - self.min.x;
        let dy = self.max.y - self.min.y;
        let dz = self.max.z - self.min.z;

        let greatest = dx.max(dy).max(dz);

        // variables to help construct the points on the dividing plane
        let (mut x0, mut y0, mut z0) = (self.min.x, self.min.y, self.min.z);
        let (mut x1, mut y1, mut z1) = (self.max.x, self.max.y, self.max.z);

        // adjust the points so that they lie on the dividing plane
        if greatest == dx {
            // eprintln!("Splitting box on x");
            x0 = x0 + dx / 2.;
            x1 = x0;
        } else if greatest == dy {
            // eprintln!("Splitting box on y");
            y0 = y0 + dy / 2.;
            y1 = y0;
        } else {
            // eprintln!("Splitting box on z");
            z0 = z0 + dz / 2.;
            z1 = z0;
        }

        let mid_min = point!(x0, y0, z0);
        let mid_max = point!(x1, y1, z1);

        // construct and return the two halves of the bounding box
        let left = BoundingBox::with_bounds(self.min, mid_max);
        let right = BoundingBox::with_bounds(mid_min, self.max);

        (left, right)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transformations::rotation_x;
    use crate::transformations::rotation_y;
    use std::f32::consts::PI;

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

    #[test]
    fn transform_bounding_box() {
        let box1 = BoundingBox::with_bounds(point!(-1, -1, -1), point!(1, 1, 1));
        let matrix = rotation_x(PI / 4.) * rotation_y(PI / 4.);
        let box2 = box1.transform(&matrix);
        assert_abs_diff_eq!(box2.min, point!(-1.4142135, -1.7071067, -1.7071067));
        assert_abs_diff_eq!(box2.max, point!(1.4142135, 1.7071067, 1.7071067));
    }

    #[test]
    fn intersecting_ray_with_bounding_box_at_origin() {
        let b = BoundingBox::with_bounds(point!(-1, -1, -1), point!(1, 1, 1));
        let test_data = vec![
            ("1", point!(5, 0.5, 0), vector!(-1, 0, 0), true),
            ("2", point!(-5, 0.5, 0), vector!(1, 0, 0), true),
            ("3", point!(0.5, 5, 0), vector!(0, -1, 0), true),
            ("4", point!(0.5, -5, 0), vector!(0, 1, 0), true),
            ("5", point!(0.5, 0, 5), vector!(0, 0, -1), true),
            ("6", point!(0.5, 0, -5), vector!(0, 0, 1), true),
            ("7", point!(0, 0.5, 0), vector!(0, 0, 1), true),
            ("8", point!(-2, 0, 0), vector!(2, 4, 6), false),
            ("9", point!(0, -2, 0), vector!(6, 2, 4), false),
            ("10", point!(0, 0, -2), vector!(4, 6, 2), false),
            ("11", point!(2, 0, 2), vector!(0, 0, -1), false),
            ("12", point!(0, 2, 2), vector!(0, -1, 0), false),
            ("13", point!(2, 2, 0), vector!(-1, 0, 0), false),
        ];
        for (name, origin, direction, expected) in test_data {
            // let direction = direciton.normalize();
            let r = Ray::new(origin, direction.norm());
            assert_eq!(expected, b.intersects(r), "Case {}", name);
        }
    }

    #[test]
    fn intersecting_ray_with_bounding_box_not_at_origin() {
        let b = BoundingBox::with_bounds(point!(5, -2, 0), point!(11, 4, 7));
        let test_data = vec![
            ("1", point!(15, 1, 2), vector!(-1, 0, 0), true),
            ("2", point!(-5, -1, 4), vector!(1, 0, 0), true),
            ("3", point!(7, 6, 5), vector!(0, -1, 0), true),
            ("4", point!(9, -5, 6), vector!(0, 1, 0), true),
            ("5", point!(8, 2, 12), vector!(0, 0, -1), true),
            ("6", point!(6, 0, -5), vector!(0, 0, 1), true),
            ("7", point!(8, 1, 3.5), vector!(0, 0, 1), true),
            ("8", point!(9, -1, -8), vector!(2, 4, 6), false),
            ("9", point!(8, 3, -4), vector!(6, 2, 4), false),
            ("10", point!(9, -1, -2), vector!(4, 6, 2), false),
            ("11", point!(4, 0, 9), vector!(0, 0, -1), false),
            ("12", point!(8, 6, -1), vector!(0, -1, 0), false),
            ("13", point!(12, 5, 4), vector!(-1, 0, 0), false),
        ];

        for (name, origin, direction, expected) in test_data {
            // let direction = direciton.normalize();
            let r = Ray::new(origin, direction.norm());
            assert_eq!(expected, b.intersects(r), "Case {}", name);
        }
    }

    #[test]
    fn splitting_perfect_cube() {
        let b = BoundingBox::with_bounds(point!(-1, -4, -5), point!(9, 6, 5));
        let (left, right) = b.split();
        assert_eq!(left.min, point!(-1, -4, -5));
        assert_eq!(left.max, point!(4, 6, 5));
        assert_eq!(right.min, point!(4, -4, -5));
        assert_eq!(right.max, point!(9, 6, 5));
    }

    #[test]
    fn splitting_x_wide_box() {
        let b = BoundingBox::with_bounds(point!(-1, -2, -3), point!(9, 5.5, 3));
        let (left, right) = b.split();
        assert_eq!(left.min, point!(-1, -2, -3));
        assert_eq!(left.max, point!(4, 5.5, 3));
        assert_eq!(right.min, point!(4, -2, -3));
        assert_eq!(right.max, point!(9, 5.5, 3));
    }

    #[test]
    fn splitting_y_wide_box() {
        let b = BoundingBox::with_bounds(point!(-1, -2, -3), point!(5, 8, 3));
        let (left, right) = b.split();
        assert_eq!(left.min, point!(-1, -2, -3));
        assert_eq!(left.max, point!(5, 3, 3));
        assert_eq!(right.min, point!(-1, 3, -3));
        assert_eq!(right.max, point!(5, 8, 3));
    }

    #[test]
    fn splitting_z_wide_box() {
        let b = BoundingBox::with_bounds(point!(-1, -2, -3), point!(5, 3, 7));
        let (left, right) = b.split();
        assert_eq!(left.min, point!(-1, -2, -3));
        assert_eq!(left.max, point!(5, 3, 2));
        assert_eq!(right.min, point!(-1, -2, 2));
        assert_eq!(right.max, point!(5, 3, 7));
    }
}
