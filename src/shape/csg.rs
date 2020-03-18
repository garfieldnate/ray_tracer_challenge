use crate::bounding_box::BoundingBox;
use crate::intersection::Intersection;
use crate::ray::Ray;
use crate::shape::base_shape::BaseShape;
use crate::shape::shape::Shape;
use crate::tuple::Tuple;
use std::cmp::Ordering::Equal;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSGOperator {
    Union(),
    Intersection(),
    Difference(),
}

#[derive(Debug)]
pub struct CSG {
    base: BaseShape,
    op: CSGOperator,
    s1: Box<dyn Shape>,
    s2: Box<dyn Shape>,
}

impl CSG {
    pub fn new(op: CSGOperator, s1: Box<dyn Shape>, s2: Box<dyn Shape>) -> Self {
        CSG {
            base: BaseShape::new(),
            op,
            s1,
            s2,
        }
    }

    fn filter_intersections<'a>(
        &self,
        intersections: &Vec<Intersection<'a>>,
    ) -> Vec<Intersection<'a>> {
        // begin outside of both children
        let mut inside_s1 = false;
        let mut inside_s2 = false;
        let mut filtered: Vec<Intersection> = vec![];

        for i in intersections {
            let hit_s1 = self.s1.includes(i.object);
            if CSG::intersection_allowed(self.op, hit_s1, inside_s1, inside_s2) {
                filtered.push(*i);
            }
            if hit_s1 {
                inside_s1 = !inside_s1;
            } else {
                inside_s2 = !inside_s2;
            }
        }
        filtered
    }

    // hit_s1: true if intersection is with a CSG's s1, false if with the s2
    // inside_s1: true if intersection is inside CSG's s1, false otherwise
    // inside_s2: true if intersection is inside CSG's s2, false otherwise
    fn intersection_allowed(
        op: CSGOperator,
        hit_s1: bool,
        inside_s1: bool,
        inside_s2: bool,
    ) -> bool {
        match op {
            CSGOperator::Union() => (hit_s1 && !inside_s2) || (!hit_s1 && !inside_s1),
            CSGOperator::Intersection() => (hit_s1 && inside_s2) || (!hit_s1 && inside_s1),
            CSGOperator::Difference() => (hit_s1 && !inside_s2) || (!hit_s1 && inside_s1),
        }
    }
}

impl Shape for CSG {
    // TODO: pass transformation down tree similar to in group
    fn get_base(&self) -> &BaseShape {
        &self.base
    }

    fn get_base_mut(&mut self) -> &mut BaseShape {
        &mut self.base
    }

    fn local_intersect(&self, object_ray: Ray) -> Vec<Intersection> {
        let mut intersections = vec![];

        let b = self.bounding_box();
        if !b.intersects(object_ray) {
            return intersections;
        }

        for i in self.s1.as_ref().intersect(object_ray) {
            intersections.push(i);
        }
        for i in self.s2.as_ref().intersect(object_ray) {
            intersections.push(i);
        }
        intersections.sort_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap_or(Equal));

        self.filter_intersections(&intersections)
    }

    fn local_norm_at(&self, _object_point: Tuple, _hit: &Intersection) -> Tuple {
        // intersection objects will always point to s1 and s2, so no need to implement this
        unimplemented!()
    }

    fn includes(&self, other: &dyn Shape) -> bool {
        if self.get_unique_id() == other.get_unique_id() {
            true
        } else {
            self.s1.includes(other) || self.s2.includes(other)
        }
    }

    fn bounding_box(&self) -> BoundingBox {
        let mut b = BoundingBox::empty();

        b.add_bounding_box(self.s1.parent_space_bounding_box());
        b.add_bounding_box(self.s2.parent_space_bounding_box());

        b
    }

    fn divide(&mut self, threshold: usize) {
        self.s1.divide(threshold);
        self.s2.divide(threshold);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::material::Material;
    use crate::shape::csg::CSGOperator::Difference;
    use crate::shape::csg::CSGOperator::Union;
    use crate::shape::cube::Cube;
    use crate::shape::group::GroupShape;
    use crate::shape::sphere::Sphere;
    use crate::shape::test_shape::TestShape;
    use crate::transformations::translation;

    #[test]
    fn csg_construction() {
        // TODO: possibly fragile test
        let s1 = Box::new(Sphere::new());
        let s1_address = s1.as_ref() as *const dyn Shape;
        let s2 = Box::new(Cube::new());
        let s2_address = s2.as_ref() as *const dyn Shape;

        let c = CSG::new(CSGOperator::Union(), s1, s2);
        assert_eq!(c.op, CSGOperator::Union());

        assert_eq!(c.s1.as_ref() as *const _, s1_address);
        assert_eq!(c.s2.as_ref() as *const _, s2_address);
    }

    #[test]
    fn csg_operation_rule_evaluation() {
        let test_data = vec![
            ("union1", Union(), true, true, true, false),
            ("union2", Union(), true, true, false, true),
            ("union3", Union(), true, false, true, false),
            ("union4", Union(), true, false, false, true),
            ("union5", Union(), false, true, true, false),
            ("union6", Union(), false, true, false, false),
            ("union7", Union(), false, false, true, true),
            ("union8", Union(), false, false, false, true),
            (
                "union9",
                CSGOperator::Intersection(),
                true,
                true,
                true,
                true,
            ),
            (
                "intersection1",
                CSGOperator::Intersection(),
                true,
                true,
                false,
                false,
            ),
            (
                "intersection2",
                CSGOperator::Intersection(),
                true,
                false,
                true,
                true,
            ),
            (
                "intersection3",
                CSGOperator::Intersection(),
                true,
                false,
                false,
                false,
            ),
            (
                "intersection4",
                CSGOperator::Intersection(),
                false,
                true,
                true,
                true,
            ),
            (
                "intersection5",
                CSGOperator::Intersection(),
                false,
                true,
                false,
                true,
            ),
            (
                "intersection6",
                CSGOperator::Intersection(),
                false,
                false,
                true,
                false,
            ),
            (
                "intersection7",
                CSGOperator::Intersection(),
                false,
                false,
                false,
                false,
            ),
            ("", Difference(), true, true, true, false),
            ("", Difference(), true, true, false, true),
            ("", Difference(), true, false, true, false),
            ("", Difference(), true, false, false, true),
            ("", Difference(), false, true, true, true),
            ("", Difference(), false, true, false, true),
            ("", Difference(), false, false, true, false),
            ("", Difference(), false, false, false, false),
        ];
        for (name, op, hit_s1, inside_s1, inside_s2, expected) in test_data {
            assert_eq!(
                expected,
                CSG::intersection_allowed(op, hit_s1, inside_s1, inside_s2),
                "Case {}",
                name
            );
        }
    }

    #[test]
    fn filter_intersections() {
        let test_data = vec![
            ("union", Union(), 0, 3),
            ("intersection", CSGOperator::Intersection(), 1, 2),
            ("difference", Difference(), 0, 1),
        ];
        for (name, op, x0, x1) in test_data {
            let s1 = Box::new(Sphere::new());
            let s2 = Box::new(Cube::new());
            let c = CSG::new(op, s1, s2);
            let xs = vec![
                Intersection::new(1., c.s1.as_ref()),
                Intersection::new(2., c.s2.as_ref()),
                Intersection::new(3., c.s1.as_ref()),
                Intersection::new(4., c.s2.as_ref()),
            ];
            let filtered = c.filter_intersections(&xs);
            println!("{:?}", filtered);
            assert_eq!(2, filtered.len(), "case: {}", name);
            assert_eq!(&filtered[0], &xs[x0], "case: {}", name);
            assert_eq!(&filtered[1], &xs[x1], "case: {}", name);
        }
    }

    #[test]
    fn ray_misses_csg_object() {
        let c = CSG::new(Union(), Box::new(Sphere::new()), Box::new(Cube::new()));
        let r = Ray::new(point!(0, 2, -5), vector!(0, 0, 1));
        let xs = c.local_intersect(r);
        assert!(xs.is_empty());
    }

    #[test]
    fn ray_hits_csg_object() {
        let s1 = Sphere::new();
        let s2 = Sphere::build(translation(0., 0., 0.5), Material::default());
        let c = CSG::new(Union(), Box::new(s1), Box::new(s2));
        let r = Ray::new(point!(0, 0, -5), vector!(0, 0, 1));
        let xs = c.local_intersect(r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], Intersection::new(4.0, c.s1.as_ref()));
        assert_eq!(xs[1], Intersection::new(6.5, c.s2.as_ref()));
    }

    #[test]
    fn csg_bounding_box_contains_children() {
        let left = Sphere::new();
        let right = {
            let mut s = Sphere::new();
            s.set_transformation(translation(2., 3., 4.));
            s
        };
        let shape = CSG::new(CSGOperator::Difference(), Box::new(left), Box::new(right));
        let b = shape.bounding_box();
        assert_eq!(b.min, point!(-1, -1, -1));
        assert_eq!(b.max, point!(3, 4, 5));
    }

    #[test]
    fn ray_intersection_doesnt_test_children_if_bounding_box_is_missed() {
        let left = TestShape::new();
        let right = TestShape::new();
        let shape = CSG::new(CSGOperator::Difference(), Box::new(left), Box::new(right));
        let r = Ray::new(point!(0, 0, -5), vector!(0, 1, 0));
        shape.intersect(r);

        let test_shape1 = shape.s1.downcast_ref::<TestShape>().unwrap();
        println!("{:?}", test_shape1.saved_ray.borrow());
        assert!(test_shape1.saved_ray.borrow().is_none());

        let test_shape2 = shape.s2.downcast_ref::<TestShape>().unwrap();
        println!("{:?}", test_shape2.saved_ray.borrow());
        assert!(test_shape2.saved_ray.borrow().is_none());
    }

    #[test]
    fn ray_intersection_tests_children_if_bounding_box_is_hit() {
        let left = TestShape::new();
        let right = TestShape::new();
        let shape = CSG::new(CSGOperator::Difference(), Box::new(left), Box::new(right));
        let r = Ray::new(point!(0, 0, -5), vector!(0, 0, 1));
        shape.intersect(r);

        let test_shape1 = shape.s1.downcast_ref::<TestShape>().unwrap();
        println!("{:?}", test_shape1.saved_ray.borrow());
        assert!(test_shape1.saved_ray.borrow().is_some());

        let test_shape2 = shape.s2.downcast_ref::<TestShape>().unwrap();
        println!("{:?}", test_shape2.saved_ray.borrow());
        assert!(test_shape2.saved_ray.borrow().is_some());
    }

    #[test]
    fn subdividing_csg_subdivides_its_children() {
        let s1 = TestShape::new();
        let s2 = TestShape::new();
        let left = GroupShape::with_children(vec![Box::new(s1), Box::new(s2)]);

        let s3 = TestShape::new();
        let s4 = TestShape::new();
        let right = GroupShape::with_children(vec![Box::new(s3), Box::new(s4)]);

        let mut shape = CSG::new(CSGOperator::Difference(), Box::new(left), Box::new(right));
        shape.divide(1);
        assert!(false);
    }
}
