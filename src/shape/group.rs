use crate::intersection::Intersection;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::shape::base_shape::BaseShape;
use crate::shape::shape::Shape;
use crate::tuple::Tuple;
use std::cmp::Ordering::Equal;

#[derive(Debug)]
pub struct GroupShape {
    base: BaseShape,
    children: Vec<Box<dyn Shape>>,
}

impl GroupShape {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add_child(&mut self, mut child: Box<dyn Shape>) {
        child.as_mut().set_parent(self);
        self.children.push(child);
    }
}

impl Default for GroupShape {
    fn default() -> GroupShape {
        GroupShape {
            base: BaseShape::new(),
            children: vec![],
        }
    }
}

impl Shape for GroupShape {
    fn get_base(&self) -> &BaseShape {
        &self.base
    }
    fn get_base_mut(&mut self) -> &mut BaseShape {
        &mut self.base
    }
    fn local_intersect(&self, object_ray: Ray) -> Vec<Intersection> {
        let mut intersections = vec![];
        for c in &mut self.children.iter() {
            for i in c.intersect(object_ray) {
                intersections.push(i);
            }
        }
        intersections.sort_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap_or(Equal));
        intersections
    }
    fn local_norm_at(&self, object_point: Tuple) -> Tuple {
        vector!(0, 0, 0)
    }
    fn transformation_inverse(&self) -> &Matrix {
        self.base.transformation_inverse()
    }
    fn transformation_inverse_transpose(&self) -> &Matrix {
        self.base.transformation_inverse_transpose()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shape::base_shape::BaseShape;
    use crate::shape::sphere::Sphere;
    use crate::transformations::scaling;
    use crate::transformations::translation;
    use std::ptr;

    #[test]
    fn add_child_to_group() {
        let s = Box::new(BaseShape::new());
        // TODO: this is possibly kind of a fragile test
        let s_address = s.as_ref() as *const dyn Shape;
        let mut g = GroupShape::new();
        g.add_child(s);
        assert_eq!(g.children.len(), 1, "g should have 1 child,");
        assert_eq!(
            g.children[0].as_ref() as *const _,
            s_address,
            "the one child should be s,"
        );
        assert!(
            ptr::eq(g.children[0].get_parent().unwrap(), &g),
            "and s's parent should be g"
        );
    }

    #[test]
    fn intersect_ray_with_empty_group() {
        let g = GroupShape::new();
        let r = Ray::new(point!(0, 0, 0), vector!(0, 0, 1));
        let xs = g.local_intersect(r);
        assert!(xs.is_empty());
    }

    #[test]
    fn intersect_ray_with_nonempty_group() {
        let (s1, mut s2, mut s3) = (Sphere::new(), Sphere::new(), Sphere::new());
        // TODO: we are just saving these to differentiate shapes later.
        // Everything would probably be easier if shapes had ID's of some kind.
        let s1_transformation = Matrix::default();
        let s2_transformation = translation(0.0, 0.0, -3.0);
        let s3_transformation = translation(5.0, 0.0, 0.0);
        s2.set_transformation(s2_transformation.clone());
        s3.set_transformation(s3_transformation);

        let mut g = GroupShape::new();
        for s in vec![s1, s2, s3] {
            g.add_child(Box::new(s));
        }

        let r = Ray::new(point!(0, 0, -5), vector!(0, 0, 1));
        let xs = g.local_intersect(r);
        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].object.transformation(), &s2_transformation);
        assert_eq!(xs[1].object.transformation(), &s2_transformation);
        assert_eq!(xs[2].object.transformation(), &s1_transformation);
        assert_eq!(xs[3].object.transformation(), &s1_transformation);
    }

    #[test]
    fn intersect_ray_with_transformed_group() {
        // tests that rays are correctly transformed by both parent and
        // child transformation matrices
        let mut g = GroupShape::new();
        g.set_transformation(scaling(2.0, 2.0, 2.0));
        let mut s = Sphere::new();
        s.set_transformation(translation(5.0, 0.0, 0.0));
        g.add_child(Box::new(s));
        let r = Ray::new(point!(10, 0, -10), vector!(0, 0, 1));
        let xs = g.intersect(r);
        assert_eq!(xs.len(), 2);
    }
}
