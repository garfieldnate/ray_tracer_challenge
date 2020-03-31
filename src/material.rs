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

// Represents the reflective properties of a surface
#[derive(PartialEq, Debug, Clone, Builder)]
#[builder(build_fn(skip))]
#[builder(pattern = "owned")]
pub struct Material {
    // #[builder(default = white())]
    // pub color: Color,
    // light reflected from other objects in the environment [0,1]
    #[builder(default = "0.1")]
    pub ambient: f32,

    // light reflected from a matte surface; depends on angle between
    // light source and surface normal [0,1]
    #[builder(default = "0.9")]
    pub diffuse: f32,

    // the reflection of the light source itself (gives specular highlight);
    // depends on the angle between the reflection vector and the eye vector [0,1]
    #[builder(default = "0.9")]
    pub specular: f32,

    // higher values give smaller and tighter specular highlights [10,200] (no real upper bound)
    #[builder(default = "200.0")]
    pub shininess: f32,

    #[builder(default)]
    pub reflective: f32,

    #[builder(default)]
    pub transparency: f32,

    #[builder(default = "1.")]
    pub refractive_index: f32,

    #[builder(default = "self.default_pattern()")]
    pub pattern: BoxedPattern,
}

impl MaterialBuilder {
    pub fn build(self) -> Material {
        let message: &str = "Field with defaults should always be present";
        Material {
            ambient: self.ambient.expect(message),
            diffuse: self.diffuse.expect(message),
            specular: self.specular.expect(message),
            shininess: self.shininess.expect(message),
            reflective: self.reflective.expect(message),
            transparency: self.transparency.expect(message),
            refractive_index: self.refractive_index.expect(message),
            pattern: self.pattern.expect(message),
        }
    }

    fn default_pattern(&self) -> BoxedPattern {
        Box::new(Solid::default())
    }
}

impl Default for Material {
    fn default() -> Self {
        MaterialBuilder::default().build()
    }
}

impl Material {
    pub fn toBuilder(&self) -> MaterialBuilder {
        let builder = MaterialBuilder::default()
            .ambient(self.ambient)
            .diffuse(self.diffuse)
            .specular(self.specular)
            .shininess(self.shininess)
            .reflective(self.reflective)
            .transparency(self.transparency)
            .refractive_index(self.refractive_index)
            .pattern(self.pattern.clone());

        builder
    }
}
