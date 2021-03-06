use crate::tuple::*;
use approx::AbsDiffEq;
use std::fmt::Display;
use std::ops;
use std::ops::Mul;

// Only supports square matrices
#[derive(Clone, Debug, PartialEq)]
pub struct Matrix {
    // TODO: maybe this should be private with accessor
    pub data: Vec<Vec<f32>>,
}

impl Matrix {
    pub fn new(size: usize) -> Matrix {
        Matrix {
            data: vec![vec![0.0; size]; size],
        }
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "[")?;
        for row in 0..self.size() {
            write!(f, "\n    {:?}", self.data[row])?;
        }
        write!(f, "\n]")
    }
}

// Use like this: matrix!([0, 1], [1.5, 2])
#[macro_export]
macro_rules! matrix {
    ($([$($x:expr),* $(,)*]),+ $(,)*) => {{
        let data = vec![$(vec![$($x as f32,)*],)*];
        if cfg!(debug_assertions) {
            let expected_size = data.len();
            for row in &data {
                assert_eq!(expected_size, row.len(), "Wrong row length; expected {}, found {}", expected_size, row.len());
            }
        }
        Matrix {
            data
        }
    }};
}

impl Default for Matrix {
    fn default() -> Self {
        identity_4x4()
    }
}

// TODO: if done as a macro, could handle different sizes
pub fn identity_4x4() -> Matrix {
    matrix!([1, 0, 0, 0], [0, 1, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1])
}

impl Mul<f32> for &Matrix {
    type Output = Matrix;
    fn mul(self, other: f32) -> Matrix {
        let mut m = Matrix::new(self.size());
        for row in 0..self.size() {
            for col in 0..self.size() {
                m.data[row][col] = self.data[row][col] * other;
            }
        }
        m
    }
}

impl_op_ex!(*|a: &Matrix, b: &Tuple| -> Tuple {
    debug_assert_eq!(
        a.size(),
        4,
        "Only 4x4 matrices can be multiplied by tuples!"
    );
    let x = a.data[0][0] * b.x + a.data[0][1] * b.y + a.data[0][2] * b.z + a.data[0][3] * b.w;
    let y = a.data[1][0] * b.x + a.data[1][1] * b.y + a.data[1][2] * b.z + a.data[1][3] * b.w;
    let z = a.data[2][0] * b.x + a.data[2][1] * b.y + a.data[2][2] * b.z + a.data[2][3] * b.w;
    let w = a.data[3][0] * b.x + a.data[3][1] * b.y + a.data[3][2] * b.z + a.data[3][3] * b.w;
    Tuple { x, y, z, w }
});

impl_op_ex!(*|a: &Matrix, b: &Matrix| -> Matrix {
    debug_assert_eq!(
        a.data.len(),
        4,
        "Only 4x4 matrices can be multiplied by tuples!"
    );
    let size = a.size();
    let mut new_matrix = Matrix::new(size);
    for r in 0..size {
        for c in 0..size {
            new_matrix.data[r][c] = a.data[r][0] * b.data[0][c]
                + a.data[r][1] * b.data[1][c]
                + a.data[r][2] * b.data[2][c]
                + a.data[r][3] * b.data[3][c]
        }
    }
    new_matrix
});

// required for approximate comparisons due to use of floating point numbers
impl AbsDiffEq for Matrix {
    type Epsilon = f32;

    fn default_epsilon() -> Self::Epsilon {
        f32::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        for row in 0..self.size() {
            for col in 0..self.size() {
                if !f32::abs_diff_eq(&self.data[row][col], &other.data[row][col], epsilon) {
                    println!(
                        "{} not close enough to {}",
                        self.data[row][col], other.data[row][col]
                    );
                    return false;
                }
            }
        }
        true
    }
}

impl Matrix {
    pub fn size(&self) -> usize {
        self.data.len()
    }
    // TODO: would it be better to mutate instead of copying?
    pub fn transpose(&self) -> Matrix {
        // debug_assert!(self.rows == 4 && self.columns == 4, "Only 4x4 matrices can be tr");
        let mut m = Matrix::new(self.size());
        for row in 0..self.size() {
            for col in 0..self.size() {
                m.data[col][row] = self.data[row][col];
            }
        }
        m
    }

    pub fn determinant(&self) -> f32 {
        // base case: 2x2 matrix
        if self.size() == 2 {
            self.data[0][0] * self.data[1][1] - self.data[0][1] * self.data[1][0]
        } else {
            // recurse: combine determinants of submatrices
            let mut det = 0.0;
            // pivot on row 0 because it's simple
            // a human would probably choose the row with the most 0's
            for col in 0..self.size() {
                let cofactor = self.cofactor(0, col);
                det += cofactor * self.data[0][col];
            }
            det
        }
    }

    // for an nxn matrix, return an n-1 x n-1 matrix with remove_row row and remove_col col removed
    pub fn submatrix(&self, remove_row: usize, remove_col: usize) -> Matrix {
        let mut m = Matrix::new(self.size() - 1);
        let mut new_row = 0;
        for old_row in 0..self.size() {
            if old_row == remove_row {
                continue;
            }
            let mut new_col = 0;
            for old_col in 0..self.size() {
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
        let mut matrix_inverse = Matrix::new(self.size());
        for row in 0..self.size() {
            for column in 0..self.size() {
                let c = self.cofactor(row, column);
                matrix_inverse.data[column][row] = c / determinant;
            }
        }
        matrix_inverse
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_matrix_multiplied_by_scalar() {
        let matrix_a = matrix!([1, 2], [3, 4]);
        let expected = matrix!([10, 20], [30, 40]);

        assert_eq!(&matrix_a * 10.0, expected);
    }

    #[test]
    fn test_matrix_multiplied_by_tuple() {
        let matrix_a = matrix!([1, 2, 3, 4], [2, 4, 4, 2], [8, 6, 4, 1], [0, 0, 0, 1]);
        let b = Tuple::new(1.0, 2.0, 3.0, 1.0);

        assert_eq!(&matrix_a * &b, Tuple::new(18.0, 24.0, 33.0, 1.0));
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

        let expected_determinant = 532.0;
        let expected_inverse = &matrix!(
            [116, 240, 128, -24],
            [-430, -775, -236, 277],
            [-42, -119, -28, 105],
            [-278, -433, -160, 163]
        ) * (1.0 / expected_determinant);

        assert_abs_diff_eq!(matrix_a.inverse(), expected_inverse);
    }

    #[test]
    fn test_matrix_inversion_2() {
        let matrix_a = matrix!([8, -5, 9, 2], [7, 5, 6, 1], [-6, 0, 9, 6], [-3, 0, -9, -4]);

        let expected_determinant = -585.0;
        let expected_inverse = &matrix!(
            [90, 90, 165, 315],
            [45, -72, -15, -18],
            [-210, -210, -255, -540],
            [405, 405, 450, 1125]
        ) * (1.0 / expected_determinant);

        assert_abs_diff_eq!(matrix_a.inverse(), expected_inverse);
    }

    #[test]
    fn test_matrix_inversion_3() {
        let matrix_a = matrix!([9, 3, 0, 9], [-5, -2, -6, -3], [-4, 9, 6, 4], [-7, 6, 6, 2]);

        let expected_determinant = 1620.0;

        let expected_inverse = &matrix!(
            [-66, -126, 234, -360],
            [-126, 54, 594, -540],
            [-47, -237, -177, 210],
            [288, 108, -432, 540]
        ) * (1.0 / expected_determinant);

        assert_abs_diff_eq!(matrix_a.inverse(), expected_inverse);
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
