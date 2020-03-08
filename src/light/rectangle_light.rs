use crate::color::Color;
use crate::light::light::Light;
use crate::tuple::Tuple;
use crate::world::World;
use derivative::Derivative;
use rand::distributions::Distribution;
use rand::{thread_rng, Rng};
// A point light: has no size and exists at single point.
#[derive(Derivative)]
#[derivative(Debug, PartialEq)]
// #[derivative(Clone, Copy, Debug, PartialEq)]
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
    // for random light sampling
    #[derivative(Debug = "ignore")]
    #[derivative(PartialEq = "ignore")]
    jitter_distribution: Box<dyn Distribution<f32>>,
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
        jitter_distribution: Box<dyn Distribution<f32>>,
    ) -> RectangleLight {
        RectangleLight {
            intensity,
            corner,
            u: full_u / u_steps as f32,
            v: full_v / v_steps as f32,
            u_steps,
            v_steps,
            cells: u_steps * v_steps,
            jitter_distribution,
            position: corner + (full_u / 2.) + (full_v / 2.),
        }
    }
    pub fn point_on_light(&self, u: i32, v: i32) -> Tuple {
        let rng = thread_rng();
        let jitter1 = rng.sample(self.jitter_distribution);
        let jitter2 = rng.sample(self.jitter_distribution);
        println!("Jittering u by {} and v by {}", jitter1, jitter2);
        self.corner + self.u * (u as f32 + jitter1) + self.v * (v as f32 + jitter2)
    }
}

impl Light for RectangleLight {
    fn position(&self) -> Tuple {
        self.position
    }
    fn intensity(&self) -> Color {
        self.intensity
    }
    fn intensity_at(&self, point: Tuple, world: &World) -> f32 {
        let mut total = 0.;
        for v in 0..self.v_steps {
            for u in 0..self.u_steps {
                let light_position = self.point_on_light(u, v);
                if !world.is_shadowed(light_position, point) {
                    total += 1.0;
                }
            }
        }

        return total / self.cells as f32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::white;
    use crate::test::utils::{ConstantDistribution, HardcodedDistribution};

    fn fake_jitter_data() -> Vec<f32> {
        vec![0.3, 0.7, 0.4, 0.5, 0.0, 0.6, 0.1, 0.9]
    }

    #[test]
    fn rectangle_light_construction() {
        let corner = point!(0, 0, 0);
        let u = vector!(2, 0, 0);
        let v = vector!(0, 0, 1);
        let light =
            RectangleLight::new(white(), corner, u, 4, v, 2, Box::new(ConstantDistribution));

        assert_eq!(light.u, vector!(0.5, 0, 0));
        assert_eq!(light.u_steps, 4);
        assert_eq!(light.v, vector!(0, 0, 0.5));
        assert_eq!(light.v_steps, 2);
        assert_eq!(light.cells, 8);
        assert_eq!(light.position, point!(1, 0, 0.5));
    }

    // TODO: delete
    #[test]
    fn finding_single_point_on_rectangle_light() {
        let corner = point!(0, 0, 0);
        let u_vec = vector!(2, 0, 0);
        let v_vec = vector!(0, 0, 1);
        let test_data = vec![
            ("1", 0, 0, point!(0.25, 0, 0.25)),
            ("2", 1, 0, point!(0.75, 0, 0.25)),
            ("3", 0, 1, point!(0.25, 0, 0.75)),
            ("4", 2, 0, point!(1.25, 0, 0.25)),
            ("5", 3, 1, point!(1.75, 0, 0.75)),
        ];
        for (name, u, v, expected) in test_data {
            let light = RectangleLight::new(
                white(),
                corner,
                u_vec,
                4,
                v_vec,
                2,
                Box::new(ConstantDistribution),
            );
            let p = light.point_on_light(u, v);
            assert_eq!(p, expected, "case: {:?}", name);
        }
    }

    #[test]
    fn intensity_at() {
        let test_data = vec![
            ("1", point!(0, 0, 2), 0.0),
            ("2", point!(1, -1, 2), 0.25),
            ("3", point!(1.5, 0, 2), 0.5),
            ("4", point!(1.25, 1.25, 3), 0.75),
            ("5", point!(0, 0, -2), 1.0),
        ];
        let w = World::default();
        let corner = point!(-0.5, -0.5, -5);
        let u_vec = vector!(1, 0, 0);
        let v_vec = vector!(0, 1, 0);
        for (name, p, expected) in test_data {
            let light = RectangleLight::new(
                white(),
                corner,
                u_vec,
                2,
                v_vec,
                2,
                Box::new(ConstantDistribution),
            );
            let intensity = light.intensity_at(p, &w);
            assert_eq!(intensity, expected, "case: {:?}", name);
        }
    }

    #[test]
    fn find_single_point_on_randomized_rectangle_light() {
        let corner = point!(0, 0, 0);
        let u_vec = vector!(2, 0, 0);
        let v_vec = vector!(0, 0, 1);
        let test_data = vec![
            ("1", 0, 0, point!(0.15, 0, 0.35)),
            ("2", 1, 0, point!(0.65, 0, 0.35)),
            ("3", 0, 1, point!(0.15, 0, 0.85)),
            ("4", 2, 0, point!(1.15, 0, 0.35)),
            ("5", 3, 1, point!(1.65, 0, 0.85)),
        ];
        for (name, u, v, expected) in test_data {
            let light = RectangleLight::new(
                white(),
                corner,
                u_vec,
                4,
                v_vec,
                2,
                Box::new(HardcodedDistribution::new(fake_jitter_data())),
            );
            let p = light.point_on_light(u, v);
            assert_eq!(p, expected, "case: {:?}", name);
        }
    }
}
