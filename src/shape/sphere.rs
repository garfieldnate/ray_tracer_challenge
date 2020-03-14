use crate::bounding_box::BoundingBox;
use crate::intersection::Intersection;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::shape::base_shape::BaseShape;
use crate::shape::shape::Shape;
use crate::tuple::Tuple;

// Base shape has radius of 1 and straddles world origin

#[derive(Debug)]
pub struct Sphere {
    base: BaseShape,
    center: Tuple,
}

impl Sphere {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(transform: Matrix, material: Material) -> Self {
        let mut s = Sphere::new();
        s.set_transformation(transform);
        s.set_material(material);
        s
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Sphere {
            center: point!(0, 0, 0),
            base: BaseShape::new(),
        }
    }
}

impl Shape for Sphere {
    fn get_base(&self) -> &BaseShape {
        &self.base
    }
    fn get_base_mut(&mut self) -> &mut BaseShape {
        &mut self.base
    }
    fn local_intersect(&self, object_ray: Ray) -> Vec<Intersection> {
        // the vector from the sphere's center to the ray origin
        let sphere_to_ray = object_ray.origin - self.center;
        // println!("sphere to ray: {:?}", sphere_to_ray);
        let a = object_ray.direction.dot(object_ray.direction);
        // println!("a: {}", a);
        let b = 2.0 * object_ray.direction.dot(sphere_to_ray);
        // println!("b: {}", b);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
        // println!("c: {}", c);
        let discriminant = b.powi(2) - 4.0 * a * c;
        // println!("discriminant: {}", discriminant);
        if discriminant < 0.0 {
            return vec![];
        }

        let two_a = 2.0 * a;
        let discriminant_sqrt = discriminant.sqrt();
        // Jingle bells!
        vec![
            Intersection::new((-b - discriminant_sqrt) / two_a, self),
            Intersection::new((-b + discriminant_sqrt) / two_a, self),
        ]
    }
    fn local_norm_at(&self, object_point: Tuple, _hit: &Intersection) -> Tuple {
        object_point - self.center
    }

    fn bounding_box(&self) -> BoundingBox {
        BoundingBox {
            min: point!(-1, -1, -1),
            max: point!(1, 1, 1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::utils::dummy_intersection;
    use crate::transformations::scaling;
    use crate::transformations::translation;

    fn frac_1_sqrt_3() -> f32 {
        1.0 / (3f32.sqrt())
    }

    #[test]
    fn intersect_scaled_sphere_with_ray() {
        let r = Ray::new(point!(0, 0, -2.5), vector!(0, 0, 0.5));
        let mut s = Sphere::new();
        s.set_transformation(scaling(2.0, 2.0, 2.0));
        let xs = s.local_intersect(r);
        assert_eq!(xs[0].distance, 3.0);
        assert_eq!(xs[1].distance, 7.0);
    }

    #[test]
    fn intersect_translated_sphere_with_ray() {
        let r = Ray::new(point!(-5, 0, -5), vector!(0, 0, 1));
        let mut s = Sphere::new();
        s.set_transformation(translation(5.0, 0.0, 0.0));
        let xs = s.local_intersect(r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn sphere_normal_on_x_axis() {
        let s = Sphere::new();
        let n = s.local_norm_at(point!(1, 0, 0), &dummy_intersection(&s));
        assert_eq!(n, vector!(1, 0, 0));
    }

    #[test]
    fn sphere_normal_on_y_axis() {
        let s = Sphere::new();
        let n = s.local_norm_at(point!(0, 1, 0), &dummy_intersection(&s));
        assert_eq!(n, vector!(0, 1, 0));
    }

    #[test]
    fn sphere_normal_on_z_axis() {
        let s = Sphere::new();
        let n = s.local_norm_at(point!(0, 0, 1), &dummy_intersection(&s));
        assert_eq!(n, vector!(0, 0, 1));
    }

    #[test]
    fn sphere_normal_on_nonaxial_point() {
        let s = Sphere::new();
        let n = s.local_norm_at(
            point!(frac_1_sqrt_3(), frac_1_sqrt_3(), frac_1_sqrt_3()),
            &dummy_intersection(&s),
        );
        assert_abs_diff_eq!(
            n,
            vector!(frac_1_sqrt_3(), frac_1_sqrt_3(), frac_1_sqrt_3())
        );
    }
}
