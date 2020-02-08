use crate::tuple::*;
use std::ops::Mul;

#[derive(Clone, Debug, PartialEq)]
pub struct Matrix {
    rows: usize,
    columns: usize,
    data: Vec<Vec<f32>>,
}

fn build_matrix(rows: usize, columns: usize) -> Matrix {
    Matrix {
        rows,
        columns,
        data: vec![vec![0.0; columns]; rows],
    }
}

pub fn identity_4x4() -> Matrix {
    let mut m = build_matrix(4, 4);
    m.data[0][0] = 1.0;
    m.data[1][1] = 1.0;
    m.data[2][2] = 1.0;
    m.data[3][3] = 1.0;
    m
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
        debug_assert_eq!(
            other.data.len(),
            4,
            "Only 4x4 matrices can be multiplied by tuples!"
        );
        let rows = self.rows;
        let columns = other.columns;
        let mut new_matrix = build_matrix(rows, columns);
        for r in 0..rows {
            for c in 0..columns {
                new_matrix.data[r][c] = self.data[r][0] * other.data[0][c]
                    + self.data[r][1] * other.data[1][c]
                    + self.data[r][2] * other.data[2][c]
                    + self.data[r][3] * other.data[3][c]
            }
        }
        new_matrix
    }
}

impl Mul<Tuple> for Matrix {
    type Output = Tuple;
    fn mul(self, other: Tuple) -> Tuple {
        debug_assert_eq!(
            self.data.len(),
            4,
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

impl Matrix {
    // TODO: would it be better to mutate instead of copying?
    pub fn transpose(&self) -> Matrix {
        // debug_assert!(self.rows == 4 && self.columns == 4, "Only 4x4 matrices can be tr");
        let mut m = build_matrix(self.columns, self.rows);
        for row in 0..self.rows {
            for col in 0..self.columns {
                m.data[col][row] = self.data[row][col];
            }
        }
        m
    }

    pub fn determinant(&self) -> f32 {
        debug_assert!(
            self.rows == 2 && self.columns == 2,
            "Can only take determinant of 2x2 matrix (this one is {}x{})",
            self.rows,
            self.columns
        );
        determinant(
            self.data[0][0],
            self.data[0][1],
            self.data[1][0],
            self.data[1][1],
        )
    }

    // for an nxn matrix, return an n-1 x n-1 matrix with remove_row row and remove_col col removed
    pub fn submatrix(&self, remove_row: usize, remove_col: usize) -> Matrix {
        let mut m = build_matrix(self.rows - 1, self.columns - 1);
        let mut new_row = 0;
        for old_row in 0..self.rows {
            if old_row == remove_row {
                continue;
            }
            let mut new_col = 0;
            for old_col in 0..self.columns {
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

    pub fn minor(&self, row: usize, column: usize) -> f32 {
        debug_assert!(
            self.rows == 3 && self.columns == 3,
            "Can only take minor of 3x3 matrix (this one is {}x{})",
            self.rows,
            self.columns
        );
        self.submatrix(row, column).determinant()
    }
}

// A is top left, d is bottom right of matrix
fn determinant(a: f32, b: f32, c: f32, d: f32) -> f32 {
    a * d - b * c
}

#[cfg(test)]
mod tests {
    use super::*;

    //     ​ 	​Scenario​: A matrix multiplied by a tuple
    // ​ 	  ​Given​ the following matrix A:
    // ​ 	      | 1 | 2 | 3 | 4 |
    // ​ 	      | 2 | 4 | 4 | 2 |
    // ​ 	      | 8 | 6 | 4 | 1 |
    // ​ 	      | 0 | 0 | 0 | 1 |
    // ​ 	    ​And​ b ← tuple(1, 2, 3, 1)
    #[test]
    fn test_matrix_multiplied_by_tuple() {
        let mut matrix_a = build_matrix(4, 4);
        matrix_a.data[0][0] = 1.0;
        matrix_a.data[0][1] = 2.0;
        matrix_a.data[0][2] = 3.0;
        matrix_a.data[0][3] = 4.0;

        matrix_a.data[1][0] = 2.0;
        matrix_a.data[1][1] = 4.0;
        matrix_a.data[1][2] = 4.0;
        matrix_a.data[1][3] = 2.0;

        matrix_a.data[2][0] = 8.0;
        matrix_a.data[2][1] = 6.0;
        matrix_a.data[2][2] = 4.0;
        matrix_a.data[2][3] = 1.0;

        matrix_a.data[3][0] = 0.0;
        matrix_a.data[3][1] = 0.0;
        matrix_a.data[3][2] = 0.0;
        matrix_a.data[3][3] = 1.0;

        let b = build_tuple(1.0, 2.0, 3.0, 1.0);

        // ​ 	  ​Then​ A * b = tuple(18, 24, 33, 1)
        assert_eq!(matrix_a * b, build_tuple(18.0, 24.0, 33.0, 1.0));
    }

    //     ​ 	​Scenario​: Multiplying two matrices
    // ​ 	  ​Given​ the following matrix A:
    // ​ 	      | 1 | 2 | 3 | 4 |
    // ​ 	      | 5 | 6 | 7 | 8 |
    // ​ 	      | 9 | 8 | 7 | 6 |
    // ​ 	      | 5 | 4 | 3 | 2 |
    // ​ 	    ​And​ the following matrix B:
    // ​ 	      | -2 | 1 | 2 |  3 |
    // ​ 	      |  3 | 2 | 1 | -1 |
    // ​ 	      |  4 | 3 | 6 |  5 |
    // ​ 	      |  1 | 2 | 7 |  8 |
    #[test]
    fn test_multiplying_two_matrices() {
        let mut matrix_a = build_matrix(4, 4);
        matrix_a.data[0][0] = 1.0;
        matrix_a.data[0][1] = 2.0;
        matrix_a.data[0][2] = 3.0;
        matrix_a.data[0][3] = 4.0;

        matrix_a.data[1][0] = 5.0;
        matrix_a.data[1][1] = 6.0;
        matrix_a.data[1][2] = 7.0;
        matrix_a.data[1][3] = 8.0;

        matrix_a.data[2][0] = 9.0;
        matrix_a.data[2][1] = 8.0;
        matrix_a.data[2][2] = 7.0;
        matrix_a.data[2][3] = 6.0;

        matrix_a.data[3][0] = 5.0;
        matrix_a.data[3][1] = 4.0;
        matrix_a.data[3][2] = 3.0;
        matrix_a.data[3][3] = 2.0;

        let mut matrix_b = build_matrix(4, 4);
        matrix_b.data[0][0] = -2.0;
        matrix_b.data[0][1] = 1.0;
        matrix_b.data[0][2] = 2.0;
        matrix_b.data[0][3] = 3.0;

        matrix_b.data[1][0] = 3.0;
        matrix_b.data[1][1] = 2.0;
        matrix_b.data[1][2] = 1.0;
        matrix_b.data[1][3] = -1.0;

        matrix_b.data[2][0] = 4.0;
        matrix_b.data[2][1] = 3.0;
        matrix_b.data[2][2] = 6.0;
        matrix_b.data[2][3] = 5.0;

        matrix_b.data[3][0] = 1.0;
        matrix_b.data[3][1] = 2.0;
        matrix_b.data[3][2] = 7.0;
        matrix_b.data[3][3] = 8.0;

        // ​ 	  ​Then​ A * B is the following 4x4 matrix:
        // ​ 	      | 20|  22 |  50 |  48 |
        // ​ 	      | 44|  54 | 114 | 108 |
        // ​ 	      | 40|  58 | 110 | 102 |
        // ​ 	      | 16|  26 |  46 |  42 |

        let mut expected = build_matrix(4, 4);
        expected.data[0][0] = 20.0;
        expected.data[0][1] = 22.0;
        expected.data[0][2] = 50.0;
        expected.data[0][3] = 48.0;

        expected.data[1][0] = 44.0;
        expected.data[1][1] = 54.0;
        expected.data[1][2] = 114.0;
        expected.data[1][3] = 108.0;

        expected.data[2][0] = 40.0;
        expected.data[2][1] = 58.0;
        expected.data[2][2] = 110.0;
        expected.data[2][3] = 102.0;

        expected.data[3][0] = 16.0;
        expected.data[3][1] = 26.0;
        expected.data[3][2] = 46.0;
        expected.data[3][3] = 42.0;
        assert_eq!(&matrix_a * &matrix_b, expected);
    }

    #[test]
    fn test_multiplying_by_identity_matrix() {
        let mut matrix_a = build_matrix(4, 4);
        matrix_a.data[0][0] = 0.0;
        matrix_a.data[0][1] = 1.0;
        matrix_a.data[0][2] = 2.0;
        matrix_a.data[0][3] = 4.0;

        matrix_a.data[1][0] = 1.0;
        matrix_a.data[1][1] = 2.0;
        matrix_a.data[1][2] = 4.0;
        matrix_a.data[1][3] = 8.0;

        matrix_a.data[2][0] = 2.0;
        matrix_a.data[2][1] = 4.0;
        matrix_a.data[2][2] = 8.0;
        matrix_a.data[2][3] = 16.0;

        matrix_a.data[3][0] = 4.0;
        matrix_a.data[3][1] = 8.0;
        matrix_a.data[3][2] = 16.0;
        matrix_a.data[3][3] = 32.0;

        let matrix_i = identity_4x4();
        assert_eq!(&matrix_a * &matrix_i, matrix_a);
    }

    #[test]
    fn test_matrix_transpose() {
        let mut m = build_matrix(4, 3);
        m.data[0][0] = 0.0;
        m.data[0][1] = 9.0;
        m.data[0][2] = 3.0;

        m.data[1][0] = 9.0;
        m.data[1][1] = 8.0;
        m.data[1][2] = 0.0;

        m.data[2][0] = 1.0;
        m.data[2][1] = 8.0;
        m.data[2][2] = 5.0;

        m.data[3][0] = 0.0;
        m.data[3][1] = 0.0;
        m.data[3][2] = 5.0;

        let mut m_transpose = build_matrix(3, 4);
        m_transpose.data[0][0] = 0.0;
        m_transpose.data[1][0] = 9.0;
        m_transpose.data[2][0] = 3.0;

        m_transpose.data[0][1] = 9.0;
        m_transpose.data[1][1] = 8.0;
        m_transpose.data[2][1] = 0.0;

        m_transpose.data[0][2] = 1.0;
        m_transpose.data[1][2] = 8.0;
        m_transpose.data[2][2] = 5.0;

        m_transpose.data[0][3] = 0.0;
        m_transpose.data[1][3] = 0.0;
        m_transpose.data[2][3] = 5.0;

        assert_eq!(m.transpose(), m_transpose);
    }

    #[test]
    fn test_transposing_identity_is_identity() {
        let matrix_i = identity_4x4();
        let transposed = matrix_i.transpose();
        assert_eq!(transposed, matrix_i);
    }

    #[test]
    fn test_determinant() {
        assert_eq!(determinant(1.0, 5.0, -3.0, 2.0), 17.0);
    }

    #[test]
    fn test_submatrix_of_3x3() {
        let mut matrix_a = build_matrix(3, 3);
        matrix_a.data[0][0] = 1.0;
        matrix_a.data[0][1] = 5.0;
        matrix_a.data[0][2] = 0.0;

        matrix_a.data[1][0] = -3.0;
        matrix_a.data[1][1] = 2.0;
        matrix_a.data[1][2] = 7.0;

        matrix_a.data[2][0] = 0.0;
        matrix_a.data[2][1] = 6.0;
        matrix_a.data[2][2] = -3.0;

        let mut expected_submatrix = build_matrix(2, 2);
        expected_submatrix.data[0][0] = -3.0;
        expected_submatrix.data[0][1] = 2.0;

        expected_submatrix.data[1][0] = -0.0;
        expected_submatrix.data[1][1] = 6.0;

        assert_eq!(matrix_a.submatrix(0, 2), expected_submatrix);
    }

    #[test]
    fn test_submatrix_of_4x4() {
        let mut matrix_a = build_matrix(4, 4);
        matrix_a.data[0][0] = -6.0;
        matrix_a.data[0][1] = 1.0;
        matrix_a.data[0][2] = 1.0;
        matrix_a.data[0][3] = 6.0;

        matrix_a.data[1][0] = -8.0;
        matrix_a.data[1][1] = 5.0;
        matrix_a.data[1][2] = 8.0;
        matrix_a.data[1][3] = 6.0;

        matrix_a.data[2][0] = -1.0;
        matrix_a.data[2][1] = 0.0;
        matrix_a.data[2][2] = 8.0;
        matrix_a.data[2][3] = 2.0;

        matrix_a.data[3][0] = -7.0;
        matrix_a.data[3][1] = 1.0;
        matrix_a.data[3][2] = -1.0;
        matrix_a.data[3][3] = 1.0;

        let mut expected_submatrix = build_matrix(3, 3);
        expected_submatrix.data[0][0] = -6.0;
        expected_submatrix.data[0][1] = 1.0;
        expected_submatrix.data[0][2] = 6.0;

        expected_submatrix.data[1][0] = -8.0;
        expected_submatrix.data[1][1] = 8.0;
        expected_submatrix.data[1][2] = 6.0;

        expected_submatrix.data[2][0] = -7.0;
        expected_submatrix.data[2][1] = -1.0;
        expected_submatrix.data[2][2] = 1.0;

        assert_eq!(matrix_a.submatrix(2, 1), expected_submatrix);
    }

    #[test]
    fn test_minor_of_3x3_matrix() {
        //         ​ 	​Scenario​: Calculating a minor of a 3x3 matrix
        // ​ 	  ​Given​ the following 3x3 matrix A:
        // ​ 	      |  3 |  5 |  0 |
        // ​ 	      |  2 | -1 | -7 |
        // ​ 	      |  6 | -1 |  5 |
        // ​ 	    ​And​ B ← submatrix(A, 1, 0)
        // ​ 	  ​Then​ determinant(B) = 25
        // ​ 	    ​And​ minor(A, 1, 0) = 25

        let mut matrix_a = build_matrix(3, 3);
        matrix_a.data[0][0] = 3.0;
        matrix_a.data[0][1] = 5.0;
        matrix_a.data[0][2] = 0.0;

        matrix_a.data[1][0] = 2.0;
        matrix_a.data[1][1] = -1.0;
        matrix_a.data[1][2] = -7.0;

        matrix_a.data[2][0] = 6.0;
        matrix_a.data[2][1] = -1.0;
        matrix_a.data[2][2] = 5.0;

        assert_eq!(matrix_a.minor(1, 0), 25.0);
    }
}
