use super::*;
use sp_std::ops::Mul;
use num_bigint::{BigInt, Sign};
use num_traits::{Zero};
use std::vec::Vec;

impl<'a, 'b> Mul<&'b BigInt> for &'a BabyJubjubPoint {
    type Output = BabyJubjubPoint;

    fn mul(self, other: &BigInt) -> BabyJubjubPoint {
        let mut acc = BabyJubjubPoint::get_origin().clone();
        let mut k = other.clone();
        let mut vec = Vec::new();

        let mut pre = Vec::new();
        pre.push(self.clone());
        let double = self + self;
        let mut i = 1;
        while i < 8 {
            pre.push(&pre[i-1] + &double);
            i += 1;
        }

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
                let (_, ds) = d.to_u32_digits();
                acc = &acc + &pre[(ds[0] >> 1) as usize]
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

