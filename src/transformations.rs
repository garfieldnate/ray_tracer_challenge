use crate::matrix::*;

pub fn translation(x: f32, y: f32, z: f32) -> Matrix {
    let mut transformation = identity_4x4();
    transformation.data[0][3] = x;
    transformation.data[1][3] = y;
    transformation.data[2][3] = z;
    transformation
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tuple::*;

    #[test]
    fn multiply_by_translation_matrix() {
        let transform = translation(5.0, -3.0, 2.0);
        let p = point(-3.0, 4.0, 5.0);
        assert_eq!(transform * p, point(2.0, 1.0, 7.0));
    }

    #[test]
    fn multiply_by_inverse_of_translation_matrix() {
        let transform = translation(5.0, -3.0, 2.0);
        let inverse_transform = transform.inverse();
        let p = point(-3.0, 4.0, 5.0);
        assert_eq!(inverse_transform * p, point(-8.0, 7.0, 3.0));
    }

    #[test]
    fn translation_does_not_affect_vector() {
        let transform = translation(5.0, -3.0, 2.0);
        let v = vector(-3.0, 4.0, 5.0);
        assert_eq!(transform * v, v);
    }
}
// ​ 	​Scenario​: Translation does not affect vectors
// ​ 	  ​Given​ transform ← translation(5, -3, 2)
// ​ 	    ​And​ v ← vector(-3, 4, 5)
// ​ 	   ​Then​ transform * v = v
