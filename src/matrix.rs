use crate::tuple::*;
use std::ops::Mul;

#[derive(Clone, Debug, PartialEq)]
struct Matrix {
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

// TODO: the self args should be &self to prevent moving; not sure how to do that
impl Mul for Matrix {
    type Output = Matrix;
    fn mul(self, other: Matrix) -> Matrix {
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
        if self.data.len() != 4 {
            panic!("Only 4x4 matrices can be multiplied by tuples!")
        }
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
        let mut A = build_matrix(4, 4);
        A.data[0][0] = 1.0;
        A.data[0][1] = 2.0;
        A.data[0][2] = 3.0;
        A.data[0][3] = 4.0;

        A.data[1][0] = 2.0;
        A.data[1][1] = 4.0;
        A.data[1][2] = 4.0;
        A.data[1][3] = 2.0;

        A.data[2][0] = 8.0;
        A.data[2][1] = 6.0;
        A.data[2][2] = 4.0;
        A.data[2][3] = 1.0;

        A.data[3][0] = 0.0;
        A.data[3][1] = 0.0;
        A.data[3][2] = 0.0;
        A.data[3][3] = 1.0;

        let b = build_tuple(1.0, 2.0, 3.0, 1.0);

        // ​ 	  ​Then​ A * b = tuple(18, 24, 33, 1)
        assert_eq!(A * b, build_tuple(18.0, 24.0, 33.0, 1.0));
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
        let mut A = build_matrix(4, 4);
        A.data[0][0] = 1.0;
        A.data[0][1] = 2.0;
        A.data[0][2] = 3.0;
        A.data[0][3] = 4.0;

        A.data[1][0] = 5.0;
        A.data[1][1] = 6.0;
        A.data[1][2] = 7.0;
        A.data[1][3] = 8.0;

        A.data[2][0] = 9.0;
        A.data[2][1] = 8.0;
        A.data[2][2] = 7.0;
        A.data[2][3] = 6.0;

        A.data[3][0] = 5.0;
        A.data[3][1] = 4.0;
        A.data[3][2] = 3.0;
        A.data[3][3] = 2.0;

        let mut B = build_matrix(4, 4);
        B.data[0][0] = -2.0;
        B.data[0][1] = 1.0;
        B.data[0][2] = 2.0;
        B.data[0][3] = 3.0;

        B.data[1][0] = 3.0;
        B.data[1][1] = 2.0;
        B.data[1][2] = 1.0;
        B.data[1][3] = -1.0;

        B.data[2][0] = 4.0;
        B.data[2][1] = 3.0;
        B.data[2][2] = 6.0;
        B.data[2][3] = 5.0;

        B.data[3][0] = 1.0;
        B.data[3][1] = 2.0;
        B.data[3][2] = 7.0;
        B.data[3][3] = 8.0;

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
        assert_eq!(A * B, expected);
    }
}
