use num_bigint::{BigInt, RandBigInt, Sign};
use std::ops::{Add, Div, Mul, Rem, Sub};

pub fn bn_0() -> BigInt {
    return BigInt::parse_bytes(b"0", 10).unwrap();
}
pub fn bn_1() -> BigInt {
    return BigInt::parse_bytes(b"1", 10).unwrap();
}
pub fn bn_2() -> BigInt {
    return BigInt::parse_bytes(b"2", 10).unwrap();
}

pub trait Order {
    fn order() -> Self;
    fn suborder() -> Self;
}

pub trait FiniteField:
    Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
    + PartialOrd
    + Sized
    + Clone
    + Eq
    + PartialEq
    + std::fmt::Debug
    + Order
{
    fn new(n: BigInt) -> Self;
    fn to_bn(&self) -> BigInt;

    // https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm#Polynomial_extended_Euclidean_algorithm
    fn inv(&self) -> Self {
        if self.clone() == Self::new(bn_0()) {
            return self.clone();
        }

        let mut t = Self::new(bn_0());
        let mut newt = Self::new(bn_1());
        let mut r = Self::order();
        let mut newr = self.clone();

        while newr != Self::new(bn_0()) {
            let quotient = Self::new(r.clone().to_bn() / newr.clone().to_bn());
            let (_r, _newr) = (newr.clone(), r.clone() - quotient.clone() * newr.clone());
            let (_t, _newt) = (newt.clone(), t.clone() - quotient.clone() * newt.clone());
            t = _t;
            newt = _newt;
            r = _r;
            newr = _newr;
        }

        if t < Self::new(bn_0()) {
            t = t + Self::order();
        }

        t
    }
}

pub trait Random {
    fn get_random(l: Self, r: Self) -> Self;
}

pub trait Encode {
    fn encode(&self) -> [u8; 32];
    fn to_array(&self) -> [u8; 32];
}

#[derive(Eq, PartialEq, Clone, Debug, PartialOrd)]
pub struct Field {
    pub v: BigInt,
}

impl Encode for Field {
    fn encode(&self) -> [u8; 32] {
        let mut to_bytes = [0u8; 32];
        let (s, v) = self.v.to_bytes_le();
        assert!(s == Sign::Plus);
        to_bytes[0..v.len()].copy_from_slice(v.as_slice());
        to_bytes
    }

    fn to_array(&self) -> [u8; 32] {
        let mut to_bytes = [0u8; 32];
        let (_, v) = self.v.to_bytes_be();

        to_bytes[32 - v.len()..32].copy_from_slice(&v[..]);
        to_bytes
    }
}

impl Add for Field {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Field {
            v: (self.v + other.v),
        } % Self::order()
    }
}

impl Sub for Field {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Field {
            v: (self.v - other.v),
        } % Self::order()
    }
}

impl Mul for Field {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Field {
            v: (self.v * other.v),
        } % Self::order()
    }
}

impl Div for Field {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        self * other.inv() % Self::order()
    }
}

impl Rem for Field {
    type Output = Self;

    fn rem(self, other: Self) -> Self::Output {
        Field {
            v: ((self.v % other.clone().v) + other.clone().v) % other.v,
        }
    }
}

impl FiniteField for Field {
    fn new(n: BigInt) -> Self {
        Self { v: n } % Self::order()
    }

    fn to_bn(&self) -> BigInt {
        self.clone().v
    }
}

impl Random for Field {
    // [l, r)
    fn get_random(l: Self, r: Self) -> Self {
        let mut rng = rand::thread_rng();

        Self {
            v: rng.gen_bigint_range(&l.v, &r.v),
        }
    }
}
