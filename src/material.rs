use crate::color::build_color;
use crate::color::Color;

// Represents the reflective properties of a surface
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Material {
	pub color: Color,
	// light reflected from other objects in the environment [0,1]
	pub ambient: f32,
	// light reflected from a matte surface; depends on angle between
	// light source and surface normal [0,1]
	pub diffuse: f32,
	// the reflection of the light source itself (gives specular highlight);
	// depends on the angle between the reflection vector and the eye vector [0,1]
	pub specular: f32,
	// higher values give smaller and tighter specular highlights [10,200] (no real upper bound)
	pub shininess: f32,
}

pub fn default_material() -> Material {
	Material {
		color: color!(1, 1, 1),
		ambient: 0.1,
		diffuse: 0.9,
		specular: 0.9,
		shininess: 200.0,
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn default_material_attributes() {
		let m = default_material();
		assert_eq!(m.color, color!(1, 1, 1));
		assert_eq!(m.ambient, 0.1);
		assert_eq!(m.diffuse, 0.9);
		assert_eq!(m.specular, 0.9);
		assert_eq!(m.shininess, 200.0);
	}
}