use crate::intersection::Intersection;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::shape::base_shape::BaseShape;
use crate::tuple::Tuple;
use std::fmt::Debug;
use std::hash::Hash;
use std::hash::Hasher;
use std::ptr;

pub trait Shape: Debug {
    // tthe BaseShape that the wrapping instance is delegating to
    fn get_base(&self) -> &BaseShape;
    fn get_base_mut(&mut self) -> &mut BaseShape;

    fn local_intersect(&self, object_ray: Ray) -> Vec<Intersection>;
    fn local_norm_at(&self, object_point: Tuple) -> Tuple;

    // The rest of these should not be overridden by Shape implementers

    fn transformation(&self) -> &Matrix {
        self.get_base().transformation()
    }
    fn set_transformation(&mut self, t: Matrix) {
        self.get_base_mut().set_transformation(t)
    }
    fn material(&self) -> &Material {
        self.get_base().material()
    }
    fn set_material(&mut self, m: Material) {
        self.get_base_mut().set_material(m)
    }
    fn casts_shadow(&self) -> bool {
        self.get_base().casts_shadow()
    }
    fn set_casts_shadow(&mut self, casts_shadow: bool) {
        self.get_base_mut().set_casts_shadow(casts_shadow)
    }
    // these allow BaseShape to cache the results
    fn transformation_inverse(&self) -> &Matrix {
        self.get_base().transformation_inverse()
    }
    fn transformation_inverse_transpose(&self) -> &Matrix {
        self.get_base().transformation_inverse_transpose()
    }

    // Inverse transform maps from world to object space
    fn world_to_object_point(&self, world_point: &Tuple) -> Tuple {
        self.transformation_inverse() * world_point
    }
    fn world_to_object_ray(&self, world_ray: &Ray) -> Ray {
        world_ray.transform(&self.transformation_inverse())
    }

    // When intersecting the shape with a ray, all shapes need to first convert the
    //ray into object space, transforming it by the inverse of the shape’s transformation
    //matrix.
    fn intersect(&self, world_ray: Ray) -> Vec<Intersection> {
        let object_ray = self.world_to_object_ray(&world_ray);
        self.local_intersect(object_ray)
    }

    fn normal_to_world(&self, object_normal: &Tuple) -> Tuple {
        // Then, after computing the normal they must transform it by the inverse of the
        // transpose of the transformation matrix, and then normalize the resulting vector
        // before returning it.
        // TODO: why the inverse transpose instead of just the inverse?
        let mut world_normal = self.transformation_inverse_transpose() * object_normal;
        // transpose of translation matrix will mess with w; manually setting it back
        // to 0 here is faster and simpler than avoiding the computation by taking the
        // 3x3 submatrix before the computation.
        world_normal.w = 0.0;

        world_normal.norm()
    }

    fn normal_at(&self, world_point: Tuple) -> Tuple {
        // When computing the normal vector, all shapes need to first convert the point to
        // object space, multiplying it by the inverse of the shape’s transformation matrix.
        let object_point = self.world_to_object_point(&world_point);
        let object_normal = self.local_norm_at(object_point);
        self.normal_to_world(&object_normal)
    }

    // should only be implemented by GroupShape
    fn get_children(&self) -> Option<&Vec<Box<dyn Shape>>> {
        None
    }
}

// I don't entirely understand why the lifetime params are required, but the compiler will not let us
// put shapes with lifetime params into a collection of Borrow values without them.

// Shapes are always globally unique. They are only equal if they are the same object
impl<'a> PartialEq for dyn Shape + 'a {
    fn eq(&self, other: &dyn Shape) -> bool {
        ptr::eq(self, other)
    }
}
impl<'a> Hash for dyn Shape + 'a {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        ptr::hash(self, hasher);
    }
}

// Shapes are always globally unique. They are only equal if they are the same object
impl<'a> Eq for dyn Shape + 'a {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shape::group::GroupShape;
    use crate::shape::sphere::Sphere;
    use crate::shape::test_shape::TestShape;
    use crate::transformations::rotation_y;
    use crate::transformations::rotation_z;
    use crate::transformations::scaling;
    use crate::transformations::translation;
    use std::f32::consts::FRAC_1_SQRT_2;
    use std::f32::consts::PI;

    #[test]
    fn intersect_scaled_shape_with_ray() {
        let r = Ray::new(point!(0, 0, -5), vector!(0, 0, 1));
        let mut s = TestShape::new();
        s.set_transformation(scaling(2.0, 2.0, 2.0));
        s.intersect(r);
        assert_eq!(
            s.saved_ray.into_inner().unwrap(),
            Ray::new(point!(0, 0, -2.5), vector!(0, 0, 0.5))
        );
    }

    #[test]
    fn intersect_translated_shape_with_ray() {
        let r = Ray::new(point!(0, 0, -5), vector!(0, 0, 1));
        let mut s = TestShape::new();
        s.set_transformation(translation(5.0, 0.0, 0.0));
        s.intersect(r);
        assert_eq!(
            s.saved_ray.into_inner().unwrap(),
            Ray::new(point!(-5, 0, -5), vector!(0, 0, 1))
        );
    }

    #[test]
    fn normal_on_translated_shape() {
        let mut s = TestShape::new();
        s.set_transformation(translation(0.0, 1.0, 0.0));
        let n = s.normal_at(point!(0, 1.70711, -0.70711));
        assert_abs_diff_eq!(n, vector!(0.0, 0.600_000_1, -0.799_999_95));
    }

    #[test]
    fn normal_on_transformed_shape() {
        let mut s = TestShape::new();
        s.set_transformation(&scaling(1.0, 0.5, 1.0) * &rotation_z(PI / 5.0));
        let n = s.normal_at(point!(0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2));
        assert_abs_diff_eq!(n, vector!(-0.083_526_63, 0.932_529_6, -0.351_300_3));
    }

    #[test]
    fn normal_is_normalized_vector() {
        let s = TestShape::new();
        let n = s.normal_at(point!(1, 5, 10));
        assert_abs_diff_eq!(n, n.norm());
    }

    #[test]
    fn converting_normal_from_object_to_world_space() {
        let frac_1_sqrt_3 = 1.0 / 3f32.sqrt();
        let g1_transform = rotation_y(PI / 2.0);
        let g2_transform = scaling(1.0, 2.0, 3.0);
        let s_transform = translation(5.0, 0.0, 0.0);
        let object_normal = vector!(frac_1_sqrt_3, frac_1_sqrt_3, frac_1_sqrt_3);

        let mut s = Sphere::new();
        s.set_transformation(s_transform.clone());
        let mut g2 = GroupShape::new();
        g2.set_transformation(g2_transform.clone());
        let mut g1 = GroupShape::new();
        g1.set_transformation(g1_transform.clone());

        g2.add_child(Box::new(s));
        g1.add_child(Box::new(g2));

        // lost ownership of these, so we have to dig them out again for testing...
        let g2 = g1.get_children().unwrap()[0].as_ref();
        let s = g2.get_children().unwrap()[0].as_ref();

        let n = s.normal_to_world(&object_normal);
        assert_abs_diff_eq!(n, vector!(0.28571427, 0.42857143, -0.85714287));
    }
}
