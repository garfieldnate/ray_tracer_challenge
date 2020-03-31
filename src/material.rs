use crate::pattern::pattern::Pattern;
use crate::pattern::solid::Solid;
use std::fmt::Debug;
use std::ptr;

type BoxedPattern = Box<dyn Pattern>;

// Just check that the objects are the same
// TODO: delete after fixed in Rust: https://github.com/rust-lang/rust/issues/39128
impl PartialEq for BoxedPattern {
    fn eq(&self, other: &Self) -> bool {
        ptr::eq(self as *const _, other as *const _)
    }
}

impl Default for BoxedPattern {
    fn default() -> Self {
        Box::new(Solid::default())
    }
}

// Represents the reflective properties of a surface
#[derive(PartialEq, Debug, Clone, TypedBuilder)]
pub struct Material {
    // #[builder(default = white())]
    // pub color: Color,
    // light reflected from other objects in the environment [0,1]
    #[builder(default = 0.1)]
    pub ambient: f32,

    // light reflected from a matte surface; depends on angle between
    // light source and surface normal [0,1]
    #[builder(default = 0.9)]
    pub diffuse: f32,

    // the reflection of the light source itself (gives specular highlight);
    // depends on the angle between the reflection vector and the eye vector [0,1]
    #[builder(default = 0.9)]
    pub specular: f32,

    // higher values give smaller and tighter specular highlights [10,200] (no real upper bound)
    #[builder(default = 200.0)]
    pub shininess: f32,

    #[builder(default)]
    pub reflective: f32,

    #[builder(default)]
    pub transparency: f32,

    #[builder(default = 1.)]
    pub refractive_index: f32,

    #[builder(default = "Box::new(Solid::default())")]
    pub pattern: BoxedPattern,
}

impl Default for Material {
    fn default() -> Self {
        Self::builder().build()
    }
}
