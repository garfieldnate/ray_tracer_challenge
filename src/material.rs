use crate::color::Color;
use crate::constants::white;
use crate::pattern::pattern::Pattern;
use std::fmt::Debug;
use std::ptr;

type BoxedPattern = Box<dyn Pattern>;
// Represents the reflective properties of a surface
#[derive(PartialEq, Debug, Clone)]
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

	pub reflective: f32,

	pub transparency: f32,
	pub refractive_index: f32,

	pub pattern: Option<BoxedPattern>,
}

// Just check that the objects are the same
// TODO: delete after fixed in Rust: https://github.com/rust-lang/rust/issues/39128
impl PartialEq for BoxedPattern {
	fn eq(&self, other: &Self) -> bool {
		ptr::eq(self as *const _, other as *const _)
	}
}

pub fn default_material() -> Material {
	Material {
		color: white(),
		ambient: 0.1,
		diffuse: 0.9,
		specular: 0.9,
		shininess: 200.0,
		pattern: None,
		reflective: 0.0,
		transparency: 0.0,
		refractive_index: 1.0,
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn default_material_attributes() {
		let m = default_material();
		assert_eq!(m.color, white());
		assert_eq!(m.ambient, 0.1);
		assert_eq!(m.diffuse, 0.9);
		assert_eq!(m.specular, 0.9);
		assert_eq!(m.shininess, 200.0);
		assert_eq!(m.reflective, 0.0);
		assert_eq!(m.transparency, 0.0);
		assert_eq!(m.refractive_index, 1.0);
	}
}
