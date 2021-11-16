use num_bigint::BigInt;

#[derive(Clone, Debug)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

pub trait Curve<T>: Sized {
    fn get_a() -> &'static T;
    fn get_d() -> &'static T;
    fn get_origin() -> &'static Point<T>;
    fn get_basepoint() -> &'static Point<T>;
    fn get_order() -> &'static BigInt;
    fn encode(&self) -> [u8; 32];
    fn decode(encode: &[u8]) -> Result<Self, String>;
}
