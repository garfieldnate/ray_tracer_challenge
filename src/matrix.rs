use crate::tuple::*;
use approx::AbsDiffEq;
use std::ops::Mul;

// Only supports square matrices
#[derive(Clone, Debug, PartialEq)]
pub struct Matrix {
    size: usize,
    // TODO: maybe this should be private with accessor
    pub data: Vec<Vec<f32>>,
}

// TODO: derive size from data dyanmically instead of storing it
pub fn build_matrix(size: usize) -> Matrix {
    Matrix {
        size,
        data: vec![vec![0.0; size]; size],
    }
}

// Use like this: matrix!([0,1],[1.5,2])
macro_rules! matrix {
    ($([$($x:expr),* $(,)*]),+ $(,)*) => {{
        let data = vec![$(vec![$($x as f32,)*],)*];
        Matrix {
            size: data.len(),
            data
        }
    }};
}

// TODO: if done as a macro, could handle different sizes
pub fn identity_4x4() -> Matrix {
    matrix!([1, 0, 0, 0], [0, 1, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1])
}

// TODO: matrix declaration is suuuuper verbose
// TODO: the self args should be &self to prevent copying; not sure how to do that
impl Mul for &Matrix {
    type Output = Matrix;
    // multiply two 4x4 matrices; the book says that's the only dimension that we'll have to deal with
    fn mul(self, other: &Matrix) -> Matrix {
        debug_assert_eq!(
            self.data.len(),
            4,
            "Only 4x4 matrices can be multiplied by tuples!"
        );
        let size = self.size;
        let mut new_matrix = build_matrix(size);
        for r in 0..size {
            for c in 0..size {
                new_matrix.data[r][c] = self.data[r][0] * other.data[0][c]
                    + self.data[r][1] * other.data[1][c]
                    + self.data[r][2] * other.data[2][c]
                    + self.data[r][3] * other.data[3][c]
            }
        }
        new_matrix
    }
}

impl Mul<&Tuple> for &Matrix {
    type Output = Tuple;
    fn mul(self, other: &Tuple) -> Tuple {
        debug_assert_eq!(
            self.size, 4,
            "Only 4x4 matrices can be multiplied by tuples!"
        );
        let x = self.data[0][0] * other.x
            + self.data[0][1] * other.y
            + self.data[0][2] * other.z
            + self.data[0][3] * other.w;
        let y = self.data[1][0] * other.x
            + self.data[1][1] * other.y
            + self.data[1][2] * other.z
            + self.data[1][3] * other.w;
        let z = self.data[2][0] * other.x
            + self.data[2][1] * other.y
            + self.data[2][2] * other.z
            + self.data[2][3] * other.w;
        let w = self.data[3][0] * other.x
            + self.data[3][1] * other.y
            + self.data[3][2] * other.z
            + self.data[3][3] * other.w;
        Tuple { x, y, z, w }
    }
}

// required for approximate comparisons due to use of floating point numbers
impl AbsDiffEq for Matrix {
    type Epsilon = f32;

    fn default_epsilon() -> Self::Epsilon {
        f32::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        for row in 0..self.size {
            for col in 0..self.size {
                if !f32::abs_diff_eq(&self.data[row][col], &other.data[row][col], epsilon) {
                    println!(
                        "{} not close enough to {}",
                        self.data[row][col], other.data[row][col]
                    );
                    return false;
                }
            }
        }
        return true;
    }
}

impl Matrix {
    // TODO: would it be better to mutate instead of copying?
    pub fn transpose(&self) -> Matrix {
        // debug_assert!(self.rows == 4 && self.columns == 4, "Only 4x4 matrices can be tr");
        let mut m = build_matrix(self.size);
        for row in 0..self.size {
            for col in 0..self.size {
                m.data[col][row] = self.data[row][col];
            }
        }
        m
    }

    pub fn determinant(&self) -> f32 {
        // base case: 2x2 matrix
        if self.size == 2 {
            self.data[0][0] * self.data[1][1] - self.data[0][1] * self.data[1][0]
        } else {
            // recurse: combine determinants of submatrices
            let mut det = 0.0;
            // pivot on row 0 because it's simple
            // a human would probably choose the row with the most 0's
            for col in 0..self.size {
                let cofactor = self.cofactor(0, col);
                det += cofactor * self.data[0][col];
            }
            det
        }
    }

    // for an nxn matrix, return an n-1 x n-1 matrix with remove_row row and remove_col col removed
    pub fn submatrix(&self, remove_row: usize, remove_col: usize) -> Matrix {
        let mut m = build_matrix(self.size - 1);
        let mut new_row = 0;
        for old_row in 0..self.size {
            if old_row == remove_row {
                continue;
            }
            let mut new_col = 0;
            for old_col in 0..self.size {
                if old_col == remove_col {
                    continue;
                }
                m.data[new_row][new_col] = self.data[old_row][old_col];
                new_col += 1;
            }
            new_row += 1;
        }
        m
    }

    pub fn cofactor(&self, row: usize, column: usize) -> f32 {
        let minor = self.minor(row, column);

        if (row + column) % 2 == 0 {
            minor
        } else {
            -minor
        }
    }

    pub fn minor(&self, row: usize, column: usize) -> f32 {
        self.submatrix(row, column).determinant()
    }

    pub fn invertible(&self) -> bool {
        self.determinant() != 0.0
    }

    pub fn inverse(&self) -> Matrix {
        debug_assert!(self.invertible());
        let determinant = self.determinant();
        let mut matrix_inverse = build_matrix(self.size);
        for row in 0..self.size {
            for column in 0..self.size {
                let c = self.cofactor(row, column);
                matrix_inverse.data[column][row] = c / determinant;
                // println!(
                //     "{},{} is {}/{}={}",
                //     column,
                //     row,
                //     c,
                //     determinant,
                //     c / determinant
                // );
            }
        }
        matrix_inverse
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_matrix_multiplied_by_tuple() {
        let matrix_a = matrix!([1, 2, 3, 4], [2, 4, 4, 2], [8, 6, 4, 1], [0, 0, 0, 1]);
        let b = build_tuple(1.0, 2.0, 3.0, 1.0);

        assert_eq!(&matrix_a * &b, build_tuple(18.0, 24.0, 33.0, 1.0));
    }

    #[test]
    fn test_multiplying_two_matrices() {
        let matrix_a = matrix!([1, 2, 3, 4], [5, 6, 7, 8], [9, 8, 7, 6], [5, 4, 3, 2]);
        let matrix_b = matrix!([-2, 1, 2, 3], [3, 2, 1, -1], [4, 3, 6, 5], [1, 2, 7, 8]);

        let expected = matrix!(
            [20, 22, 50, 48],
            [44, 54, 114, 108],
            [40, 58, 110, 102],
            [16, 26, 46, 42]
        );
        assert_eq!(&matrix_a * &matrix_b, expected);
    }

    #[test]
    fn test_multiplying_by_identity_matrix() {
        let matrix_a = matrix!([0, 1, 2, 4], [1, 2, 4, 8], [2, 4, 8, 16], [4, 8, 16, 32]);
        let matrix_i = identity_4x4();
        assert_eq!(&matrix_a * &matrix_i, matrix_a);
    }

    #[test]
    fn test_matrix_transpose() {
        let m = matrix!([0, 9, 3, 0], [9, 8, 0, 0], [1, 8, 5, 0], [0, 0, 5, 0]);

        let expected_m_transpose = matrix!([0, 9, 1, 0], [9, 8, 8, 0], [3, 0, 5, 5], [0, 0, 0, 0]);
        assert_eq!(m.transpose(), expected_m_transpose);
    }

    #[test]
    fn test_transposing_identity_is_identity() {
        let matrix_i = identity_4x4();
        let transposed = matrix_i.transpose();
        assert_eq!(transposed, matrix_i);
    }

    #[test]
    fn test_determinant_2x2() {
        assert_eq!(matrix!([1, 5], [-3, 2]).determinant(), 17.0);
    }

    #[test]
    fn test_submatrix_of_3x3() {
        let matrix_a = matrix!([1, 5, 0], [-3, 2, 7], [0, 6, -3]);

        let expected_submatrix = matrix!([-3, 2], [0, 6]);
        assert_eq!(matrix_a.submatrix(0, 2), expected_submatrix);
    }

    #[test]
    fn test_submatrix_of_4x4() {
        let matrix_a = matrix!([-6, 1, 1, 6], [-8, 5, 8, 6], [-1, 0, 8, 2], [-7, 1, -1, 1]);

        let expected_submatrix = matrix!([-6, 1, 6], [-8, 8, 6], [-7, -1, 1]);
        assert_eq!(matrix_a.submatrix(2, 1), expected_submatrix);
    }

    #[test]
    fn test_minor_of_3x3_matrix() {
        let matrix_a = matrix!([3, 5, 0], [2, -1, -7], [6, -1, 5]);
        assert_eq!(matrix_a.minor(1, 0), 25.0);
    }

    #[test]
    fn test_cofactor_of_3x3_matrix() {
        let matrix_a = matrix!([3, 5, 0], [2, -1, -7], [6, -1, 5]);

        assert_eq!(matrix_a.cofactor(0, 0), -12.0);
        assert_eq!(matrix_a.cofactor(1, 0), -25.0);
    }

    #[test]
    fn test_determinant_of_3x3_matrix() {
        let matrix_a = matrix!([1, 2, 6], [-5, 8, -4], [2, 6, 4]);

        assert_eq!(matrix_a.cofactor(0, 0), 56.0);
        assert_eq!(matrix_a.cofactor(0, 1), 12.0);
        assert_eq!(matrix_a.cofactor(0, 2), -46.0);
        assert_eq!(matrix_a.determinant(), -196.0);
    }

    #[test]
    fn test_determinant_of_4x4_matrix() {
        let matrix_a = matrix!([-2, -8, 3, 5], [-3, 1, 7, 3], [1, 2, -9, 6], [-6, 7, 7, -9]);

        assert_eq!(matrix_a.cofactor(0, 0), 690.0);
        assert_eq!(matrix_a.cofactor(0, 1), 447.0);
        assert_eq!(matrix_a.cofactor(0, 2), 210.0);
        assert_eq!(matrix_a.cofactor(0, 3), 51.0);
        assert_eq!(matrix_a.determinant(), -4071.0);
    }

    #[test]
    fn test_non_0_determinant_matrix_is_invertible() {
        let matrix_a = matrix!([6, 4, 4, 4], [5, 5, 7, 6], [4, -9, 3, -7], [9, 1, 7, -6]);

        assert_eq!(matrix_a.determinant(), -2120.0);
        assert!(matrix_a.invertible());
    }

    #[test]
    fn test_0_determinant_matrix_is_invertible() {
        let matrix_a = matrix!([-4, 2, -2, -3], [9, 6, 2, 6], [0, -5, 1, -5], [0, 0, 0, 0]);

        assert_eq!(matrix_a.determinant(), 0.0);
        assert!(!matrix_a.invertible());
    }

    #[test]
    fn test_matrix_inversion_1() {
        let matrix_a = matrix!([-5, 2, 6, -8], [1, -5, 1, 8], [7, 7, -6, -7], [1, -3, 7, 4]);

        //TODO: implement matrix scalar multiplication to simplify this
        let mut expected_inverse = build_matrix(4);
        let expected_determinant = 532.0;
        expected_inverse.data[0][0] = 116.0 / expected_determinant;
        expected_inverse.data[1][0] = -430.0 / expected_determinant;
        expected_inverse.data[2][0] = -42.0 / expected_determinant;
        expected_inverse.data[3][0] = -278.0 / expected_determinant;
        expected_inverse.data[0][1] = 240.0 / expected_determinant;
        expected_inverse.data[1][1] = -775.0 / expected_determinant;
        expected_inverse.data[2][1] = -119.0 / expected_determinant;
        expected_inverse.data[3][1] = -433.0 / expected_determinant;
        expected_inverse.data[0][2] = 128.0 / expected_determinant;
        expected_inverse.data[1][2] = -236.0 / expected_determinant;
        expected_inverse.data[2][2] = -28.0 / expected_determinant;
        expected_inverse.data[3][2] = -160.0 / expected_determinant;
        expected_inverse.data[0][3] = -24.0 / expected_determinant;
        expected_inverse.data[1][3] = 277.0 / expected_determinant;
        expected_inverse.data[2][3] = 105.0 / expected_determinant;
        expected_inverse.data[3][3] = 163.0 / expected_determinant;

        assert_eq!(matrix_a.inverse(), expected_inverse);
    }

    #[test]
    fn test_matrix_inversion_2() {
        let matrix_a = matrix!([8, -5, 9, 2], [7, 5, 6, 1], [-6, 0, 9, 6], [-3, 0, -9, -4]);

        let mut expected_inverse = build_matrix(4);
        let expected_determinant = -585.0;
        expected_inverse.data[0][0] = 90.0 / expected_determinant;
        expected_inverse.data[1][0] = 45.0 / expected_determinant;
        expected_inverse.data[2][0] = -210.0 / expected_determinant;
        expected_inverse.data[3][0] = 405.0 / expected_determinant;
        expected_inverse.data[0][1] = 90.0 / expected_determinant;
        expected_inverse.data[1][1] = -72.0 / expected_determinant;
        expected_inverse.data[2][1] = -210.0 / expected_determinant;
        expected_inverse.data[3][1] = 405.0 / expected_determinant;
        expected_inverse.data[0][2] = 165.0 / expected_determinant;
        expected_inverse.data[1][2] = -15.0 / expected_determinant;
        expected_inverse.data[2][2] = -255.0 / expected_determinant;
        expected_inverse.data[3][2] = 450.0 / expected_determinant;
        expected_inverse.data[0][3] = 315.0 / expected_determinant;
        expected_inverse.data[1][3] = -18.0 / expected_determinant;
        expected_inverse.data[2][3] = -540.0 / expected_determinant;
        expected_inverse.data[3][3] = 1125.0 / expected_determinant;

        assert_eq!(matrix_a.inverse(), expected_inverse);
    }

    #[test]
    fn test_matrix_inversion_3() {
        let matrix_a = matrix!([9, 3, 0, 9], [-5, -2, -6, -3], [-4, 9, 6, 4], [-7, 6, 6, 2]);

        let mut expected_inverse = build_matrix(4);
        let expected_determinant = 1620.0;
        expected_inverse.data[0][0] = -66.0 / expected_determinant;
        expected_inverse.data[1][0] = -126.0 / expected_determinant;
        expected_inverse.data[2][0] = -47.0 / expected_determinant;
        expected_inverse.data[3][0] = 288.0 / expected_determinant;
        expected_inverse.data[0][1] = -126.0 / expected_determinant;
        expected_inverse.data[1][1] = 54.0 / expected_determinant;
        expected_inverse.data[2][1] = -237.0 / expected_determinant;
        expected_inverse.data[3][1] = 108.0 / expected_determinant;
        expected_inverse.data[0][2] = 234.0 / expected_determinant;
        expected_inverse.data[1][2] = 594.0 / expected_determinant;
        expected_inverse.data[2][2] = -177.0 / expected_determinant;
        expected_inverse.data[3][2] = -432.0 / expected_determinant;
        expected_inverse.data[0][3] = -360.0 / expected_determinant;
        expected_inverse.data[1][3] = -540.0 / expected_determinant;
        expected_inverse.data[2][3] = 210.0 / expected_determinant;
        expected_inverse.data[3][3] = 540.0 / expected_determinant;

        assert_eq!(matrix_a.inverse(), expected_inverse);
    }

    #[test]
    fn test_invert_inverts_multiplication() {
        let matrix_a = matrix!([3, -9, 7, 3], [3, -8, 2, -9], [-4, 4, 4, 1], [-6, 5, -1, 1]);
        let matrix_b = matrix!([8, 2, 2, 2], [3, -1, 7, 0], [7, 0, 5, 4], [6, -2, 0, 5]);

        let matrix_c = &matrix_a * &matrix_b;
        let matrix_c_times_b_inverse = &matrix_c * &matrix_b.inverse();

        // higher epsilon because of multiplications
        assert!(matrix_c_times_b_inverse.abs_diff_eq(&matrix_a, 10.0 * f32::default_epsilon()));
    }
}
