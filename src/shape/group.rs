use crate::bounding_box::BoundingBox;
use crate::intersection::Intersection;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::shape::base_shape::BaseShape;
use crate::shape::shape::Shape;
use crate::tuple::Tuple;
use std::cell::RefCell;
use std::cmp::Ordering::Equal;

// instead of using BaseShape for the transform here, we propagate transforms to the children and then
// locally always assume a transform of I, allowing children to do all actual ray transformations.
// This leads to fewer multiplications and also allows us to avoid linking to parent groups, which
// is a pain in the Rusty...
#[derive(Debug, Default)]
pub struct GroupShape {
    base: BaseShape,
    children: Vec<Box<dyn Shape>>,
    cached_bounding_box: RefCell<Option<BoundingBox>>,
}

impl GroupShape {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_children(children: Vec<Box<dyn Shape>>) -> Self {
        let mut g = Self::default();
        g.children = children;
        g
    }

    /// Note to clients: the children's transforms will have this group's transform baked in.
    /// To get the child in its origin form, call remove_child (not implemented)
    pub fn get_children(&self) -> &Vec<Box<dyn Shape>> {
        &self.children
    }

    pub fn add_child(&mut self, mut child: Box<dyn Shape>) {
        // bake this group's transform into the child's existing transform
        let old_child_transform = child.transformation().clone();
        child.set_transformation(self.transformation() * &old_child_transform);
        self.children.push(child);
    }

    // Meant ONLY to be used by divide because returned left and right children will
    // still have the group's transform baked into their own.
    fn partition_children(&mut self) -> (Vec<Box<dyn Shape>>, Vec<Box<dyn Shape>>) {
        let (left_bounds, right_bounds) = self.bounding_box().split();
        let mut left = vec![];
        let mut right = vec![];
        let mut new_children = vec![];
        for c in self.children.drain(..) {
            let child_bounds = c.as_ref().parent_space_bounding_box();
            if left_bounds.contains_bounding_box(child_bounds) {
                left.push(c);
            } else if right_bounds.contains_bounding_box(child_bounds) {
                right.push(c);
            } else {
                new_children.push(c)
            }
        }
        self.children = new_children;
        (left, right)
    }

    // Meant ONLY to be used by divide because it does NOT push down this group's
    // transformation (partition children left the transformation baked in).
    fn make_subgroup(&mut self, mut new_group_children: Vec<Box<dyn Shape>>) {
        // don't bother wrapping a single shape in another group object
        if new_group_children.len() == 1 {
            self.children.push(new_group_children.remove(0));
        } else {
            let new_child = GroupShape::with_children(new_group_children);
            self.children.push(Box::new(new_child));
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
    fn includes(&self, other: &dyn Shape) -> bool {
        if self.get_unique_id() == other.get_unique_id() {
            true
        } else {
            self.children.iter().any(|s| s.as_ref().includes(other))
        }
    }
    // just pass the material on to the children
    // TODO: could be very inefficient for large groups
    fn set_material(&mut self, m: Material) {
        for child in &mut self.children.iter_mut() {
            child.set_material(m.clone());
        }
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

        let b = self.bounding_box();
        if !b.intersects(object_ray) {
            return intersections;
        }

        for c in &mut self.children.iter() {
            for i in c.intersect(object_ray) {
                intersections.push(i);
            }
        }
        intersections.sort_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap_or(Equal));
        intersections
    }
    fn local_norm_at(&self, _object_point: Tuple, _hit: &Intersection) -> Tuple {
        unreachable!("Groups do not have normals. This method should never be called.")
    }

    fn bounding_box(&self) -> BoundingBox {
        let mut cached_box = self.cached_bounding_box.borrow_mut();
        cached_box.get_or_insert_with(|| {
            let mut b = BoundingBox::empty();

            for child in &mut self.children.iter() {
                let child_box = child.parent_space_bounding_box();
                b.add_bounding_box(child_box);
            }
            b
        });
        cached_box.unwrap()
    }

    fn parent_space_bounding_box(&self) -> BoundingBox {
        // transformation for self is always pushed down to children, so we can't use shape's default implementation here.
        // TODO: put self.transformation in a separate field so that we don't have to override this here.
        self.bounding_box()
    }

    fn divide(&mut self, threshold: usize) {
        if threshold <= self.children.len() {
            let (left, right) = self.partition_children();
            if !left.is_empty() {
                self.make_subgroup(left);
            }
            if !right.is_empty() {
                self.make_subgroup(right);
            }
        }

        for child in &mut self.children.iter_mut() {
            child.divide(threshold);
        }
    }
}

impl Clone for GroupShape {
    fn clone(&self) -> Self {
        Self {
            base: self.base.clone(),
            children: self.children.clone(),
            cached_bounding_box: RefCell::new(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shape::base_shape::BaseShape;
    use crate::shape::cylinder::Cylinder;
    use crate::shape::sphere::Sphere;
    use crate::shape::test_shape::TestShape;
    use crate::test::utils::dummy_intersection;
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
    fn material_is_propagated_to_children() {
        let mut g = GroupShape::with_children(vec![
            Box::new(Sphere::new()),
            Box::new(Sphere::new()),
            Box::new(Sphere::new()),
        ]);
        let group_shininess = 123.456;
        g.set_material(Material::builder().shininess(group_shininess).build());
        for c in g.get_children().iter() {
            assert_eq!(c.material().shininess, group_shininess);
        }
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
            g.get_children()[0].transformation(),
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
            g.get_children()[0].transformation(),
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
            g.get_children()[0].transformation(),
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
        let g2 = g1.get_children()[0]
            .as_ref()
            .downcast_ref::<GroupShape>()
            .unwrap();
        let s = g2.get_children()[0].as_ref();

        let p = s.world_to_object_point(&point!(-2, 0, -10));
        assert_abs_diff_eq!(p, point!(5.0, 0.0, -0.66666657));
    }

    #[test]
    fn finding_normal_on_child_object() {
        let g1_transform = rotation_y(PI / 2.0);
        let g2_transform = scaling(1.0, 2.0, 3.0);
        let s_transform = translation(5.0, 0.0, 0.0);
        let world_point = point!(1.7321, 1.1547, -5.5774);

        let mut s = Sphere::new();
        s.set_transformation(s_transform.clone());
        let mut g2 = GroupShape::new();
        g2.set_transformation(g2_transform.clone());
        let mut g1 = GroupShape::new();
        g1.set_transformation(g1_transform.clone());

        g2.add_child(Box::new(s));
        g1.add_child(Box::new(g2));

        // lost ownership of these, so we have to dig them out again for testing...
        let g2 = g1.get_children()[0]
            .as_ref()
            .downcast_ref::<GroupShape>()
            .unwrap();
        let s = g2.get_children()[0].as_ref();

        let n = s.normal_at(&world_point, &dummy_intersection(&g1));
        assert_abs_diff_eq!(n, vector!(0.2857036, 0.42854306, -0.8571606));
    }

    #[test]
    fn group_bounding_box_contains_children() {
        let mut s = Sphere::new();
        s.set_transformation(translation(2., 5., -3.) * scaling(2., 2., 2.));

        let mut c = Cylinder::new();
        c.minimum_y = -2.;
        c.maximum_y = 2.;
        c.set_transformation(translation(-4., -1., 4.) * scaling(0.5, 1., 0.5));

        let mut shape = GroupShape::new();
        shape.add_child(Box::new(s));
        shape.add_child(Box::new(c));

        let b = shape.bounding_box();

        assert_eq!(b.min, point!(-4.5, -3, -5));
        assert_eq!(b.max, point!(4, 7, 4.5));
    }

    #[test]
    fn group_parent_space_bounding_box_ignores_passed_down_transformation() {
        let mut s = Sphere::new();
        s.set_transformation(scaling(2., 2., 2.));

        let mut c = Cylinder::new();
        c.minimum_y = -1.;
        c.maximum_y = 1.;
        c.set_transformation(scaling(2., 2., 2.));

        let mut shape = GroupShape::new();
        shape.add_child(Box::new(s));
        shape.add_child(Box::new(c));
        shape.set_transformation(scaling(0.5, 0.5, 0.5));

        let b1 = shape.bounding_box();
        let b2 = shape.parent_space_bounding_box();
        assert_eq!(b1, b2);
    }

    #[test]
    fn ray_intersection_doesnt_test_children_if_bounding_box_is_missed() {
        let child = TestShape::new();
        let mut shape = GroupShape::new();
        shape.add_child(Box::new(child));
        let r = Ray::new(point!(0, 0, -5), vector!(0, 1, 0));
        shape.intersect(r);

        let test_shape = shape.get_children()[0].downcast_ref::<TestShape>().unwrap();
        println!("{:?}", test_shape.saved_ray.borrow());
        assert!(test_shape.saved_ray.borrow().is_none());
    }

    #[test]
    fn ray_intersection_tests_children_if_bounding_box_is_hit() {
        let child = TestShape::new();
        let mut shape = GroupShape::new();
        shape.add_child(Box::new(child));
        let r = Ray::new(point!(0, 0, -5), vector!(0, 0, 1));
        shape.intersect(r);

        let test_shape = shape.get_children()[0].downcast_ref::<TestShape>().unwrap();
        println!("{:?}", test_shape.saved_ray.borrow());
        assert!(test_shape.saved_ray.borrow().is_some());
    }

    #[test]
    fn partitioning_children() {
        let mut s1 = Sphere::new();
        s1.set_transformation(translation(-2., 0., 0.));
        let s1_id = s1.get_unique_id();

        let mut s2 = Sphere::new();
        s2.set_transformation(translation(2., 0., 0.));
        let s2_id = s2.get_unique_id();

        let s3 = Sphere::new();
        let s3_id = s3.get_unique_id();

        let mut g = GroupShape::new();
        g.add_child(Box::new(s1));
        g.add_child(Box::new(s2));
        g.add_child(Box::new(s3));

        let (left, right) = g.partition_children();

        // g should contain s3
        let g_children = g.get_children();
        assert_eq!(g_children.len(), 1);
        assert_eq!(g_children[0].get_unique_id(), s3_id);

        // left should contain s1
        assert_eq!(left.len(), 1);
        assert_eq!(left[0].get_unique_id(), s1_id);

        // right should contain s2
        assert_eq!(right.len(), 1);
        assert_eq!(right[0].get_unique_id(), s2_id);
    }

    #[test]
    fn creating_subgroup_from_list_of_children() {
        let s1 = Sphere::new();
        let s1_id = s1.get_unique_id();

        let s2 = Sphere::new();
        let s2_id = s2.get_unique_id();

        let mut g = GroupShape::new();
        g.make_subgroup(vec![Box::new(s1), Box::new(s2)]);

        let g_children = g.get_children();
        assert_eq!(g_children.len(), 1);

        let g_child = g_children[0].downcast_ref::<GroupShape>().unwrap();
        let g_grandchild_ids: Vec<usize> = g_child
            .get_children()
            .iter()
            .map(|c| c.get_unique_id())
            .collect();
        assert_eq!(g_grandchild_ids, vec![s1_id, s2_id]);
    }

    #[test]
    fn subdividing_group_partitions_its_children() {
        let mut s1 = Sphere::new();
        let s1_id = s1.get_unique_id();
        println!("s1 id: {}", s1_id);
        s1.set_transformation(translation(-2., -2., 0.));

        let mut s2 = Sphere::new();
        let s2_id = s2.get_unique_id();
        println!("s2 id: {}", s2_id);
        s2.set_transformation(translation(-2., 2., 0.));

        let mut s3 = Sphere::new();
        let s3_id = s3.get_unique_id();
        println!("s3 id: {}", s3_id);
        s3.set_transformation(scaling(4., 4., 4.));

        let mut g = GroupShape::new();
        g.add_child(Box::new(s1));
        g.add_child(Box::new(s2));
        g.add_child(Box::new(s3));
        g.divide(1);

        let g_children = g.get_children();

        assert_eq!(g_children[0].get_unique_id(), s3_id);

        let subgroup = g_children[1].downcast_ref::<GroupShape>().unwrap();
        let ids: Vec<usize> = subgroup
            .get_children()
            .iter()
            .map(|c| c.get_unique_id())
            .collect();
        assert_eq!(ids, vec![s1_id, s2_id]);
    }

    #[test]
    fn subdividing_group_with_too_few_children() {
        let mut s1 = Sphere::new();
        let s1_id = s1.get_unique_id();
        println!("s1 id: {}", s1_id);
        s1.set_transformation(translation(-2., 0., 0.));

        let mut s2 = Sphere::new();
        let s2_id = s2.get_unique_id();
        println!("s2 id: {}", s2_id);
        s2.set_transformation(translation(2., 1., 0.));

        let mut s3 = Sphere::new();
        let s3_id = s3.get_unique_id();
        println!("s3 id: {}", s3_id);
        s3.set_transformation(translation(2., -1., 0.));

        let mut subgroup = GroupShape::new();
        let subgroup_id = subgroup.get_unique_id();
        println!("subgroup id: {}", subgroup_id);
        subgroup.add_child(Box::new(s1));
        subgroup.add_child(Box::new(s2));
        subgroup.add_child(Box::new(s3));

        let s4 = Sphere::new();
        let s4_id = s4.get_unique_id();
        println!("s4 id: {}", s4_id);

        let mut g = GroupShape::new();
        println!("g id: {}", g.get_unique_id());
        g.add_child(Box::new(subgroup));
        g.add_child(Box::new(s4));

        g.divide(3);

        let g_children = g.get_children();
        println!("{:?}", g_children[0]);
        assert_eq!(g_children[0].get_unique_id(), subgroup_id);
        assert_eq!(g_children[1].get_unique_id(), s4_id);

        let subgroup_children = g_children[0]
            .downcast_ref::<GroupShape>()
            .unwrap()
            .get_children();
        assert_eq!(subgroup_children[0].get_unique_id(), s1_id);

        let ids: Vec<usize> = subgroup_children[1]
            .downcast_ref::<GroupShape>()
            .unwrap()
            .get_children()
            .iter()
            .map(|c| c.get_unique_id())
            .collect();
        assert_eq!(ids, vec![s2_id, s3_id]);
    }

    #[test]
    fn divide_preserves_pushed_down_transformation() {
        let mut s1 = Sphere::new();
        s1.set_transformation(translation(-2., 0., 0.));

        let mut s2 = Sphere::new();
        s2.set_transformation(translation(2., -1., 0.));

        let mut s3 = Sphere::new();
        s3.set_transformation(translation(2., 1., 0.));

        let mut group = GroupShape::new();
        group.set_transformation(translation(1., 1., 0.));
        group.add_child(Box::new(s1));
        group.add_child(Box::new(s2));
        group.add_child(Box::new(s3));

        // we expect the transformation to be passed down to the children; this
        // should not change when the group is divided
        group.divide(2);

        assert_eq!(
            group.get_children()[0].transformation(),
            &translation(-1., 1., 0.),
            "s1 transformation should be preserved during division"
        );
        let subgroup = group.get_children()[1]
            .downcast_ref::<GroupShape>()
            .unwrap();
        assert_eq!(
            subgroup.get_children()[0].transformation(),
            &translation(3., 0., 0.),
            "s2 transformation should be preserved during division"
        );
        assert_eq!(
            subgroup.get_children()[1].transformation(),
            &translation(3., 2., 0.),
            "s3 transformation should be preserved during division"
        );
    }
}
