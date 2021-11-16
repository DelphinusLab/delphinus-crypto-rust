use super::*;
use sp_std::ops::Mul;
use num_bigint::{BigInt};

impl<'a, 'b> Mul<&'b BigInt> for &'a BabyJubjubPoint {
    type Output = BabyJubjubPoint;

    fn mul(self, other: &BigInt) -> BabyJubjubPoint {
        self._mul(other)
    }
}

impl<'b> Mul<&'b BigInt> for BabyJubjubPoint {
    type Output = BabyJubjubPoint;

    fn mul(self, other: &BigInt) -> BabyJubjubPoint {
        &self * other
    }
}

impl<'a> Mul<BigInt> for &'a BabyJubjubPoint {
    type Output = BabyJubjubPoint;

    fn mul(self, other: BigInt) -> BabyJubjubPoint {
        self * &other
    }
}

impl Mul<BigInt> for BabyJubjubPoint {
    type Output = BabyJubjubPoint;

    fn mul(self, other: BigInt) -> BabyJubjubPoint {
        &self * &other
    }
}

