use crate::transforms::{Affine2, Stochastic};

pub fn barnsley_fern() -> Stochastic<[f64; 2], [f64; 2]> {
    let stem = Affine2 {
        a: [0.0, 0.0, 0.0, 0.16],
        offset: [0.0, 0.0],
    };
    let smaller = Affine2 {
        a: [0.85, 0.04, -0.04, 0.85],
        offset: [0.0, 1.6],
    };
    let left = Affine2 {
        a: [0.2, -0.26, 0.23, 0.22],
        offset: [1.6, 0.07],
    };
    let right = Affine2 {
        a: [-0.15, 0.28, 0.26, 0.24],
        offset: [0.0, 0.44],
    };

    Stochastic {
        transforms: vec![
            (0.01, Box::from(stem)),
            (0.85, Box::from(smaller)),
            (0.07, Box::from(left)),
            (0.07, Box::from(right)),
        ],
    }
}
