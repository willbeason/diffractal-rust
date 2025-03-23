use crate::transforms::transform::{Transform, TransformRng};
use rand::Rng;

pub struct Stochastic<T, U> {
    pub transforms: Vec<(f64, Box<dyn Transform<T, U>>)>,
}

impl<T, U> TransformRng<T, U> for Stochastic<T, U> {
    fn transform(&self, z: T, rng: &mut impl Rng) -> U {
        let mut r = rng.random::<f64>();

        for t in self.transforms.iter() {
            r -= t.0;
            if r < 0.0 {
                return t.1.transform(z);
            }
        }

        panic!("Should never reach here");
    }
}
