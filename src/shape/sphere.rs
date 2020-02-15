use crate::material::default_material;
use crate::material::Material;
use crate::matrix::identity_4x4;
use crate::matrix::Matrix;
use crate::ray::build_intersection;
use crate::ray::Intersection;
use crate::ray::Ray;
use crate::tuple::{build_tuple, Tuple};

#[derive(Clone, Debug, PartialEq)]
pub struct Sphere {
    center: Tuple,
    transform: Matrix,
    pub material: Material,
}

pub fn default_sphere() -> Sphere {
    Sphere {
        center: point!(0, 0, 0),
        transform: identity_4x4(),
        material: default_material(),
    }
}

pub fn build_sphere(transform: Matrix, material: Material) -> Sphere {
    Sphere {
        center: point!(0, 0, 0),
        transform,
        material,
    }
}

impl Sphere {
    pub fn set_transform(&mut self, transform_matrix: Matrix) {
        self.transform = transform_matrix;
    }
    pub fn set_material(&mut self, m: Material) {
        self.material = m;
    }
}

impl Sphere {
    pub fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let transformed_ray = ray.transform(&self.transform.inverse());
        // ​# the vector from the sphere's center to the ray origin​
        let sphere_to_ray = transformed_ray.origin - self.center;
        // println!("sphere to ray: {:?}", sphere_to_ray);
        let a = transformed_ray.direction.dot(transformed_ray.direction);
        // println!("a: {}", a);
        let b = 2.0 * transformed_ray.direction.dot(sphere_to_ray);
        // println!("b: {}", b);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
        // println!("c: {}", c);
        let discriminant = b.powf(2.0) - 4.0 * a * c;
        // println!("discriminant: {}", discriminant);
        if discriminant < 0.0 {
            return vec![];
        }

        // Jingle bells!
        vec![
            build_intersection((-b - discriminant.sqrt()) / (2.0 * a), self),
            build_intersection((-b + discriminant.sqrt()) / (2.0 * a), self),
        ]
    }
    pub fn normal_at(&self, world_point: Tuple) -> Tuple {
        let object_point = &self.transform.inverse() * &world_point;
        let object_normal = object_point - self.center;
        // TODO: why?
        let mut world_normal = &self.transform.inverse().transpose() * &object_normal;
        // transpose of translation matrix will mess with w; manually setting it back
        // to 0 here is faster and simpler than avoiding the computation by taking the
        // 3x3 submatrix before the computation.
        world_normal.w = 0.0;
        world_normal.norm()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matrix::identity_4x4;
    use crate::ray::build_ray;
    use crate::shape::sphere::default_sphere;
    use crate::transformations::rotation_z;
    use crate::transformations::scaling;
    use crate::transformations::translation;
    use crate::tuple::build_tuple;
    use std::f32::consts::FRAC_1_SQRT_2;
    use std::f32::consts::PI;

    fn frac_1_sqrt_3() -> f32 {
        1.0 / (3f32.sqrt())
    }

    #[test]
    fn sphere_default_values() {
        let s = default_sphere();
        assert_eq!(s.transform, identity_4x4());
        assert_eq!(s.material, default_material());
    }

    #[test]
    fn set_sphere_values() {
        let mut s = default_sphere();
        let t = translation(2.0, 3.0, 4.0);
        let mut m = default_material();
        m.ambient = 1.0;
        s.set_transform(t.clone());
        s.set_material(m);
        assert_eq!(s.transform, t);
        assert_eq!(s.material, m);
    }

    #[test]
    fn intersect_scaled_sphere_with_ray() {
        let r = build_ray(point!(0, 0, -5), vector!(0, 0, 1));
        let mut s = default_sphere();
        s.set_transform(scaling(2.0, 2.0, 2.0));
        let xs = s.intersect(r);
        assert_eq!(xs[0].distance, 3.0);
        assert_eq!(xs[1].distance, 7.0);
    }

    #[test]
    fn intersect_translated_sphere_with_ray() {
        let r = build_ray(point!(0, 0, -5), vector!(0, 0, 1));
        let mut s = default_sphere();
        s.set_transform(translation(5.0, 0.0, 0.0));
        let xs = s.intersect(r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn sphere_normal_on_x_axis() {
        let s = default_sphere();
        let n = s.normal_at(point!(1, 0, 0));
        assert_eq!(n, vector!(1, 0, 0));
    }

    #[test]
    fn sphere_normal_on_y_axis() {
        let s = default_sphere();
        let n = s.normal_at(point!(0, 1, 0));
        assert_eq!(n, vector!(0, 1, 0));
    }

    #[test]
    fn sphere_normal_on_z_axis() {
        let s = default_sphere();
        let n = s.normal_at(point!(0, 0, 1));
        assert_eq!(n, vector!(0, 0, 1));
    }

    #[test]
    fn sphere_normal_on_nonaxial_point() {
        let s = default_sphere();
        let n = s.normal_at(point!(frac_1_sqrt_3(), frac_1_sqrt_3(), frac_1_sqrt_3()));
        assert_abs_diff_eq!(
            n,
            vector!(frac_1_sqrt_3(), frac_1_sqrt_3(), frac_1_sqrt_3())
        );
    }

    #[test]
    fn normal_is_normalized_vector() {
        let s = default_sphere();
        let n = s.normal_at(point!(frac_1_sqrt_3(), frac_1_sqrt_3(), frac_1_sqrt_3()));
        assert_abs_diff_eq!(n, n.norm());
    }

    #[test]
    fn normal_of_translated_sphere() {
        let mut s = default_sphere();
        s.set_transform(translation(0.0, 1.0, 0.0));
        let n = s.normal_at(point!(0, 1.70711, -0.70711));
        assert_abs_diff_eq!(n, vector!(0, 0.7071068, -0.70710677));
    }

    #[test]
    fn normal_of_transformed_sphere() {
        let mut s = default_sphere();
        let m = &scaling(1.0, 0.5, 1.0) * &rotation_z(PI / 5.0);
        s.set_transform(m);
        let n = s.normal_at(point!(0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2));
        assert_abs_diff_eq!(n, vector!(0, 0.97014254, -0.24253564));
    }
}
