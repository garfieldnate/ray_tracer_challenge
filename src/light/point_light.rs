use crate::color::Color;
use crate::light::light::Light;
use crate::tuple::Tuple;
use crate::world::World;
// A point light: has no size and exists at single point.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PointLight {
    pub position: Tuple,
    pub intensity: Color,
}

impl PointLight {
    pub fn new(position: Tuple, intensity: Color) -> PointLight {
        PointLight {
            position,
            intensity,
        }
    }
}

impl Light for PointLight {
    fn intensity_at(&self, point: Tuple, world: &World) -> f32 {
        if world.is_shadowed(self.position, point) {
            0.
        } else {
            1.
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::white;

    #[test]
    fn point_light_has_position_and_intensity() {
        let position = point!(0, 0, 0);
        let intensity = white();
        let light = PointLight::new(position, intensity);
        assert_eq!(light.position, position);
        assert_eq!(light.intensity, intensity);
    }
}
