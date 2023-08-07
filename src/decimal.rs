use rust_decimal::Decimal;

pub trait CloneAndRescale {
    fn clone_with_scale(&self, scale: u32) -> Self;
}

impl CloneAndRescale for Decimal {
    /// Clones given `Decimal` and sets it's scale to the given scale value.
    /// If the given scale is lower than the current scale,
    /// the `Decimal` will be rounded down, using the `MidpointAwayFromZero` strategy.
    fn clone_with_scale(&self, scale: u32) -> Self {
        let mut c = self.to_owned();
        c.rescale(scale);
        c
    }
}
