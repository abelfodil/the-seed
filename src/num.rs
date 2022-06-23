use num::Float;

pub trait Round {
    fn signed_ceil(&self) -> Self;
    fn signed_floor(&self) -> Self;
}

impl<T: Float> Round for T {
    fn signed_ceil(&self) -> Self {
        self.signum() * self.abs().ceil()
    }
    fn signed_floor(&self) -> Self {
        self.signum() * self.abs().floor()
    }
}
