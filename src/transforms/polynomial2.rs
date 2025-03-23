use num_complex::Complex;

pub struct Polynomial2 {
    pub(crate) a0: Complex<f64>,
    pub(crate) a1: Complex<f64>,
    pub(crate) a2: Complex<f64>,
}

impl Polynomial2 {
    pub(crate) fn transform(&self, x: Complex<f64>) -> Complex<f64> {
        self.a0 + x * (self.a1 + x * self.a2)
    }
}
