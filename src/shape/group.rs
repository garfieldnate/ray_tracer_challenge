use crate::intersection::Intersection;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::shape::base_shape::BaseShape;
use crate::shape::shape::Shape;
use crate::tuple::Tuple;
use std::cmp::Ordering::Equal;

// instead of using BaseShape for the transform here, we propagate transforms to the children and then
// locally always assume a transform of I, allowing children to do all actual ray transformations.
// This leads to fewer multiplications and also allows us to avoid linking to parent groups, which
// is a pain in the Rusty...
#[derive(Debug, Default)]
pub struct GroupShape {
    base: BaseShape,
    children: Vec<Box<dyn Shape>>,
}

impl GroupShape {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add_child(&mut self, mut child: Box<dyn Shape>) {
        // bake this group's transform into the child's existing transform
        let old_child_transform = child.transformation().clone();
        child.set_transformation(self.transformation() * &old_child_transform);
        self.children.push(child);
    }
}

impl Shape for GroupShape {
    fn get_base(&self) -> &BaseShape {
        &self.base
    }
    fn get_base_mut(&mut self) -> &mut BaseShape {
        &mut self.base
    }
    /// Note to clients: the children's transforms will have this group's transform baked in.
    /// To get the child in its origin form, call remove_child (not implemented)
    fn get_children(&self) -> Option<&Vec<Box<dyn Shape>>> {
        Some(&self.children)
    }
    fn set_transformation(&mut self, t: Matrix) {
        // loop over children and undo the previous transformation that was applied to them
        // by multiplying their transform by the inverse of this group's transform. Then
        // apply the new group transform.
        if self.children.len() > 0 {
            let child_transformer = &t * self.transformation_inverse();
            for c in self.children.iter_mut() {
                let old_child_transform = c.transformation().clone();
                c.set_transformation(&child_transformer * &old_child_transform);
            }
        }
        // important in case parent group needs to undo its own transform propagated to this group
        self.get_base_mut().set_transformation(t);
    }
    fn intersect(&self, world_ray: Ray) -> Vec<Intersection> {
        // skip world to local conversion for Group, since the transformation matrix is propagated to the children
        self.local_intersect(world_ray)
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
    fn local_norm_at(&self, _object_point: Tuple) -> Tuple {
        unreachable!("Groups do not have normals. This method should never be called.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shape::base_shape::BaseShape;
    use crate::shape::sphere::Sphere;
    use crate::transformations::rotation_y;
    use crate::transformations::scaling;
    use crate::transformations::translation;
    use crate::tuple::Tuple;
    use std::f32::consts::PI;

    #[test]
    fn add_child_to_group() {
        let s = Box::new(BaseShape::new());
        // TODO: this is possibly kind of a fragile test
        let s_address = s.as_ref() as *const dyn Shape;
        let mut g = GroupShape::new();
        g.add_child(s);
        assert_eq!(g.children.len(), 1, "g should have 1 child...");
        assert_eq!(
            g.children[0].as_ref() as *const _,
            s_address,
            " and the one child should be s"
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
    fn intersect_ray_with_transformed_group_set_transform_before_adding() {
        // tests that rays are correctly transformed by both parent and
        // child transformation matrices
        let mut g = GroupShape::new();
        g.set_transformation(scaling(2.0, 2.0, 2.0));
        let mut s = Sphere::new();
        s.set_transformation(translation(5.0, 0.0, 0.0));
        g.add_child(Box::new(s));

        assert_eq!(
            g.get_children().unwrap()[0].transformation(),
            &matrix!(
                [2.0, 0.0, 0.0, 10.0],
                [0.0, 2.0, 0.0, 0.0],
                [0.0, 0.0, 2.0, 0.0],
                [0.0, 0.0, 0.0, 1.0]
            )
        );

        let r = Ray::new(point!(10, 0, -10), vector!(0, 0, 1));
        let xs = g.intersect(r);
        assert_eq!(xs.len(), 2);
    }

    #[test]
    fn intersect_ray_with_transformed_group_set_transform_after_adding() {
        // tests that rays are correctly transformed by both parent and
        // child transformation matrices
        let mut g = GroupShape::new();
        let mut s = Sphere::new();
        s.set_transformation(translation(5.0, 0.0, 0.0));
        g.add_child(Box::new(s));
        g.set_transformation(scaling(2.0, 2.0, 2.0));
        let r = Ray::new(point!(10, 0, -10), vector!(0, 0, 1));

        assert_eq!(
            g.get_children().unwrap()[0].transformation(),
            &matrix!(
                [2.0, 0.0, 0.0, 10.0],
                [0.0, 2.0, 0.0, 0.0],
                [0.0, 0.0, 2.0, 0.0],
                [0.0, 0.0, 0.0, 1.0]
            )
        );

        let xs = g.intersect(r);
        assert_eq!(xs.len(), 2);
    }

    #[test]
    fn intersect_ray_with_transformed_group_set_transform_before_and_after_adding() {
        // tests that rays are correctly transformed by both parent and
        // child transformation matrices
        let mut g = GroupShape::new();
        let mut s = Sphere::new();
        s.set_transformation(translation(5.0, 0.0, 0.0));

        g.set_transformation(scaling(3.0, 4.0, 8.0));
        g.add_child(Box::new(s));
        g.set_transformation(scaling(2.0, 2.0, 2.0));
        let r = Ray::new(point!(10, 0, -10), vector!(0, 0, 1));

        assert_eq!(
            g.get_children().unwrap()[0].transformation(),
            &matrix!(
                [2.0, 0.0, 0.0, 10.0],
                [0.0, 2.0, 0.0, 0.0],
                [0.0, 0.0, 2.0, 0.0],
                [0.0, 0.0, 0.0, 1.0]
            )
        );

        let xs = g.intersect(r);
        assert_eq!(xs.len(), 2);
    }

    #[test]
    fn converting_point_in_child_from_world_to_object_space() {
        let mut g1 = GroupShape::new();
        g1.set_transformation(rotation_y(PI / 2.0));
        let mut g2 = GroupShape::new();
        g2.set_transformation(scaling(1.0, 2.0, 3.0));
        let mut s = Sphere::new();
        s.set_transformation(translation(5.0, 0.0, 0.0));
        g2.add_child(Box::new(s));
        g1.add_child(Box::new(g2));

        // lost ownership of these, so we have to dig them out again for testing...
        let g2 = g1.get_children().unwrap()[0].as_ref();
        let s = g2.get_children().unwrap()[0].as_ref();

        let p = s.world_to_object_point(&point!(-2, 0, -10));
        assert_abs_diff_eq!(p, point!(5.0, 0.0, -0.66666657));
    }
}
