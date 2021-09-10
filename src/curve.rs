use num_bigint::BigInt;
use std::ops::{Add, Mul};

#[derive(Clone, Debug)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

pub trait Curve<T>: Add<Output = Self> + Mul<T> + Sized {
    fn get_a() -> T;
    fn get_d() -> T;
    fn get_origin() -> Point<T>;
    fn get_basepoint() -> Point<T>;
    fn get_order() -> T;
    fn encode(&self) -> [u8; 32];
    fn decode(encode: &[u8]) -> Self;
    fn mul_scalar(self, k: BigInt) -> Self;
}
