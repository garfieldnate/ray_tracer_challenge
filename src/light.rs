use crate::color::build_color;
use crate::color::Color;
use crate::material::Material;
use crate::ray::Ray;
use crate::tuple::Tuple;

// A point light: has no size and exists at single point.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PointLight {
	pub position: Tuple,
	pub intensity: Color,
}

pub fn build_point_light(position: Tuple, intensity: Color) -> PointLight {
	PointLight {
		position,
		intensity,
	}
}

// Given scene parameters, determine the lighting at a given point assuming
// the Phong model of lighting: the result color is the sum of colors produced
// by modeling ambient, diffuse and specular lighting.
pub fn phong_lighting(
	material: Material,
	light: PointLight,
	point: Tuple,
	eye_vector: Tuple,
	surface_normal: Tuple,
	in_shadow: bool,
) -> Color {
	// mix the surface color with the light's color
	let effective_color = material.color * light.intensity;

	let ambient = effective_color * material.ambient;

	if in_shadow {
		return ambient;
	}

	let direction_point_to_light = (light.position - point).norm();
	let light_normal_cosine = direction_point_to_light.dot(surface_normal);

	let diffuse: Color;
	let specular: Color;
	// negative cosine indicates the light is behind the surface
	if light_normal_cosine < 0.0 {
		diffuse = build_color(0.0, 0.0, 0.0); // black
		specular = build_color(0.0, 0.0, 0.0); // black
	} else {
		diffuse = effective_color * material.diffuse * light_normal_cosine;
		let surface_reflection = Ray::reflect(-direction_point_to_light, surface_normal);
		let reflection_eye_cosine = surface_reflection.dot(eye_vector);
		// negative cosine indicates the light reflecting away from the eye
		if reflection_eye_cosine <= 0.0 {
			specular = build_color(0.0, 0.0, 0.0);
		} else {
			// Assumes microfacet normals are approximately Gaussian
			// https://en.wikipedia.org/wiki/Specular_highlight#Phong_distribution
			let factor = reflection_eye_cosine.powf(material.shininess);
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
	use crate::material::default_material;
	use crate::tuple::build_tuple;
	use std::f32::consts::FRAC_1_SQRT_2;

	#[test]
	fn point_light_has_position_and_intensity() {
		let position = point!(0, 0, 0);
		let intensity = build_color(1.0, 1.0, 1.0);
		let light = build_point_light(position, intensity);
		assert_eq!(light.position, position);
		assert_eq!(light.intensity, intensity);
	}

	#[test]
	fn lighting_eye_between_light_and_surface() {
		let m = default_material();
		let position = point!(0, 0, 0);
		let eye_vector = vector!(0, 0, -1);
		let surface_normal = vector!(0, 0, -1);
		let light = build_point_light(point!(0, 0, -10), build_color(1.0, 1.0, 1.0));
		let result = phong_lighting(m, light, position, eye_vector, surface_normal, false);
		assert_eq!(result, build_color(1.9, 1.9, 1.9));
	}

	#[test]
	fn light_eye_between_light_and_surface_eye_offset_45_degrees() {
		let m = default_material();
		let position = point!(0, 0, 0);
		let eye_vector = vector!(0, FRAC_1_SQRT_2, FRAC_1_SQRT_2);
		let surface_normal = vector!(0, 0, -1);
		let light = build_point_light(point!(0, 0, -10), build_color(1.0, 1.0, 1.0));
		let result = phong_lighting(m, light, position, eye_vector, surface_normal, false);
		assert_eq!(result, build_color(1.0, 1.0, 1.0));
	}

	#[test]
	fn light_eye_between_light_and_surface_light_offset_45_degrees() {
		let m = default_material();
		let position = point!(0, 0, 0);
		let eye_vector = vector!(0, 0, -1);
		let surface_normal = vector!(0, 0, -1);
		let light = build_point_light(point!(0, 10, -10), build_color(1.0, 1.0, 1.0));
		let result = phong_lighting(m, light, position, eye_vector, surface_normal, false);
		let expected_intensity = 0.1 + 0.9 * FRAC_1_SQRT_2;
		assert_eq!(
			result,
			build_color(expected_intensity, expected_intensity, expected_intensity)
		);
	}

	#[test]
	fn light_eye_in_path_of_reflection_vector() {
		let m = default_material();
		let position = point!(0, 0, 0);
		let eye_vector = vector!(0, -FRAC_1_SQRT_2, -FRAC_1_SQRT_2);
		let surface_normal = vector!(0, 0, -1);
		let light = build_point_light(point!(0, 10, -10), build_color(1.0, 1.0, 1.0));
		let result = phong_lighting(m, light, position, eye_vector, surface_normal, false);
		// 0.1 + 0.9 * FRAC_1_SQRT_2 + 0.9, but with some floating point errors
		assert_abs_diff_eq!(result, build_color(1.6363853, 1.6363853, 1.6363853));
	}

	#[test]
	fn light_behind_surface() {
		let m = default_material();
		let position = point!(0, 0, 0);
		let eye_vector = vector!(0, 0, -1);
		let surface_normal = vector!(0, 0, -1);
		let light = build_point_light(point!(0, 0, 10), build_color(1.0, 1.0, 1.0));
		let result = phong_lighting(m, light, position, eye_vector, surface_normal, false);
		assert_abs_diff_eq!(result, build_color(0.1, 0.1, 0.1));
	}

	#[test]
	fn lighting_shadowed_surface() {
		let material = default_material();
		let position = point!(0, 0, 0);
		let eye_vector = vector!(0, 0, -1);
		let surface_normal = vector!(0, 0, -1);
		let light = build_point_light(point!(0, 0, -10), build_color(1.0, 1.0, 1.0));
		let result = phong_lighting(material, light, position, eye_vector, surface_normal, true);
		assert_eq!(result, build_color(0.1, 0.1, 0.1));
	}
}
