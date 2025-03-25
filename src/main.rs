mod transforms;
use crate::transforms::transform::TransformRng;
use crate::transforms::{Affine2, Polynomial2, Stochastic};
use num_complex::Complex;

fn main() {
    let mut c = Complex::new(-0.5, 0.5);

    let p = Polynomial2 {
        a0: c,
        a1: Complex::new(0.0, 0.0),
        a2: Complex::new(1.0, 0.0),
    };

    for _ in 0..5 {
        println!("{}", c);
        c = p.transform(c);
    }

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

    let mut z = [0.0, 0.0];
    let s: Stochastic<[f64; 2], [f64; 2]> = Stochastic {
        transforms: vec![
            (0.01, Box::from(stem)),
            (0.85, Box::from(smaller)),
            (0.07, Box::from(left)),
            (0.07, Box::from(right)),
        ],
    };

    let mut rng = rand::rng();
    for _ in 0..100 {
        println!("({}, {})", z[0], z[1]);
        z = s.transform(z, &mut rng);
    }

    println!("Hello, world!");
}
