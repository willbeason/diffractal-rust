
use num_complex::Complex;

pub struct Polynomial1 {
    a0: Complex<f64>,
    a1: Complex<f64>,
}

impl Polynomial1 {
    pub(crate) fn new(a0: Complex<f64>, a1: Complex<f64>) -> Polynomial1 {
        Polynomial1{
            a0,
            a1,
        }
    }
    
    pub(crate) fn eval(&self, x: Complex<f64>) -> Complex<f64> {
        self.a0 + self.a1 * x
    }
}
