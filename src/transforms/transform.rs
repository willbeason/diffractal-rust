pub trait Transform<T,U> {
    fn transform(&self, z: T) -> U;
}

pub trait TransformRng<T,U> {
    fn transform(&self, z: T, rng: &mut impl rand::Rng) -> U;
}
