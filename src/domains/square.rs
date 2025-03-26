use num_complex::Complex64;
use rand::Rng;
use rand::rngs::ThreadRng;

// A Square input domain specifies a uniform-ish distribution in the unit square.
// partitions defines the number of subunits where the distribution is guaranteed
// to contain exactly the same number of samples.
pub struct Square {
    pub partitions: usize,

    delta: (f64, f64),

    min: (f64, f64),
}

// new constructs a new Square distribution partitioned into nxn sub-squares.
fn new(n: usize, min: (f64, f64), max: (f64, f64)) -> Square {
    Square {
        partitions: n,
        min,
        delta: ((max.0 - min.0) / n as f64, (max.1 - min.1) / n as f64),
    }
}

impl Square {
    // new_out allocates a new vector for use with the Square distribution.
    pub fn new_out_real(&self) -> Vec<(f64, f64)> {
        vec![(0.0, 0.0); self.partitions * self.partitions]
    }

    // generate fills in an allocated vector with uniformly-distributed points.
    pub fn generate_real(&self, mut rng: ThreadRng, out: &mut Vec<(f64, f64)>) -> () {
        let mut i: usize = 0;
        let mut p: (f64, f64) = self.min;

        for _ in 0..self.partitions {
            p.1 = self.min.0;
            for _ in 0..self.partitions {
                let rx: f64 = rng.random::<f64>() * self.delta.0;
                let ry: f64 = rng.random::<f64>() * self.delta.1;
                out[i] = (p.0 + rx, p.1 + ry);

                i += 1;
                p.1 += self.delta.1;
            }
            p.0 += self.delta.0;
        }
    }

    pub fn new_out_complex(&self) -> Vec<Complex64> {
        vec![Complex64::new(0.0, 0.0); self.partitions * self.partitions]
    }

    // generate fills in an allocated vector with uniformly-distributed points.
    pub fn generate_complex(&self, mut rng: ThreadRng, out: &mut Vec<Complex64>) -> () {
        let mut i: usize = 0;
        let mut p: Complex64 = Complex64::new(self.min.0, self.min.1);

        for _ in 0..self.partitions {
            p.im = self.min.1;
            for _ in 0..self.partitions {
                let rx: f64 = rng.random::<f64>() * self.delta.0;
                let ry: f64 = rng.random::<f64>() * self.delta.1;
                out[i] = p + Complex64::new(rx, ry);

                i += 1;
                p.im += self.delta.1;
            }
            p.re += self.delta.0;
        }
    }
}
