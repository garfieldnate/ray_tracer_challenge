use crate::light::PointLight;
use crate::ray::Intersection;
use crate::ray::Ray;
use crate::ray::Sphere;
use crate::tuple::Tuple;
use std::cmp::Ordering::Equal;

pub struct World {
    objects: Vec<Sphere>,
    light: Option<PointLight>,
}

pub fn build_world() -> World {
    World {
        objects: vec![],
        light: Option::None,
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
        // intersections.sort_by_key(|i| i.distance);

        // for o in &self.objects {
        //     let xs = o.intersect(r);
        //     intersections.append(xs),
        // }
        intersections
    }
}

pub struct PrecomputedValues<'a> {
    distance: f32,
    object: &'a Sphere,
    point: Tuple,
    eye_vector: Tuple,
    surface_normal: Tuple,
}
pub fn precompute_values<'a>(r: Ray, i: Intersection<'a>) -> PrecomputedValues<'a> {
    let point = r.position(i.distance);
    PrecomputedValues {
        // copy the intersection's properties, for convenience​
        distance: i.distance,
        object: i.object,
        // precompute some useful values
        point: point,
        eye_vector: -r.direction,
        surface_normal: i.object.normal_at(point),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::build_color;
    use crate::light::build_point_light;
    use crate::material::default_material;
    use crate::matrix::identity_4x4;
    use crate::ray::build_intersection;
    use crate::ray::build_ray;
    use crate::ray::build_sphere;
    use crate::ray::default_sphere;
    use crate::transformations::scaling;
    use crate::tuple::build_tuple;

    fn default_world() -> World {
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
        let comps = precompute_values(r, i);
        assert_eq!(comps.distance, i.distance);
        assert_eq!(comps.point, point!(0, 0, -1));
        assert_eq!(comps.eye_vector, vector!(0, 0, -1));
        assert_eq!(comps.surface_normal, vector!(0, 0, -1));
    }
}
