use rand::Rng;

pub trait RandomValue {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Self;
}
