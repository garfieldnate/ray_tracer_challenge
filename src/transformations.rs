use crate::matrix::*;

pub fn translation(x: f32, y: f32, z: f32) -> Matrix {
    matrix!([1, 0, 0, x], [0, 1, 0, y], [0, 0, 1, z], [0, 0, 0, 1])
}

pub fn scaling(x: f32, y: f32, z: f32) -> Matrix {
    matrix!([x, 0, 0, 0], [0, y, 0, 0], [0, 0, z, 0], [0, 0, 0, 1])
}

pub fn rotation_x(radians: f32) -> Matrix {
    let cosine = radians.cos();
    let sine = radians.sin();
    matrix!(
        [1, 0, 0, 0],
        [0, cosine, -sine, 0],
        [0, sine, cosine, 0],
        [0, 0, 0, 1]
    )
}

pub fn rotation_y(radians: f32) -> Matrix {
    let cosine = radians.cos();
    let sine = radians.sin();
    matrix!(
        [cosine, 0, sine, 0],
        [0, 1, 0, 0],
        [-sine, 0, cosine, 0],
        [0, 0, 0, 1]
    )
}

pub fn rotation_z(radians: f32) -> Matrix {
    let cosine = radians.cos();
    let sine = radians.sin();
    matrix!(
        [cosine, -sine, 0, 0],
        [sine, cosine, 0, 0],
        [0, 0, 1, 0],
        [0, 0, 0, 1]
    )
}

// `x_y` meaning it shears x in proportion to y, etc.
pub fn shearing(x_y: f32, x_z: f32, y_x: f32, y_z: f32, z_x: f32, z_y: f32) -> Matrix {
    matrix!(
        [1, x_y, x_z, 0],
        [y_x, 1, y_z, 0],
        [z_x, z_y, 1, 0],
        [0, 0, 0, 1]
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tuple::*;
    use std::f32::consts::{FRAC_1_SQRT_2, FRAC_PI_2, FRAC_PI_4};

    #[test]
    fn multiply_by_translation_matrix() {
        let transform = translation(5.0, -3.0, 2.0);
        let p = point!(-3, 4, 5);
        assert_eq!(&transform * &p, point!(2, 1, 7));
    }

    #[test]
    fn multiply_by_inverse_of_translation_matrix() {
        let transform = translation(5.0, -3.0, 2.0);
        let inverse_transform = transform.inverse();
        let p = point!(-3, 4, 5);
        assert_eq!(&inverse_transform * &p, point!(-8, 7, 3));
    }

    #[test]
    fn translation_does_not_affect_vector() {
        let transform = translation(5.0, -3.0, 2.0);
        let v = vector!(-3, 4, 5);
        assert_eq!(&transform * &v, v);
    }

    #[test]
    fn scaling_matrix_applied_to_point() {
        let transform = scaling(2.0, 3.0, 4.0);
        let p = point!(-4, 6, 8);
        assert_eq!(&transform * &p, point!(-8, 18, 32));
    }

    #[test]
    fn scaling_matrix_applied_to_vector() {
        let transform = scaling(2.0, 3.0, 4.0);
        let p = vector!(-4, 6, 8);
        assert_eq!(&transform * &p, vector!(-8, 18, 32));
    }

    #[test]
    fn multiply_by_inverse_of_scaling_matrix() {
        let transform = scaling(2.0, 3.0, 4.0);
        let inv = transform.inverse();
        let v = vector!(-4, 6, 8);
        assert_eq!(&inv * &v, vector!(-2, 2, 2));
    }

    #[test]
    fn reflection_is_scaling_by_negative_value() {
        let transform = scaling(-1.0, 1.0, 1.0);
        let p = point!(2, 3, 4);
        assert_eq!(&transform * &p, point!(-2, 3, 4));
    }

    #[test]
    fn rotating_point_around_x_axis() {
        let p = point!(0, 1, 0);
        let half_quarter = rotation_x(FRAC_PI_4);
        let full_quarter = rotation_x(FRAC_PI_2);
        assert_abs_diff_eq!(&half_quarter * &p, point!(0, FRAC_1_SQRT_2, FRAC_1_SQRT_2));
        assert_abs_diff_eq!(&full_quarter * &p, &point!(0, 0, 1));
    }

    #[test]
    fn inverse_x_rototation_rotates_in_opposite_direction() {
        let p = point!(0, 1, 0);
        let half_quarter = rotation_x(FRAC_PI_4);
        let inv = half_quarter.inverse();
        assert_abs_diff_eq!(&inv * &p, point!(0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2));
    }

    #[test]
    fn rotating_point_around_y_axis() {
        let p = point!(0, 0, 1);
        let half_quarter = rotation_y(FRAC_PI_4);
        let full_quarter = rotation_y(FRAC_PI_2);
        assert_abs_diff_eq!(&half_quarter * &p, point!(FRAC_1_SQRT_2, 0, FRAC_1_SQRT_2));
        assert_abs_diff_eq!(&full_quarter * &p, &point!(1, 0, 0));
    }

    #[test]
    fn rotating_point_around_z_axis() {
        let p = point!(0, 1, 0);
        let half_quarter = rotation_z(FRAC_PI_4);
        let full_quarter = rotation_z(FRAC_PI_2);
        assert_abs_diff_eq!(&half_quarter * &p, point!(-FRAC_1_SQRT_2, FRAC_1_SQRT_2, 0));
        assert_abs_diff_eq!(&full_quarter * &p, &point!(-1, 0, 0));
    }

    #[test]
    fn shearing_moves_x_in_proportion_to_y() {
        let transform = shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = point!(2, 3, 4);
        assert_eq!(&transform * &p, point!(5, 3, 4));
    }

    #[test]
    fn shearing_moves_x_in_proportion_to_z() {
        let transform = shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = point!(2, 3, 4);
        assert_eq!(&transform * &p, point!(6, 3, 4));
    }

    #[test]
    fn shearing_moves_y_in_proportion_to_x() {
        let transform = shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = point!(2, 3, 4);
        assert_eq!(&transform * &p, point!(2, 5, 4));
    }

    #[test]
    fn shearing_moves_y_in_proportion_to_z() {
        let transform = shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = point!(2, 3, 4);
        assert_eq!(&transform * &p, point!(2, 7, 4));
    }

    #[test]
    fn shearing_moves_z_in_proportion_to_x() {
        let transform = shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = point!(2, 3, 4);
        assert_eq!(&transform * &p, point!(2, 3, 6));
    }

    #[test]
    fn shearing_moves_z_in_proportion_to_y() {
        let transform = shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = point!(2, 3, 4);
        assert_eq!(&transform * &p, point!(2, 3, 7));
    }

    #[test]
    fn transforms_applied_in_sequence() {
        let p = point!(1, 0, 1);
        let rotate = rotation_x(FRAC_PI_2);
        let scale = scaling(5.0, 5.0, 5.0);
        let translate = translation(10.0, 5.0, 7.0);

        assert_eq!(&(&(&translate * &scale) * &rotate) * &p, point!(15, 0, 7));
    }
}
