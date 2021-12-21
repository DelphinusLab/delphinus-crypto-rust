use super::*;
use sp_std::ops::Mul;
use num_bigint::{BigInt};
use num_traits::{Zero};
use std::vec::Vec;

impl<'a, 'b> Mul<&'b BigInt> for &'a BabyJubjubPoint {
    type Output = BabyJubjubPoint;

    fn mul(self, other: &BigInt) -> BabyJubjubPoint {
        let mut acc = BabyJubjubPoint::get_origin().clone();
        let mut k = other.clone();
        let mut vec = Vec::new();

        while !k.is_zero() {
            if k.is_odd() {
                let d: BigInt = &k % 16;
                k = &k - &d;
                vec.push(d);
            } else {
                vec.push(BigInt::zero())
            }
            k = k >> 1u32;
        }

        while let Some(d) = vec.pop() {
            acc = &acc + &acc;
            if d.is_odd() {
                acc = &acc + self._mul(&d);
            }
        }

        acc
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

