pub struct Halton {
    value: f32,
    inv_base: f32,
}

impl Halton {
    fn new(mut index: i32, base: i32) -> Halton {
        let inv_base = 1. / (base as f32);
        let mut fraction = 1.;
        let mut value = 0.;
        while index > 0 {
            fraction *= inv_base;
            value += fraction * (index % base) as f32;
            index /= base;
        }
        Halton { value, inv_base }
    }

    fn next(&mut self) {
        let r: f32 = 1. - self.value - 1e-7;
        if self.inv_base < r {
            self.value += self.inv_base;
        } else {
            let mut h: f32 = self.inv_base;
            let mut hh: f32;
            loop {
                hh = h;
                h *= self.inv_base;
                if h < r {
                    break;
                }
            }
            self.value += hh + h - 1.;
        }
    }

    fn get(&self) -> f32 {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor() {
        assert!(false);
    }
}
