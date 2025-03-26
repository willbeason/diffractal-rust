mod domains;
mod transforms;

use crate::transforms::transform::TransformRng;
use crate::transforms::{barnsley_fern, Polynomial2};
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

    let mut z = [0.0, 0.0];
    let s = barnsley_fern();

    let mut rng = rand::rng();
    for _ in 0..100 {
        println!("({}, {})", z[0], z[1]);
        z = s.transform(z, &mut rng);
    }

    println!("Hello, world!");
}
