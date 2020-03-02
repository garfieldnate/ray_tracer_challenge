use crate::intersection::Intersection;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::shape::base_shape::BaseShape;
use crate::shape::shape::Shape;
use crate::tuple::Tuple;
use std::f32;

// Base shape is y=0 (so an xz plane, extending into the screen as a floor)

#[derive(Debug)]
pub struct Plane {
    base: BaseShape,
}
impl Plane {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(transform: Matrix, material: Material) -> Self {
        let mut s = Plane::new();
        s.set_transformation(transform);
        s.set_material(material);
        s
    }
}

impl Default for Plane {
    fn default() -> Self {
        Plane {
            base: BaseShape::new(),
        }
    }
}

impl Shape for Plane {
    fn get_base(&self) -> &BaseShape {
        &self.base
    }
    fn get_base_mut(&mut self) -> &mut BaseShape {
        &mut self.base
    }
    fn local_intersect(&self, object_ray: Ray) -> Vec<Intersection> {
        // the plane is in the xz plane, so its y is 0.
        // if the ray is roughly coplanar or parallel with the plane,
        // we won't be able to see it
        if object_ray.direction.y.abs() < f32::EPSILON * 10000.0 {
            vec![]
        } else {
            // this formula works because the plain sits in the xz plane
            let distance = -object_ray.origin.y / object_ray.direction.y;
            vec![Intersection::new(distance, self)]
        }
    }
    fn local_norm_at(&self, _object_point: Tuple, _hit: &Intersection) -> Tuple {
        vector!(0, 1, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::utils::dummy_intersection;
    #[test]
    fn normal_of_plane_is_constant_everywhere() {
        let p = Plane::new();
        let n1 = p.local_norm_at(point!(0, 0, 0), &dummy_intersection(&p));
        let n2 = p.local_norm_at(point!(10, 0, -10), &dummy_intersection(&p));
        let n3 = p.local_norm_at(point!(-5, 0, 150), &dummy_intersection(&p));
        assert_eq!(n1, vector!(0, 1, 0));
        assert_eq!(n2, vector!(0, 1, 0));
        assert_eq!(n3, vector!(0, 1, 0));
    }

    #[test]
    fn intersect_with_parallel_ray() {
        let p = Plane::new();
        let r = Ray::new(point!(0, 10, 0), vector!(0, 0, 1));
        let xs = p.local_intersect(r);
        assert!(xs.is_empty());
    }

    #[test]
    fn intersect_with_coplanar_ray() {
        let p = Plane::new();
        let r = Ray::new(point!(0, 0, 0), vector!(0, 0, 1));
        let xs = p.local_intersect(r);
        assert!(xs.is_empty());
    }

    #[test]
    fn ray_intersects_plane_from_above() {
        let p = Plane::new();
        let r = Ray::new(point!(0, 1, 0), vector!(0, -1, 0));
        let xs = p.local_intersect(r);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].distance, 1.0);
    }

    #[test]
    fn ray_intersects_plane_from_below() {
        let p = Plane::new();
        let r = Ray::new(point!(0, -1, 0), vector!(0, 1, 0));
        let xs = p.local_intersect(r);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].distance, 1.0);
    }
}
