use rand::Rng;

pub trait RandomValue<T> {
    fn random(rng: &mut impl Rng) -> T;
}
