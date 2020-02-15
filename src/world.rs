use crate::color::build_color;
use crate::color::Color;
use crate::light::build_point_light;
use crate::light::phong_lighting;
use crate::light::PointLight;
use crate::material::default_material;
use crate::matrix::identity_4x4;
use crate::ray::build_ray;
use crate::ray::build_sphere;
use crate::ray::Intersection;
use crate::ray::Ray;
use crate::ray::Sphere;
use crate::transformations::scaling;
use crate::tuple::{build_tuple, Tuple};
use std::cmp::Ordering::Equal;
use std::f32;

// TODO: book said no light by default, but that seems weird. We always have a light, otherwise we can't see anything! Plus using Option complicates/makes dangerous everything.
pub struct World {
    pub objects: Vec<Sphere>,
    pub light: Option<PointLight>,
}

pub fn build_world() -> World {
    World {
        objects: vec![],
        light: Option::None,
    }
}

pub fn default_world() -> World {
    let mut m = default_material();
    m.color = build_color(0.8, 1.0, 0.6);
    m.diffuse = 0.7;
    m.specular = 0.2;
    let s1 = build_sphere(identity_4x4(), m);
    let s2 = build_sphere(scaling(0.5, 0.5, 0.5), default_material());
    World {
        objects: vec![s1, s2],
        light: Some(build_point_light(
            point!(-10.0, 10.0, -10.0),
            build_color(1.0, 1.0, 1.0),
        )),
    }
}

impl World {
    pub fn intersect(&self, r: Ray) -> Vec<Intersection> {
        let mut intersections: Vec<Intersection> = (&self.objects)
            .iter()
            .map(|o| o.intersect(r))
            .flatten()
            .collect();
        intersections.sort_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap_or(Equal));
        intersections
    }

    pub fn shade_hit(&self, comps: PrecomputedValues) -> Color {
        phong_lighting(
            comps.object.material,
            self.light.unwrap(),
            comps.over_point,
            comps.eye_vector,
            comps.surface_normal,
            self.is_shadowed(comps.over_point),
        )
    }

    pub fn color_at(&self, r: Ray) -> Color {
        let intersections = self.intersect(r);
        if intersections.is_empty() {
            build_color(0.0, 0.0, 0.0)
        } else {
            match Intersection::hit(&intersections) {
                Some(hit) => {
                    let comps = precompute_values(r, hit);
                    self.shade_hit(comps)
                }
                None => build_color(0.0, 0.0, 0.0),
            }
        }
    }

    pub fn is_shadowed(&self, point: Tuple) -> bool {
        // create a ray from a point to the light
        // if there's an intersection between the light and the point, then the point is in shadow
        let light_to_point_vector = self.light.unwrap().position - point;
        let distance = light_to_point_vector.magnitude();
        let direction = light_to_point_vector.norm();

        let r = build_ray(point, direction);
        let intersections = self.intersect(r);

        let hit = Intersection::hit(&intersections);
        match hit {
            Some(i) => i.distance < distance,
            None => false,
        }
    }
}

pub struct PrecomputedValues<'a> {
    distance: f32,
    object: &'a Sphere,
    point: Tuple,
    eye_vector: Tuple,
    surface_normal: Tuple,
    inside: bool,
    // a point a tiny distance above surface to allow correct shadow calcluations with inexact floating point arithmetic
    over_point: Tuple,
}

pub fn precompute_values<'a>(r: Ray, i: &Intersection<'a>) -> PrecomputedValues<'a> {
    let point = r.position(i.distance);
    let mut surface_normal = i.object.normal_at(point);
    let eye_vector = -r.direction;

    let inside;
    if surface_normal.dot(eye_vector) < 0.0 {
        // surface and eye are pointed in opposite directions, so the hit must be inside
        inside = true;
        surface_normal = -surface_normal;
    } else {
        inside = false;
    }

    let over_point = point + surface_normal * f32::EPSILON;
    // println!("point: {:?}, over_point:{:?}", point, over_point);

    PrecomputedValues {
        // copy the intersection's properties, for convenience​
        distance: i.distance,
        object: i.object,
        // precompute some useful values
        point,
        eye_vector,
        surface_normal,
        inside,
        over_point,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::build_color;
    use crate::light::build_point_light;
    use crate::ray::build_intersection;
    use crate::ray::build_ray;
    use crate::ray::default_sphere;
    use crate::transformations::translation;
    use crate::tuple::build_tuple;

    #[test]
    fn create_blank_world() {
        let w = build_world();
        assert!(w.objects.is_empty());
        assert!(w.light.is_none());
    }

    #[test]
    fn intersect_world_with_ray() {
        let w = default_world();
        let r = build_ray(point!(0, 0, -5), vector!(0, 0, 1));
        let xs = w.intersect(r);
        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].distance, 4.0);
        assert_eq!(xs[1].distance, 4.5);
        assert_eq!(xs[2].distance, 5.5);
        assert_eq!(xs[3].distance, 6.0);
    }

    #[test]
    fn precompute_intersection_state() {
        let r = build_ray(point!(0, 0, -5), vector!(0, 0, 1));
        let shape = default_sphere();
        let i = build_intersection(4.0, &shape);
        let comps = precompute_values(r, &i);
        assert_eq!(comps.distance, i.distance);
        assert_eq!(comps.point, point!(0, 0, -1));
        assert_eq!(comps.eye_vector, vector!(0, 0, -1));
        assert_eq!(comps.surface_normal, vector!(0, 0, -1));
    }

    #[test]
    fn precompute_hit_occurs_outside() {
        let r = build_ray(point!(0, 0, -5), vector!(0, 0, 1));
        let shape = default_sphere();
        let i = build_intersection(4.0, &shape);
        let comps = precompute_values(r, &i);
        assert!(!comps.inside);
    }

    #[test]
    fn precompute_hit_occurs_inside() {
        let r = build_ray(point!(0, 0, 0), vector!(0, 0, 1));
        let shape = default_sphere();
        let i = build_intersection(1.0, &shape);
        let comps = precompute_values(r, &i);
        assert_eq!(comps.point, point!(0, 0, 1));
        assert_eq!(comps.eye_vector, vector!(0, 0, -1));
        assert_eq!(comps.inside, true);
        assert_eq!(
            comps.surface_normal,
            vector!(0, 0, -1),
            "Surface normal should be inverted because hit is inside shape"
        );
    }

    #[test]
    fn shade_intersection() {
        let w = default_world();
        let r = build_ray(point!(0, 0, -5), vector!(0, 0, 1));
        let shape = &w.objects[0];
        let i = build_intersection(4.0, shape);
        let comps = precompute_values(r, &i);
        let c = w.shade_hit(comps);
        assert_abs_diff_eq!(c, build_color(0.38066125, 0.4758265, 0.28549594))
    }

    #[test]
    fn shade_intersection_from_inside() {
        let mut w = default_world();
        w.light = Some(build_point_light(
            point!(0, 0.25, 0),
            build_color(1.0, 1.0, 1.0),
        ));
        let r = build_ray(point!(0, 0, 0), vector!(0, 0, 1));
        let shape = &w.objects[1];
        let i = build_intersection(0.5, shape);
        let comps = precompute_values(r, &i);
        let c = w.shade_hit(comps);
        assert_abs_diff_eq!(c, build_color(0.9049845, 0.9049845, 0.9049845))
    }

    #[test]
    fn color_when_ray_misses() {
        let w = default_world();
        let r = build_ray(point!(0, 0, -5), vector!(0, 1, 0));
        let c = w.color_at(r);
        assert_eq!(c, build_color(0.0, 0.0, 0.0));
    }

    #[test]
    fn color_when_ray_hits() {
        let w = default_world();
        let r = build_ray(point!(0, 0, -5), vector!(0, 0, 1));
        let c = w.color_at(r);
        assert_abs_diff_eq!(c, build_color(0.38066125, 0.4758265, 0.28549594))
    }

    #[test]
    fn color_when_intersection_behind_ray() {
        let mut w = default_world();
        // TODO: can't take w.objects[x] and mutate it...
        // outer
        w.objects[0].material.ambient = 1.0;
        // inner
        w.objects[1].material.ambient = 1.0;
        let r = build_ray(point!(0, 0, 0.75), vector!(0, 0, -1));
        let c = w.color_at(r);
        assert_eq!(c, w.objects[1].material.color);
    }

    #[test]
    fn no_shadow_when_nothing_is_colinear_with_point_and_light() {
        let w = default_world();
        let p = point!(0, 10, 0);
        assert_eq!(w.is_shadowed(p), false);
    }

    #[test]
    fn no_shadow_when_object_is_between_point_and_light() {
        let w = default_world();
        let p = point!(10, -10, 10);
        assert_eq!(w.is_shadowed(p), true);
    }

    #[test]
    fn no_shadow_when_object_is_behind_light() {
        let w = default_world();
        let p = point!(-20, 20, -20);
        assert_eq!(w.is_shadowed(p), false);
    }

    #[test]
    fn no_shadow_when_object_is_behind_point() {
        let w = default_world();
        let p = point!(-2, 2, -2);
        assert_eq!(w.is_shadowed(p), false);
    }

    #[test]
    fn hit_should_offset_point_for_shadow_calculations() {
        let r = build_ray(point!(0, 0, -5), vector!(0, 0, 1));
        let shape = build_sphere(translation(0.0, 0.0, 1.0), default_material());
        let intersection = build_intersection(5.0, &shape);
        let comps = precompute_values(r, &intersection);
        // println!("{:?}", comps.point);
        // println!("{:?}", comps.over_point);
        assert!(comps.over_point.z < -f32::EPSILON / 2.0);
        assert!(comps.over_point.z > -f32::EPSILON * 2.0);
        assert!(comps.point.z > comps.over_point.z);
    }

    #[test]
    fn shade_hit_for_intersection_in_shadow() {
        let mut w = build_world();
        w.light = Some(build_point_light(
            point!(0, 0, -10),
            build_color(1.0, 1.0, 1.0),
        ));
        let s1 = default_sphere();
        let s2 = build_sphere(translation(0.0, 0.0, 10.0), default_material());
        w.objects.push(s1);
        w.objects.push(s2);
        let r = build_ray(point!(0, 0, 5), vector!(0, 0, 1));
        let i = build_intersection(4.0, &w.objects[1]);
        let comps = precompute_values(r, &i);
        let c = w.shade_hit(comps);
        assert_eq!(c, build_color(0.1, 0.1, 0.1));
    }
}
