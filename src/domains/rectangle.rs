use rand::rngs::ThreadRng;
use rand::Rng;

// A Square input domain specifies a uniform-ish distribution in the unit square.
// partitions defines the number of subunits where the distribution is guaranteed
// to contain exactly the same number of samples.
pub struct Square {
    pub partitions: usize,
    inv_partitions: f64,
}

// new constructs a new Square distribution partitioned into nxn sub-squares.
fn new(n: usize) -> Square {
    Square {
        partitions: n,
        inv_partitions: 1.0 / n as f64,
    }
}

impl Square {
    // new_out allocates a new vector for use with the Square distribution.
    pub fn new_out(&self) -> Vec<(f64, f64)> {
        vec![(0.0, 0.0); self.partitions * self.partitions]
    }

    // generate fills in an allocated vector with uniformly-distributed points.
    pub fn generate(&self, mut rng: ThreadRng, out: &mut Vec<(f64, f64)>) -> () {
        let mut i: usize = 0;
        let mut px: f64 = 0.0;
        let mut py: f64;

        for _ in 0..self.partitions {
            py = 0.0;
            for _ in 0..self.partitions {
                let rx: f64 = rng.random::<f64>() * self.inv_partitions;
                let ry: f64 = rng.random::<f64>() * self.inv_partitions;
                out[i] = (px + rx, py + ry);

                i += 1;
                py += self.inv_partitions;
            }
            px += self.inv_partitions;
        }
    }
}
