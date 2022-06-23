pub trait Round {
    fn signed_ceil(&self) -> Self;
    fn signed_floor(&self) -> Self;
}

impl Round for f64 {
    fn signed_ceil(&self) -> Self {
        self.signum() * self.abs().ceil()
    }
    fn signed_floor(&self) -> Self {
        self.signum() * self.abs().floor()
    }
}
