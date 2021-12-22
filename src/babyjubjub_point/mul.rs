use super::*;
use sp_std::ops::Mul;
use num_bigint::{BigInt};
use num_traits::{Zero};
use std::vec::Vec;

impl<'a, 'b> Mul<&'b BigInt> for &'a BabyJubjubPoint {
    type Output = BabyJubjubPoint;

    fn mul(self, other: &BigInt) -> BabyJubjubPoint {
        // ref: https://en.wikipedia.org/wiki/Elliptic_curve_point_multiplication#w-ary_non-adjacent_form_(wNAF)_method
        let mut acc = BabyJubjubPoint::get_origin().clone();
        let mut k = other.clone();
        let w = 1 << 4;

        // Compute the non-adjacent form of the multiplicand
        let mut naf = Vec::new();
        while !k.is_zero() {
            if k.is_odd() {
                let d: BigInt = &k % w;
                k = &k - &d;
                naf.push(d);
            } else {
                naf.push(BigInt::zero())
            }
            k = k >> 1u32;
        }

        // Pre compute {1,3,5,...,w-1}P
        let mut pre = Vec::new();
        pre.push(self.clone());
        let double = self + self;
        let mut i = 1;
        while i < (w >> 1) {
            pre.push(&pre[i-1] + &double);
            i += 1;
        }

        while let Some(d) = naf.pop() {
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

