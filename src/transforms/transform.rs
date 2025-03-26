pub trait Transform<T, U> {
    fn transform(&self, z: T) -> U;
}

pub trait TransformRng<T, U> {
    fn transform(&self, z: T, rng: &mut impl rand::Rng) -> U;
}

pub struct IteratedFn<T> {
    // f is a function that can be called repeatedly
    f: Box<dyn Fn(T) -> T>,

    // stop is a condition that considers both the
    stop: Box<dyn Fn(i32, T) -> bool>,
}

pub struct Mandelbrot {}

impl Mandelbrot {
    fn paths() -> () {}
}
