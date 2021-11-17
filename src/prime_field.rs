use num_bigint::{BigInt, Sign, ToBigInt};
use num_traits::{Zero, One};

mod add;
mod sub;
mod mul;
mod div;

lazy_static! {
    pub static ref BN_0: BigInt = BigInt::from(0);
    pub static ref BN_1: BigInt = BigInt::from(1);
    pub static ref BN_2: BigInt = BigInt::from(2);
}

pub trait Order {
    fn order() -> &'static BigInt;
    fn suborder() -> &'static BigInt;
}

pub fn modulus(a: &BigInt, m: &BigInt) -> BigInt {
    ((a % m) + m) % m
}

fn legendre_symbol(a: &BigInt, q: &BigInt) -> i32 {
    // returns 1 if has a square root modulo q
    let ls: BigInt = a.modpow(&((q - 1) >> 1), &q);
    if ls == q - 1 {
        -1
    } else {
        1
    }
}

#[derive(Eq, PartialEq, Clone, Debug, PartialOrd)]
pub enum Error {
    NotASqure
}

pub trait PrimeField:
    PartialOrd
    + Sized
    + Clone
    + Eq
    + PartialEq
    + sp_std::fmt::Debug
    + Order
{
    fn new(n: &BigInt) -> Self;
    fn to_bn(&self) -> &BigInt;

    fn sqrt(&self) -> Result<Self, Error> {
        let a = self.to_bn();
        let q = Self::order();
        let two: &BigInt = &BN_2;

        if legendre_symbol(&a, q) != 1 || a.is_zero() || q == two {
            return Err(Error::NotASqure);
        }

        if q % 4 == 3.to_bigint().unwrap() {
            let r = &a.modpow(&((q + 1) / 4), q);
            return Ok(Self::new(r));
        }

        let s: BigInt = q - 1;
        let zeros: u64 = s.trailing_zeros().unwrap();
        let e: BigInt = zeros.to_bigint().unwrap();
        let s: BigInt = s >> zeros;

        let mut n: BigInt = BN_2.clone();
        while legendre_symbol(&n, q) != -1 {
            n = &n + 1;
        }

        let mut y = a.modpow(&((&s + 1) >> 1), q);
        let mut b = a.modpow(&s, q);
        let mut g = n.modpow(&s, q);
        let mut r = e;

        loop {
            let mut t = b.clone();
            let mut m = BN_0.clone();
            while !t.is_one() {
                t = modulus(&(&t * &t), q);
                m += 1;
            }

            if m.is_zero() {
                return Ok(Self::new(&y));
            }

            t = g.modpow(&(BN_2.modpow(&(&r - &m - 1), q)), q);
            g = g.modpow(&(BN_2.modpow(&(&r - &m), q)), q);
            y = modulus(&(&y * &t), q);
            b = modulus(&(&b * &g), q);
            r = m;
        }
    }

    // https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm#Polynomial_extended_Euclidean_algorithm
    fn inv(&self) -> Self {
        if self.to_bn().is_zero() {
            return self.clone();
        }

        let mut t = BN_0.clone();
        let mut newt = BN_1.clone();
        let mut r = Self::order().clone();
        let mut newr = self.to_bn().clone();

        while !newr.is_zero() {
            let quotient = &r / &newr;
            let _newr = &r - &quotient * &newr;
            let _newt = &t - &quotient * &newt;
            t = newt;
            r = newr;
            newt = _newt;
            newr = _newr;
        }

        if t < *BN_0 {
            t = t + Self::order();
        }

        Self::new(&t)
    }
}

pub trait Encode {
    fn encode(&self) -> [u8; 32];
    fn decode(encode: &[u8]) -> Self;
    fn to_array(&self) -> [u8; 32];
}

#[derive(Eq, PartialEq, Clone, Debug, PartialOrd)]
pub struct Field {
    pub v: BigInt,
}

impl PrimeField for Field {
    fn new(n: &BigInt) -> Self {
        Self {
            v: n % Self::order(),
        }
    }

    fn to_bn(&self) -> &BigInt {
        &self.v
    }
}

impl Encode for Field {
    fn encode(&self) -> [u8; 32] {
        let mut to_bytes = [0u8; 32];
        let (s, v) = self.v.to_bytes_le();
        assert!(s == Sign::Plus);
        to_bytes[0..v.len()].copy_from_slice(v.as_slice());
        to_bytes
    }

    fn decode(encode: &[u8]) -> Self {
        Field {
            v: BigInt::from_bytes_le(Sign::Plus, encode),
        }
    }

    fn to_array(&self) -> [u8; 32] {
        let mut to_bytes = [0u8; 32];
        let (_, v) = self.v.to_bytes_be();

        to_bytes[32 - v.len()..32].copy_from_slice(&v[..]);
        to_bytes
    }
}

#[cfg(feature="std")]
use num_bigint::RandBigInt;

#[cfg(feature="std")]
pub trait Random {
    fn get_random(l: &BigInt, r: &BigInt) -> Self;
}


#[cfg(feature="std")]
impl Random for Field {
    // [l, r)
    fn get_random(l: &BigInt, r: &BigInt) -> Self {
        let mut rng = rand::thread_rng();

        Self {
            v: rng.gen_bigint_range(l, r),
        }
    }
}
