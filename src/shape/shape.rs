use crate::material::default_material;
use crate::material::Material;
use crate::matrix::identity_4x4;
use crate::matrix::Matrix;
use crate::ray::Intersection;
use crate::ray::Ray;
use crate::tuple::Tuple;
use std::fmt::Debug;

pub trait Shape: Debug {
    fn transformation(&self) -> &Matrix;
    fn set_transformation(&mut self, t: Matrix);
    fn material(&self) -> Material;
    fn set_material(&mut self, m: Material);
    fn local_intersect(&self, object_ray: Ray) -> Vec<Intersection>;
    fn local_norm_at(&self, object_point: Tuple) -> Tuple;

    // These two should not be overridden by Shape implementers

    // When intersecting the shape with a ray, all shapes need to first convert the
    //ray into object space, transforming it by the inverse of the shape’s transformation
    //matrix.
    fn intersect(&self, world_ray: Ray) -> Vec<Intersection> {
        let object_ray = world_ray.transform(&self.transformation().inverse());
        self.local_intersect(object_ray)
    }

    fn normal_at(&self, world_point: Tuple) -> Tuple {
        // When computing the normal vector, all shapes need to first convert the point to
        // object space, multiplying it by the inverse of the shape’s transformation matrix.
        let object_point = &self.transformation().inverse() * &world_point;

        let object_normal = self.local_norm_at(object_point);

        // Then, after computing the normal they must transform it by the inverse of the
        // transpose of the transformation matrix, and then normalize the resulting vector
        // before returning it.
        // TODO: why the inverse transpose instead of just the inverse?
        let mut world_normal = &self.transformation().inverse().transpose() * &object_normal;
        // transpose of translation matrix will mess with w; manually setting it back
        // to 0 here is faster and simpler than avoiding the computation by taking the
        // 3x3 submatrix before the computation.
        world_normal.w = 0.0;
        world_normal.norm()
    }
}

// Other shape implementations are meant to delegate to this one where these defaults are acceptable.
// TODO: Maybe someday Rust will support delegation: https://github.com/rust-lang/rfcs/pull/2393
// like Kotlin does. Could also check out the ambassador crate.
// TODO: cache inverse and inverse transpose
#[derive(Clone, Debug, PartialEq)]
pub struct BaseShape {
    t: Matrix,
    m: Material,
}

impl BaseShape {
    pub fn new() -> Self {
        BaseShape {
            t: identity_4x4(),
            m: default_material(),
        }
    }
}

impl Shape for BaseShape {
    fn transformation(&self) -> &Matrix {
        &self.t
    }
    fn set_transformation(&mut self, t: Matrix) {
        self.t = t;
    }
    fn material(&self) -> Material {
        self.m
    }
    fn set_material(&mut self, m: Material) {
        self.m = m;
    }

    // These two methods cannot be delegated to
    fn local_intersect(&self, _object_ray: Ray) -> Vec<Intersection> {
        unimplemented!()
    }
    fn local_norm_at(&self, _object_point: Tuple) -> Tuple {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ray::build_ray;
    use crate::transformations::rotation_z;
    use crate::transformations::scaling;
    use crate::transformations::translation;
    use crate::tuple::build_tuple;
    use std::cell::RefCell;
    use std::f32::consts::FRAC_1_SQRT_2;
    use std::f32::consts::PI;

    #[derive(Debug)]
    struct TestShape {
        base: BaseShape,
        saved_ray: RefCell<Option<Ray>>,
    }

    impl TestShape {
        fn new() -> Self {
            TestShape {
                base: BaseShape::new(),
                saved_ray: RefCell::new(None),
            }
        }
    }

    impl Shape for TestShape {
        fn transformation(&self) -> &Matrix {
            &self.base.t
        }
        fn set_transformation(&mut self, t: Matrix) {
            self.base.t = t;
        }
        fn material(&self) -> Material {
            self.base.m
        }
        fn set_material(&mut self, m: Material) {
            self.base.m = m;
        }
        fn local_intersect(&self, _object_ray: Ray) -> Vec<Intersection> {
            // save the incoming ray for a comparison test
            self.saved_ray.borrow_mut().replace(_object_ray);
            vec![]
        }
        fn local_norm_at(&self, _object_point: Tuple) -> Tuple {
            // return something that will let us test both the input and output calculations
            vector!(
                2.0 * _object_point.x,
                3.0 * _object_point.y,
                4.0 * _object_point.z
            )
        }
    }

    #[test]
    fn shape_transformation() {
        let mut shape = BaseShape::new();
        assert_eq!(
            shape.transformation(),
            &identity_4x4(),
            "Default transform should be identity"
        );

        shape.set_transformation(translation(2.0, 3.0, 4.0));
        assert_eq!(
            shape.transformation(),
            &translation(2.0, 3.0, 4.0),
            "transformation should be settable"
        );
    }

    #[test]
    fn shape_material() {
        let mut shape = BaseShape::new();
        assert_eq!(shape.material(), default_material(), "Default material");

        let mut override_material = default_material();
        override_material.ambient = 1.0;
        shape.set_material(override_material);
        assert_eq!(
            shape.material(),
            override_material,
            "material should be settable"
        );
    }

    #[test]
    fn intersect_scaled_shape_with_ray() {
        let r = build_ray(point!(0, 0, -5), vector!(0, 0, 1));
        let mut s = TestShape::new();
        s.set_transformation(scaling(2.0, 2.0, 2.0));
        s.intersect(r);
        assert_eq!(
            s.saved_ray.into_inner().unwrap(),
            build_ray(point!(0, 0, -2.5), vector!(0, 0, 0.5))
        );
    }

    #[test]
    fn intersect_translated_shape_with_ray() {
        let r = build_ray(point!(0, 0, -5), vector!(0, 0, 1));
        let mut s = TestShape::new();
        s.set_transformation(translation(5.0, 0.0, 0.0));
        s.intersect(r);
        assert_eq!(
            s.saved_ray.into_inner().unwrap(),
            build_ray(point!(-5, 0, -5), vector!(0, 0, 1))
        );
    }

    #[test]
    fn normal_on_translated_shape() {
        let mut s = TestShape::new();
        s.set_transformation(translation(0.0, 1.0, 0.0));
        let n = s.normal_at(point!(0, 1.70711, -0.70711));
        assert_abs_diff_eq!(n, vector!(0.0, 0.6000001, -0.79999995));
    }

    #[test]
    fn normal_on_transformed_shape() {
        let mut s = TestShape::new();
        s.set_transformation(&scaling(1.0, 0.5, 1.0) * &rotation_z(PI / 5.0));
        let n = s.normal_at(point!(0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2));
        assert_abs_diff_eq!(n, vector!(-0.08352663, 0.9325296, -0.3513003));
    }

    #[test]
    fn normal_is_normalized_vector() {
        let s = TestShape::new();
        let n = s.normal_at(point!(1, 5, 10));
        assert_abs_diff_eq!(n, n.norm());
    }
}
