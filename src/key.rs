use crate::curve::{Curve, Point};
pub use crate::prime_field::{BN_0, BN_1, BN_2, PrimeField, Random};
use num_bigint::BigInt;

#[derive(Debug)]
pub struct Sign<T> {
    pub r: Point<T>,
    pub s: T,
}

pub trait EDDSA<F: PrimeField + Random, C: Curve<F>> {
    fn secret_scalar(secret_key: &F) -> BigInt;
    fn pubkey_from_secretkey(secret_key: &F) -> Point<F>;
    fn verify(data: &[u8], signature: Sign<F>, public_key: C) -> bool;
    fn sign(data: &[u8], secret_key: F) -> Sign<F>;
    fn gen_secretkey() -> F {
        F::get_random(&BN_0, F::order())
    }
    fn hash(data: &[u8]) -> Vec<u8>;
}
