use num_complex::Complex64;

struct Mandelbrot {
    max_iter: usize,
    threshold: f64,
}

impl Default for Mandelbrot {
    fn default() -> Self {
        Mandelbrot{
            max_iter: 100,
            // 4.0 corresponds to escaping the circle of radius 2, guaranteeing divergence.
            threshold: 4.0,
        }
    }
}

impl Mandelbrot {
    pub fn new_path_out(&self) -> Vec<Complex64> {
        vec![Complex64::new(0.0, 0.0); self.max_iter]
    }
    
    // generate_path fills in out with the first max_iter positions.
    // Returns the number of iterations to pass threshold.
    pub fn generate_path(&self, c: Complex64, out: &mut Vec<Complex64>) -> Option<usize> {
        out[0] = c;
        let mut z = c;
        for i in 0..self.max_iter {
            if z.norm() > self.threshold {
                return Some(i);
            }
            z = z*z + c;
            out[i] = z;
        }
        
        None
    }
}
