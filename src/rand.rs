use rand::Rng;

pub trait RandomValue {
    fn random(rng: &mut impl Rng) -> Self;
}
