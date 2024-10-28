use crate::Fee;

pub type Amount = rust_decimal::Decimal;

pub trait AmountExt {
    fn sub_fee(self, fee: Fee) -> Self;
}

impl AmountExt for Amount {
    fn sub_fee(self, fee: Fee) -> Self {
        self * (Fee::ONE - fee)
    }
}
