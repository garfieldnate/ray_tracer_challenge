use crate::color::Color;
use crate::tuple::Tuple;
use crate::world::World;

pub trait Light {
    //TODO: name is dumb
    fn intensity(&self) -> Color;
    fn position(&self) -> Tuple;
    // TODO: shouldn't be mut
    fn intensity_at(&self, point: Tuple, world: &World) -> f32;
}
