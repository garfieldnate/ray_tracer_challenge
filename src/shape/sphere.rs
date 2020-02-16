use crate::material::Material;
use crate::matrix::Matrix;
use crate::ray::build_intersection;
use crate::ray::Intersection;
use crate::ray::Ray;
use crate::shape::shape::BaseShape;
use crate::shape::shape::Shape;
use crate::tuple::{build_tuple, Tuple};

#[derive(Clone, Debug, PartialEq)]
pub struct Sphere {
    base: BaseShape,
    center: Tuple,
}

pub fn build_sphere(transform: Matrix, material: Material) -> Sphere {
    let mut s = Sphere::new();
    s.set_transformation(transform);
    s.set_material(material);
    s
}

impl Shape for Sphere {
    fn new() -> Self {
        Sphere {
            center: point!(0, 0, 0),
            base: BaseShape::new(),
        }
    }
    fn local_intersect(&self, object_ray: Ray) -> Vec<Intersection> {
        // ​# the vector from the sphere's center to the ray origin​
        let sphere_to_ray = object_ray.origin - self.center;
        // println!("sphere to ray: {:?}", sphere_to_ray);
        let a = object_ray.direction.dot(object_ray.direction);
        // println!("a: {}", a);
        let b = 2.0 * object_ray.direction.dot(sphere_to_ray);
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
    fn local_norm_at(&self, object_point: Tuple) -> Tuple {
        object_point - self.center
    }

    // forward these to BaseShape (TODO: need delegation RFC to be accepted!)
    fn transformation(&self) -> &Matrix {
        &self.base.transformation()
    }
    fn set_transformation(&mut self, t: Matrix) {
        self.base.set_transformation(t);
    }
    fn material(&self) -> Material {
        self.base.material()
    }
    fn set_material(&mut self, m: Material) {
        self.base.set_material(m);
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
    use std::f32::consts::FRAC_1_SQRT_2;
    use std::f32::consts::PI;

    fn frac_1_sqrt_3() -> f32 {
        1.0 / (3f32.sqrt())
    }

    #[test]
    fn intersect_scaled_sphere_with_ray() {
        let r = build_ray(point!(0, 0, -5), vector!(0, 0, 1));
        let mut s = Sphere::new();
        s.set_transformation(scaling(2.0, 2.0, 2.0));
        let xs = s.intersect(r);
        assert_eq!(xs[0].distance, 3.0);
        assert_eq!(xs[1].distance, 7.0);
    }

    #[test]
    fn intersect_translated_sphere_with_ray() {
        let r = build_ray(point!(0, 0, -5), vector!(0, 0, 1));
        let mut s = Sphere::new();
        s.set_transformation(translation(5.0, 0.0, 0.0));
        let xs = s.intersect(r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn sphere_normal_on_x_axis() {
        let s = Sphere::new();
        let n = s.normal_at(point!(1, 0, 0));
        assert_eq!(n, vector!(1, 0, 0));
    }

    #[test]
    fn sphere_normal_on_y_axis() {
        let s = Sphere::new();
        let n = s.normal_at(point!(0, 1, 0));
        assert_eq!(n, vector!(0, 1, 0));
    }

    #[test]
    fn sphere_normal_on_z_axis() {
        let s = Sphere::new();
        let n = s.normal_at(point!(0, 0, 1));
        assert_eq!(n, vector!(0, 0, 1));
    }

    #[test]
    fn sphere_normal_on_nonaxial_point() {
        let s = Sphere::new();
        let n = s.normal_at(point!(frac_1_sqrt_3(), frac_1_sqrt_3(), frac_1_sqrt_3()));
        assert_abs_diff_eq!(
            n,
            vector!(frac_1_sqrt_3(), frac_1_sqrt_3(), frac_1_sqrt_3())
        );
    }

    #[test]
    fn normal_is_normalized_vector() {
        let s = Sphere::new();
        let n = s.normal_at(point!(frac_1_sqrt_3(), frac_1_sqrt_3(), frac_1_sqrt_3()));
        assert_abs_diff_eq!(n, n.norm());
    }

    #[test]
    fn normal_of_translated_sphere() {
        let mut s = Sphere::new();
        s.set_transformation(translation(0.0, 1.0, 0.0));
        let n = s.normal_at(point!(0, 1.70711, -0.70711));
        assert_abs_diff_eq!(n, vector!(0, 0.7071068, -0.70710677));
    }

    #[test]
    fn normal_of_transformed_sphere() {
        let mut s = Sphere::new();
        let m = &scaling(1.0, 0.5, 1.0) * &rotation_z(PI / 5.0);
        s.set_transformation(m);
        let n = s.normal_at(point!(0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2));
        assert_abs_diff_eq!(n, vector!(0, 0.97014254, -0.24253564));
    }
}