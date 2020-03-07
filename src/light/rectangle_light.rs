use crate::color::Color;
use crate::light::light::Light;
use crate::tuple::Tuple;
use crate::world::World;
// A point light: has no size and exists at single point.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RectangleLight {
    // TODO: in real life, don't lights come in non-uniform colors and intensities?
    pub intensity: Color,
    //  the position of one corner of the light source; u and v start here
    pub corner: Tuple,
    // u and v are edges of one rectangular cell of the light; u_step and v_step specify
    // the number of cells in each direction.
    pub u: Tuple,
    pub u_steps: i32,
    pub v: Tuple,
    pub v_steps: i32,
    pub cells: i32,
    // TODO: remove
    // the very center of the rectangle
    pub position: Tuple,
}

impl RectangleLight {
    pub fn new(
        intensity: Color,
        corner: Tuple,
        full_u: Tuple,
        u_steps: i32,
        full_v: Tuple,
        v_steps: i32,
    ) -> RectangleLight {
        RectangleLight {
            intensity,
            corner,
            u: full_u / u_steps as f32,
            v: full_v / v_steps as f32,
            u_steps,
            v_steps,
            cells: u_steps * v_steps,
            position: corner + (full_u / 2.) + (full_v / 2.),
        }
    }
}

impl Light for RectangleLight {
    fn intensity_at(&self, point: Tuple, world: &World) -> f32 {
        1.
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::white;
    #[test]
    fn rectangle_light_construction() {
        let corner = point!(0, 0, 0);
        let v1 = vector!(2, 0, 0);
        let v2 = vector!(0, 0, 1);
        let light = RectangleLight::new(white(), corner, v1, 4, v2, 2);

        assert_eq!(light.u, vector!(0.5, 0, 0));
        assert_eq!(light.u_steps, 4);
        assert_eq!(light.v, vector!(0, 0, 0.5));
        assert_eq!(light.v_steps, 2);
        assert_eq!(light.cells, 8);
        assert_eq!(light.position, point!(1, 0, 0.5));
    }
}
