use crate::approx::AbsDiffEq;
use crate::matrix::*;
use std::f32::consts::{FRAC_1_SQRT_2, FRAC_PI_2, FRAC_PI_4};

pub fn translation(x: f32, y: f32, z: f32) -> Matrix {
    let mut transform = identity_4x4();
    transform.data[0][3] = x;
    transform.data[1][3] = y;
    transform.data[2][3] = z;
    transform
}

pub fn scaling(x: f32, y: f32, z: f32) -> Matrix {
    let mut transform = build_matrix(4, 4);
    transform.data[0][0] = x;
    transform.data[1][1] = y;
    transform.data[2][2] = z;
    transform.data[3][3] = 1.0;
    transform
}

pub fn rotation_x(radians: f32) -> Matrix {
    let mut transform = build_matrix(4, 4);

    transform.data[0][0] = 1.0;
    transform.data[3][3] = 1.0;

    let cosine = radians.cos();
    transform.data[1][1] = cosine;
    transform.data[2][2] = cosine;

    let sine = radians.sin();
    transform.data[2][1] = sine;
    transform.data[1][2] = -sine;

    transform
}

pub fn rotation_y(radians: f32) -> Matrix {
    let mut transform = build_matrix(4, 4);

    transform.data[1][1] = 1.0;
    transform.data[3][3] = 1.0;

    let cosine = radians.cos();
    transform.data[0][0] = cosine;
    transform.data[2][2] = cosine;

    let sine = radians.sin();
    transform.data[0][2] = sine;
    transform.data[2][0] = -sine;

    transform
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tuple::*;

    #[test]
    fn multiply_by_translation_matrix() {
        let transform = translation(5.0, -3.0, 2.0);
        let p = point(-3.0, 4.0, 5.0);
        assert_eq!(&transform * &p, point(2.0, 1.0, 7.0));
    }

    #[test]
    fn multiply_by_inverse_of_translation_matrix() {
        let transform = translation(5.0, -3.0, 2.0);
        let inverse_transform = transform.inverse();
        let p = point(-3.0, 4.0, 5.0);
        assert_eq!(&inverse_transform * &p, point(-8.0, 7.0, 3.0));
    }

    #[test]
    fn translation_does_not_affect_vector() {
        let transform = translation(5.0, -3.0, 2.0);
        let v = vector(-3.0, 4.0, 5.0);
        assert_eq!(&transform * &v, v);
    }

    #[test]
    fn scaling_matrix_applied_to_point() {
        let transform = scaling(2.0, 3.0, 4.0);
        let p = point(-4.0, 6.0, 8.0);
        assert_eq!(&transform * &p, point(-8.0, 18.0, 32.0));
    }

    #[test]
    fn scaling_matrix_applied_to_vector() {
        let transform = scaling(2.0, 3.0, 4.0);
        let p = vector(-4.0, 6.0, 8.0);
        assert_eq!(&transform * &p, vector(-8.0, 18.0, 32.0));
    }

    #[test]
    fn multiply_by_inverse_of_scaling_matrix() {
        let transform = scaling(2.0, 3.0, 4.0);
        let inv = transform.inverse();
        let v = vector(-4.0, 6.0, 8.0);
        assert_eq!(&inv * &v, vector(-2.0, 2.0, 2.0));
    }

    #[test]
    fn reflection_is_scaling_by_negative_value() {
        let transform = scaling(-1.0, 1.0, 1.0);
        let p = point(2.0, 3.0, 4.0);
        assert_eq!(&transform * &p, point(-2.0, 3.0, 4.0));
    }

    #[test]
    fn rotating_point_around_x_axis() {
        let p = point(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(FRAC_PI_4);
        let full_quarter = rotation_x(FRAC_PI_2);
        assert_abs_diff_eq!(&half_quarter * &p, point(0.0, FRAC_1_SQRT_2, FRAC_1_SQRT_2));
        assert_abs_diff_eq!(&full_quarter * &p, &point(0.0, 0.0, 1.0));
    }

    #[test]
    fn inverse_x_rototation_rotates_in_opposite_direction() {
        let p = point(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(FRAC_PI_4);
        let inv = half_quarter.inverse();
        assert_abs_diff_eq!(&inv * &p, point(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2));
    }

    #[test]
    fn rotating_point_around_y_axis() {
        let p = point(0.0, 0.0, 1.0);
        let half_quarter = rotation_y(FRAC_PI_4);
        let full_quarter = rotation_y(FRAC_PI_2);
        assert_abs_diff_eq!(&half_quarter * &p, point(FRAC_1_SQRT_2, 0.0, FRAC_1_SQRT_2));
        assert_abs_diff_eq!(&full_quarter * &p, &point(1.0, 0.0, 0.0));
    }
    // ​ 	  ​Given​ p ← point(0, 0, 1)
    // ​ 	    ​And​ half_quarter ← rotation_y(π / 4)
    // ​ 	    ​And​ full_quarter ← rotation_y(π / 2)
    // ​ 	  ​Then​ half_quarter * p = point(√2/2, 0, √2/2)
    // ​ 	    ​And​ full_quarter * p = point(1, 0, 0)
}
