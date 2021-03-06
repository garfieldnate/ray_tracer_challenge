use approx::AbsDiffEq;
use std::fmt::Display;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Tuple {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

// TODO: implement approximate comparison via approx crate
// TODO: allow changing datatypes to f64?
impl Tuple {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        debug_assert!(w == 1.0 || w == 0.0, "w must be 0 or 1; was {}", w);
        debug_assert!(!x.is_nan(), "x cannot be NaN");
        debug_assert!(!y.is_nan(), "x cannot be NaN");
        debug_assert!(!z.is_nan(), "z cannot be NaN");
        Tuple { x, y, z, w }
    }
    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }
    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }
    pub fn magnitude(&self) -> f32 {
        debug_assert!(self.is_vector());
        // TODO: book says w is included.
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + (self.w as f32).powi(2)).sqrt()
    }
    pub fn norm(&self) -> Tuple {
        //TODO: should only take vectors, not tuples
        let magnitude = self.magnitude();
        Tuple {
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude,
            w: self.w,
        }
    }
    pub fn dot(&self, other: Tuple) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z + (self.w * other.w) as f32
    }
    pub fn cross(&self, other: Tuple) -> Tuple {
        Tuple {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
            // result is also a vector
            w: 0.0,
        }
    }
}

impl Display for Tuple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        if self.is_point() {
            write!(f, "point!(")?;
        } else {
            write!(f, "vector!(")?;
        }
        write!(f, "{}, {}, {})", self.x, self.y, self.z)?;

        Ok(())
    }
}

// Use like this: point!(1,2,3)
#[macro_export]
macro_rules! point {
    ($x:expr, $y:expr, $z:expr) => {{
        Tuple::new($x as f32, $y as f32, $z as f32, 1.0)
    }};
}

// Use like this: vector!(1,2,3)
#[macro_export]
macro_rules! vector {
    ($x:expr, $y:expr, $z:expr) => {{
        Tuple::new($x as f32, $y as f32, $z as f32, 0.0)
    }};
}

impl Add for Tuple {
    type Output = Tuple;
    fn add(self, other: Tuple) -> Tuple {
        Tuple::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
            self.w + other.w,
        )
    }
}

impl Sub for Tuple {
    type Output = Tuple;
    fn sub(self, other: Tuple) -> Tuple {
        Tuple::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
            self.w - other.w,
        )
    }
}

impl Mul<f32> for Tuple {
    type Output = Tuple;
    fn mul(self, scalar: f32) -> Tuple {
        Tuple {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
            // TODO: book says w should only be 0 or 1, but scalar multiplication applies
            w: self.w * scalar,
        }
    }
}

impl Mul<Tuple> for f32 {
    type Output = Tuple;
    fn mul(self, tuple: Tuple) -> Tuple {
        tuple * self
    }
}

impl Div<f32> for Tuple {
    type Output = Tuple;
    fn div(self, scalar: f32) -> Tuple {
        Tuple {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
            // TODO: book says w should only be 0 or 1, but scalar division applies
            w: self.w / scalar,
        }
    }
}

impl Neg for Tuple {
    type Output = Tuple;
    fn neg(self) -> Tuple {
        Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            // TODO: w should only be 1 or 0, but book says w should also be negated
            w: -self.w,
        }
    }
}

// required for approximate comparisons due to use of floating point numbers
impl AbsDiffEq for Tuple {
    type Epsilon = f32;

    fn default_epsilon() -> Self::Epsilon {
        f32::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        f32::abs_diff_eq(&self.x, &other.x, epsilon)
            && f32::abs_diff_eq(&self.y, &other.y, epsilon)
            && f32::abs_diff_eq(&self.z, &other.z, epsilon)
            && f32::abs_diff_eq(&self.w, &other.w, epsilon)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tuple_constructor() {
        let tuple = Tuple::new(1.1, 2.2, 3.3, 0.0);
        assert_eq!(
            tuple,
            Tuple {
                x: 1.1,
                y: 2.2,
                z: 3.3,
                w: 0.0
            }
        );
    }

    #[test]
    fn test_tuple_with_w_equal_1_is_point() {
        let tuple = Tuple {
            x: 1.1,
            y: 2.2,
            z: 3.3,
            w: 1.0,
        };
        assert!(tuple.is_point());
    }

    #[test]
    fn test_tuple_with_w_equal_0_is_vector() {
        let tuple = Tuple {
            x: 1.1,
            y: 2.2,
            z: 3.3,
            w: 0.0,
        };
        assert!(tuple.is_vector());
    }

    #[test]
    fn test_point_creates_tuple_with_w_equal_1() {
        let p = point!(1.1, 2.2, 3.3);
        assert_eq!(
            p,
            Tuple {
                x: 1.1,
                y: 2.2,
                z: 3.3,
                w: 1.0
            }
        );
    }

    #[test]
    fn test_vector_creates_tuple_with_w_equal_0() {
        let v = vector!(1.1, 2.2, 3.3);
        assert_eq!(
            v,
            Tuple {
                x: 1.1,
                y: 2.2,
                z: 3.3,
                w: 0.0
            }
        );
    }

    #[test]
    fn test_add_tuples() {
        let tuple_1 = Tuple {
            x: 1.1,
            y: 2.2,
            z: 3.3,
            w: 0.0,
        };
        let tuple_2 = Tuple {
            x: 1.1,
            y: -7.4,
            z: 3.3,
            w: 1.0,
        };
        let sum = tuple_1 + tuple_2;
        assert_abs_diff_eq!(
            sum,
            Tuple {
                x: 2.2,
                y: -5.2,
                z: 6.6,
                w: 1.0
            }
        );
    }

    #[test]
    fn test_subtract_points() {
        let p1 = point!(1, 2, 3);
        let p2 = point!(4, 5, 6);

        let subtrahend = p1 - p2;
        assert_abs_diff_eq!(subtrahend, vector!(-3, -3, -3));
    }

    #[test]
    fn test_subtract_vector_from_point() {
        let p = point!(3, 2, 1);
        let v = vector!(5, 6, 7);

        let subtrahend = p - v;
        assert_abs_diff_eq!(subtrahend, point!(-2, -4, -6));
    }

    #[test]
    fn test_subtract_vectors() {
        let v1 = vector!(3, 2, 1);
        let v2 = vector!(5, 6, 7);

        let subtrahend = v1 - v2;
        assert_abs_diff_eq!(subtrahend, vector!(-2, -4, -6));
    }

    #[test]
    fn test_tuple_negation() {
        let tuple = Tuple::new(1.0, -2.0, 3.0, 1.0);
        let negated = -tuple;

        assert_abs_diff_eq!(
            negated,
            Tuple {
                x: -1.0,
                y: 2.0,
                z: -3.0,
                w: -1.0
            }
        );
    }

    #[test]
    fn test_tuple_scalar_multiplication() {
        let tuple = Tuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: 4.0,
        };
        let scaled_1 = tuple * 3.5;
        let scaled_2 = 3.5 * tuple;

        // check commutivity
        assert_abs_diff_eq!(scaled_2, scaled_1);

        assert_abs_diff_eq!(
            scaled_1,
            Tuple {
                x: 3.5,
                y: -7.0,
                z: 10.5,
                w: 14.0
            }
        );
    }

    #[test]
    fn test_tuple_scalar_division() {
        let tuple = Tuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: 4.0,
        };
        let scaled = tuple / 2.0;

        assert_abs_diff_eq!(
            scaled,
            Tuple {
                x: 0.5,
                y: -1.0,
                z: 1.5,
                w: 2.0
            }
        );
    }

    #[test]
    fn test_vector_magnitude() {
        let x = vector!(1, 0, 0);
        assert_eq!(x.magnitude(), 1.0);

        let y = vector!(0, 1, 0);
        assert_eq!(y.magnitude(), 1.0);

        let z = vector!(0, 0, 1);
        assert_eq!(z.magnitude(), 1.0);

        // Note: should technically use some kind of epsilon comparison
        let v1 = vector!(1, 2, 3);
        assert_eq!(v1.magnitude(), (14.0 as f32).sqrt());

        let v2 = vector!(-1, -2, -3);
        assert_eq!(v2.magnitude(), (14.0 as f32).sqrt());
    }

    #[test]
    fn test_vector_norm() {
        let x = vector!(4, 0, 0);
        assert_abs_diff_eq!(x.norm(), vector!(1, 0, 0));

        let y = vector!(1, 2, 3);
        let mag = (14.0 as f32).sqrt();
        assert_abs_diff_eq!(y.norm(), vector!(1.0 / mag, 2.0 / mag, 3.0 / mag));

        let normed = vector!(1, 2, 3).norm();
        assert_abs_diff_eq!(normed.magnitude(), 1.0);
    }

    #[test]
    fn test_vector_dot_product() {
        let x = vector!(1.1, 2.2, 3.3);
        let y = vector!(2.2, 3.3, 4.4);
        assert_abs_diff_eq!(x.dot(y), 24.2);
    }

    #[test]
    fn test_vector_cross_product() {
        let x = vector!(1, 2, 3);
        let y = vector!(2, 3, 4);
        assert_eq!(x.cross(y), vector!(-1, 2, -1));
        assert_eq!(y.cross(x), vector!(1, -2, 1));
    }
}
