use crate::intersection::Intersection;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::shape::base_shape::BaseShape;
use crate::shape::group::GroupShape;
use crate::shape::shape::Shape;
use crate::tuple::Tuple;
use std::f32;
use std::rc::Rc;

// Base shape is a double-napped cone with tips meeting at the origin and extending vertically along the y axis.

#[derive(Clone, Debug, PartialEq)]
pub struct Cone {
    base: BaseShape,
    pub minimum_y: f32,
    pub maximum_y: f32,
    pub closed: bool,
}

impl Cone {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(transform: Matrix, material: Material) -> Self {
        let mut s = Cone::new();
        s.set_transformation(transform);
        s.set_material(material);
        s
    }
}

impl Default for Cone {
    fn default() -> Self {
        Cone {
            base: BaseShape::new(),
            minimum_y: f32::NEG_INFINITY,
            maximum_y: f32::INFINITY,
            closed: false,
        }
    }
}

impl Shape for Cone {
    fn local_intersect(&self, object_ray: Ray) -> Vec<Intersection> {
        let mut intersections: Vec<Intersection> = Vec::with_capacity(2);
        self.intersect_sides(&object_ray, &mut intersections);
        self.intersect_caps(&object_ray, &mut intersections);
        intersections
    }

    // norms at the corners are the norms of one of the adjacent sides
    fn local_norm_at(&self, object_point: Tuple) -> Tuple {
        let dist_square = object_point.x.powi(2) + object_point.z.powi(2);
        // TODO: why does this work? Shouldn't it be < y?
        if dist_square < 1.0 {
            if object_point.y >= self.maximum_y - CLOSE_TO_ZERO {
                return vector!(0, 1, 0);
            } else if object_point.y <= self.minimum_y + CLOSE_TO_ZERO {
                return vector!(0, -1, 0);
            }
        }
        let y = (object_point.x.powi(2) + object_point.z.powi(2)).sqrt();
        let y = if object_point.y > 0.0 { -y } else { y };
        vector!(object_point.x, y, object_point.z)
    }

    // forward these to BaseShape (TODO: need delegation RFC to be accepted!)
    fn transformation(&self) -> &Matrix {
        &self.base.transformation()
    }
    fn set_transformation(&mut self, t: Matrix) {
        self.base.set_transformation(t);
    }
    fn material(&self) -> &Material {
        self.base.material()
    }
    fn set_material(&mut self, m: Material) {
        self.base.set_material(m);
    }
    fn get_parent(&self) -> &Option<Rc<GroupShape>> {
        self.base.get_parent()
    }
    fn set_parent(&mut self, shape: Option<Rc<GroupShape>>) {
        self.base.set_parent(shape)
    }
    fn casts_shadow(&self) -> bool {
        self.base.casts_shadow()
    }
    fn set_casts_shadow(&mut self, casts_shadow: bool) {
        self.base.set_casts_shadow(casts_shadow)
    }
    fn transformation_inverse(&self) -> &Matrix {
        self.base.transformation_inverse()
    }
    fn transformation_inverse_transpose(&self) -> &Matrix {
        self.base.transformation_inverse_transpose()
    }
}

const CLOSE_TO_ZERO: f32 = 0.000001;
impl Cone {
    fn intersect_sides<'a>(&'a self, object_ray: &Ray, intersections: &mut Vec<Intersection<'a>>) {
        // calculating 2a here instead of a to save a multiplication later
        let two_a = 2.0
            * (object_ray.direction.x.powi(2) - object_ray.direction.y.powi(2)
                + object_ray.direction.z.powi(2));
        let b = 2.0
            * (object_ray.origin.x * object_ray.direction.x
                - object_ray.origin.y * object_ray.direction.y
                + object_ray.origin.z * object_ray.direction.z);

        // TODO: turn this into shared constant somewhere?
        if two_a.abs() < CLOSE_TO_ZERO {
            if b.abs() < CLOSE_TO_ZERO {
                // ray misses both halves of cone
                return;
            }
            // there's only one intersection point
            let c = Cone::calc_c(&object_ray);
            let distance = -c / (2.0 * b);
            intersections.push(Intersection::new(distance, self));
            return;
        }

        let c = Cone::calc_c(&object_ray);
        let discriminant = b.powi(2) - 2.0 * two_a * c;

        if discriminant < 0.0 {
            //ray does not intersect Cone
            return;
        }

        // Jingle all the way!
        let discriminant_sqrt = discriminant.sqrt();
        let distance1 = (-b - discriminant_sqrt) / two_a;
        let distance2 = (-b + discriminant_sqrt) / two_a;

        let (distance1, distance2) = if distance1 > distance2 {
            (distance2, distance1)
        } else {
            (distance1, distance2)
        };

        let y1 = object_ray.origin.y + distance1 * object_ray.direction.y;
        if self.minimum_y < y1 && y1 < self.maximum_y {
            intersections.push(Intersection::new(distance1, self));
        }
        let y2 = object_ray.origin.y + distance2 * object_ray.direction.y;
        if self.minimum_y < y2 && y2 < self.maximum_y {
            intersections.push(Intersection::new(distance2, self));
        }
    }

    // this is the c from the quadratic equation used in the side intersection check
    // it's just here for code reuse
    fn calc_c(object_ray: &Ray) -> f32 {
        object_ray.origin.x.powi(2) - object_ray.origin.y.powi(2) + object_ray.origin.z.powi(2)
    }

    // check if the intersection at distance is within the radius from the y axis
    fn check_cap(radius: f32, ray: &Ray, distance: f32) -> bool {
        let x = ray.origin.x + distance * ray.direction.x;
        let z = ray.origin.z + distance * ray.direction.z;
        // TODO: the book didn't use an epsilon. Maybe switching to f64 everywhere would fix this?
        (x.powi(2) + z.powi(2)) <= radius + CLOSE_TO_ZERO
    }

    // add intersections with the end caps of the Cone to intersections
    fn intersect_caps<'a>(&'a self, object_ray: &Ray, intersections: &mut Vec<Intersection<'a>>) {
        // don't bother checking for intersection if the Cone isn't close
        // TODO: book says we should also have `|| object_ray.direction.y <= CLOSE_TO_ZERO ` here.
        // That makes no sense, though, right? A vertical ray can intersect both caps. Maybe report as
        // error?
        if !self.closed {
            return;
        }

        // TODO: cache ray direction inverses
        let distance = (self.minimum_y - object_ray.origin.y) / object_ray.direction.y;
        if Cone::check_cap(self.minimum_y.abs(), &object_ray, distance) {
            intersections.push(Intersection::new(distance, self));
        }
        let distance = (self.maximum_y - object_ray.origin.y) / object_ray.direction.y;
        if Cone::check_cap(self.maximum_y.abs(), &object_ray, distance) {
            intersections.push(Intersection::new(distance, self));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::AbsDiffEq;
    use std::f32::consts::SQRT_2;

    // #[test]
    // fn ray_misses_cone() {
    //     TODO
    // }

    #[test]
    fn ray_intersects_cone_sides() {
        let c = Cone::new();
        let test_data = vec![
            ("1", point!(0, 0, -5), vector!(0, 0, 1), 5., 5.),
            (
                "2",
                // Note: book specifies exactly 5 for z, but our floating point numbers are just a bit different.
                point!(0, 0, -4.999999),
                vector!(1, 1, 1),
                8.660253,
                8.660253,
            ),
            (
                "3",
                point!(1, 1, -5),
                vector!(-0.5, -1, 1),
                4.5500546,
                49.449955,
            ),
        ];
        for (name, origin, direction, distance1, distance2) in test_data {
            let r = Ray::new(origin, direction.norm());
            let xs = c.local_intersect(r);
            assert_eq!(xs.len(), 2, "{}: should find 2 intersections", name);
            debug_assert!(
                xs[0]
                    .distance
                    .abs_diff_eq(&distance1, f32::default_epsilon()),
                "{}: distance to first intersection (expected {}, got {})",
                name,
                distance1,
                xs[0].distance
            );
            debug_assert!(
                xs[1]
                    .distance
                    .abs_diff_eq(&distance2, f32::default_epsilon()),
                "{}: distance to second intersection (expected {}, got {})",
                name,
                distance2,
                xs[1].distance
            );
        }
    }

    #[test]
    fn intersect_cone_with_ray_parallel_to_one_half() {
        let s = Cone::new();
        let r = Ray::new(point!(0, 0, -1), vector!(0, 1, 1).norm());
        let intersections = s.local_intersect(r);
        assert_eq!(intersections.len(), 1);
        assert_abs_diff_eq!(intersections[0].distance, 0.35355338);
    }

    #[test]
    fn ray_intersects_caps_of_closed_cone() {
        let c = {
            let mut c = Cone::new();
            c.minimum_y = -0.5;
            c.maximum_y = 0.5;
            c.closed = true;
            c
        };
        let test_data = vec![
            ("1", point!(0, 0, -5), vector!(0, 1, 0), 0),
            ("2", point!(0, 0, -0.25), vector!(0, 1, 1), 2),
            ("3", point!(0, 0, -0.25), vector!(0, 1, 0), 4),
        ];
        for (name, origin, direction, expected_num_intersections) in test_data {
            let r = Ray::new(origin, direction.norm());
            let xs = c.local_intersect(r);
            assert_eq!(xs.len(), expected_num_intersections, "{}", name);
        }
    }

    #[test]
    fn cone_normal_vector() {
        let c = Cone::new();
        let test_data = vec![
            ("1", point!(0, 0, 0), vector!(0, 0, 0)),
            ("2", point!(1, 1, 1), vector!(1, -SQRT_2, 1)),
            ("3", point!(-1, -1, 0), vector!(-1, 1, 0)),
        ];
        for (name, point, expected_normal) in test_data {
            let normal = c.local_norm_at(point);
            assert_eq!(normal, expected_normal, "{}", name);
        }
    }
}
