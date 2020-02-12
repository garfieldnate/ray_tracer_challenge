use crate::color::build_color;
use crate::color::Color;
use crate::material::Material;
use crate::ray::Ray;
use crate::tuple::Tuple;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Light {
    position: Tuple,
    intensity: Color,
}

pub fn point_light(position: Tuple, intensity: Color) -> Light {
    Light {
        position,
        intensity,
    }
}

pub fn lighting(
    material: Material,
    light: Light,
    point: Tuple,
    eyev: Tuple,
    normalv: Tuple,
) -> Color {
    // combine the surface color with the light's color/intensity
    let effective_color = material.color * light.intensity;
    //  find the direction to the light source
    let lightv = (light.position - point).norm();
    // compute the ambient contribution
    let ambient = effective_color * material.ambient;

    // light_dot_normal represents the cosine of the angle between the
    // light vector and the normal vector. A negative number means the
    // light is on the other side of the surface.
    let light_dot_normal = lightv.dot(normalv);

    let diffuse: Color;
    let specular: Color;
    if light_dot_normal < 0.0 {
        diffuse = build_color(0.0, 0.0, 0.0); // black
        specular = build_color(0.0, 0.0, 0.0); // black
    } else {
        // compute the diffuse contribution
        diffuse = effective_color * material.diffuse * light_dot_normal;
        // reflect_dot_eye represents the cosine of the angle between the
        // reflection vector and the eye vector. A negative number means the
        // light reflects away from the eye.
        let reflectv = Ray::reflect(-lightv, normalv);
        let reflect_dot_eye = reflectv.dot(eyev);
        if reflect_dot_eye <= 0.0 {
            specular = build_color(0.0, 0.0, 0.0);
        } else {
            // compute the specular contribution
            let factor = reflect_dot_eye.powf(material.shininess);
            specular = light.intensity * material.specular * factor;
        }
    }
    // Add the three contributions together to get the final shading
    ambient + diffuse + specular
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::build_color;
    use crate::material::build_material;
    use crate::tuple::build_tuple;
    use std::f32::consts::FRAC_1_SQRT_2;

    #[test]
    fn point_light_has_position_and_intensity() {
        let position = point!(0, 0, 0);
        let intensity = build_color(1.0, 1.0, 1.0);
        let light = point_light(position, intensity);
        assert_eq!(light.position, position);
        assert_eq!(light.intensity, intensity);
    }

    #[test]
    fn lighting_eye_between_light_and_surface() {
        let m = build_material();
        let position = point!(0, 0, 0);
        let eyev = vector!(0, 0, -1);
        let normalv = vector!(0, 0, -1);
        let light = point_light(point!(0, 0, -10), build_color(1.0, 1.0, 1.0));
        let result = lighting(m, light, position, eyev, normalv);
        assert_eq!(result, build_color(1.9, 1.9, 1.9));
    }

    #[test]
    fn light_eye_between_light_and_surface_eye_offset_45_degrees() {
        let m = build_material();
        let position = point!(0, 0, 0);
        let eyev = vector!(0, FRAC_1_SQRT_2, FRAC_1_SQRT_2);
        let normalv = vector!(0, 0, -1);
        let light = point_light(point!(0, 0, -10), build_color(1.0, 1.0, 1.0));
        let result = lighting(m, light, position, eyev, normalv);
        assert_eq!(result, build_color(1.0, 1.0, 1.0));
    }

    #[test]
    fn light_eye_between_light_and_surface_light_offset_45_degrees() {
        let m = build_material();
        let position = point!(0, 0, 0);
        let eyev = vector!(0, 0, -1);
        let normalv = vector!(0, 0, -1);
        let light = point_light(point!(0, 10, -10), build_color(1.0, 1.0, 1.0));
        let result = lighting(m, light, position, eyev, normalv);
        let expected_intensity = 0.1 + 0.9 * FRAC_1_SQRT_2;
        assert_eq!(
            result,
            build_color(expected_intensity, expected_intensity, expected_intensity)
        );
    }

    #[test]
    fn light_eye_in_path_of_reflection_vector() {
        let m = build_material();
        let position = point!(0, 0, 0);
        let eyev = vector!(0, -FRAC_1_SQRT_2, -FRAC_1_SQRT_2);
        let normalv = vector!(0, 0, -1);
        let light = point_light(point!(0, 10, -10), build_color(1.0, 1.0, 1.0));
        let result = lighting(m, light, position, eyev, normalv);
        // 0.1 + 0.9 * FRAC_1_SQRT_2 + 0.9, but with some floating point errors
        assert_abs_diff_eq!(result, build_color(1.6363853, 1.6363853, 1.6363853));
    }

    #[test]
    fn light_behind_surface() {
        let m = build_material();
        let position = point!(0, 0, 0);
        let eyev = vector!(0, 0, -1);
        let normalv = vector!(0, 0, -1);
        let light = point_light(point!(0, 0, 10), build_color(1.0, 1.0, 1.0));
        let result = lighting(m, light, position, eyev, normalv);
        assert_abs_diff_eq!(result, build_color(0.1, 0.1, 0.1));
    }
}
