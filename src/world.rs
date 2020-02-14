use crate::color::build_color;
use crate::color::Color;
use crate::light::build_point_light;
use crate::light::phong_lighting;
use crate::light::PointLight;
use crate::material::default_material;
use crate::matrix::identity_4x4;
use crate::ray::build_sphere;
use crate::ray::Intersection;
use crate::ray::Ray;
use crate::ray::Sphere;
use crate::transformations::scaling;
use crate::tuple::{build_tuple, Tuple};
use std::cmp::Ordering::Equal;

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
            comps.point,
            comps.eye_vector,
            comps.surface_normal,
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
}

pub struct PrecomputedValues<'a> {
    distance: f32,
    object: &'a Sphere,
    point: Tuple,
    eye_vector: Tuple,
    surface_normal: Tuple,
    inside: bool,
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
    PrecomputedValues {
        // copy the intersection's properties, for convenience​
        distance: i.distance,
        object: i.object,
        // precompute some useful values
        point,
        eye_vector,
        surface_normal,
        inside,
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
}
