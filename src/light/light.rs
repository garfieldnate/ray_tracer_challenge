use crate::tuple::Tuple;
use crate::world::World;

pub trait Light {
    fn intensity_at(&self, point: Tuple, world: &World) -> f32;
}
