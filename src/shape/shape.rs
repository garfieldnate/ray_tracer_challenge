use crate::bounding_box::BoundingBox;
use crate::intersection::Intersection;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::shape::base_shape::BaseShape;
use crate::tuple::Tuple;
use downcast_rs::Downcast;
use std::fmt::Debug;
use std::hash::Hash;
use std::hash::Hasher;

// TODO: update to DowncastSync later when parallelizing
pub trait Shape: Debug + Downcast {
    // tthe BaseShape that the wrapping instance is delegating to
    fn get_base(&self) -> &BaseShape;
    fn get_base_mut(&mut self) -> &mut BaseShape;

    fn local_intersect(&self, object_ray: Ray) -> Vec<Intersection>;
    fn local_norm_at(&self, object_point: Tuple, hit: &Intersection) -> Tuple;

    fn bounding_box(&self) -> BoundingBox;

    // The rest of these should not be overridden by Shape implementers

    fn get_unique_id(&self) -> usize {
        self.get_base().get_unique_id()
    }
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
        // A normal was computed in object space and must be returned in world space.
        // This is a different problem from converting a *point* from object to world space.
        // We are not concerned with the location of the normal on the surface of the object
        // but rather the direction that it points in. We have to consider several types of
        // transformations given to the parent object:
        //
        // The first is rotation. This needs to be applied to the vector as-is: if you stick
        // a toothpick in a peach to represent the normal on the peach's surface, then you will
        // see that rotating the peach rotates the toothpick in exactly the same manner.
        //
        // The next is uniform scaling. This does not affect a normal at all; as the peach
        // grows and shrinks, the tooth pick will point in the same direction.
        //
        // Next is non-uniform scaling. This is more complex. If you poke several toothpicks
        // close together on one side of the peach so that they are almost parallel and then
        // you squish the peach downwards, scaling y by 1/2 so that it becomes one of those
        // weird donut peaches, the toothpicks will change direction a little bit so that they
        // are pointing more away from each other. From the top, the peach will look the same,
        // but from the side, you can see that the normals change more slowly on the top and
        // more quickly on the sides. Scaling the y axis by 1/2 actually doubles the
        // y-component of all of the normals on the peach. This means scaling the normals by
        // the inverse of the matrix that scaled the object; the inverse of a scaling matrix
        // is just the same matrix but with each of the scaling components inverted.
        //
        // Last is shearing. I will use a different image here. Imagine a cardboard box with
        // no lid or bottom, sitting on its side on a table. If you look through the center of
        // the box, it will appear as a square. If you squish the box a little, it will become
        // a parallelogram. This is a shearing operation which displaces part of the box
        // farther along the x-axis as we measure it over increasing y-values. You'll notice that
        // as we increase this x-y shear, the normals on the sides instead have their y-components
        // compared to the y axis. which is a shear operation in the x-y axis that increases the size of the box
        . As you squish the box more

        // * rotation: needs to be applied to the normal as-is
        // * translation: these do not apply to vectors; we'll come back to this
        // * non-uniform scaling: the inverse of the scaling needs to be applied to the normal
        // * shearing: the inverse transpose of the scaling needs to be applied to the normal
        //
        // The reason non-uniform scaling needs to be inverted is that it changes the slopes
        // of surfaces, which is measured by the normal. For example, imagine squishing a
        // sphere by multiplying just the y-axis by 1/2 so that it looks like an M&M. The x
        // and z axes remain unaffected, so from above it looks like a circle, but from the
        // sides it looks like an oval. This oval has normals that change quickly on the side
        // and more slowly on the top and bottom. The 1/2 scaling in the y axis doubles the
        // y-component of all of the normals on the sphere/M&M. This means taking the inverse
        // of the scaling matrix, because the inverse of a scaling matrix is created by taking
        // the reciprocal of each element in the matrix.
        //
        // So we need the reciprocal of the scaling component of a matrix and we need the
        // rotation component as-is. There is a trick to this:
        // means taking the inverse of th
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

    fn normal_at(&self, world_point: &Tuple, hit: &Intersection) -> Tuple {
        // When computing the normal vector, all shapes need to first convert the point to
        // object space, multiplying it by the inverse of the shape’s transformation matrix.
        let object_point = self.world_to_object_point(&world_point);
        let object_normal = self.local_norm_at(object_point, hit);
        self.normal_to_world(&object_normal)
    }

    // should only be overridden by GroupShape and CSG
    fn includes(&self, other: &dyn Shape) -> bool {
        // TODO: how to unify this with the PartialEq implementation
        self.get_unique_id() == other.get_unique_id()
    }

    fn parent_space_bounding_box(&self) -> BoundingBox {
        self.bounding_box().transform(self.transformation())
    }

    // no-op for shapes that do not combine other shapes
    fn divide(&mut self, _threshold: usize) {}
}

// TODO: add 'sync' keyword when parallelizing
impl_downcast!(Shape);

impl PartialEq for dyn Shape {
    fn eq(&self, other: &Self) -> bool {
        self.get_unique_id() == other.get_unique_id()
    }
}

impl Eq for dyn Shape {}

impl Hash for dyn Shape {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.get_unique_id().hash(hasher);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shape::group::GroupShape;
    use crate::shape::sphere::Sphere;
    use crate::shape::test_shape::TestShape;
    use crate::test::utils::dummy_intersection;
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
        let n = s.normal_at(&point!(0, 1.70711, -0.70711), &dummy_intersection(&s));
        assert_abs_diff_eq!(n, vector!(0.0, 0.600_000_1, -0.799_999_95));
    }

    #[test]
    fn normal_on_transformed_shape() {
        let mut s = TestShape::new();
        s.set_transformation(scaling(1.0, 0.5, 1.0) * rotation_z(PI / 5.0));
        let n = s.normal_at(
            &point!(0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
            &dummy_intersection(&s),
        );
        assert_abs_diff_eq!(n, vector!(-0.083_526_63, 0.932_529_6, -0.351_300_3));
    }

    #[test]
    fn normal_is_normalized_vector() {
        let s = TestShape::new();
        let n = s.normal_at(&point!(1, 5, 10), &dummy_intersection(&s));
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
        let g2 = g1.get_children()[0].as_ref();
        let s = g2.downcast_ref::<GroupShape>().unwrap().get_children()[0].as_ref();

        let n = s.normal_to_world(&object_normal);
        assert_abs_diff_eq!(n, vector!(0.28571427, 0.42857143, -0.85714287));
    }

    #[test]
    fn querying_shape_boundary_box_in_parent_space() {
        let mut s = Sphere::new();
        s.set_transformation(translation(1., -3., 5.) * scaling(0.5, 2., 4.));
        let b = s.parent_space_bounding_box();
        assert_eq!(b.min, point!(0.5, -5, 1));
        assert_eq!(b.max, point!(1.5, -1, 9.));
    }
}
