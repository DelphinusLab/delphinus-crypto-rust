pub use crate::babyjubjub::BabyJubjubField;
pub use crate::curve::{Curve, Point};
pub use crate::prime_field::*;
pub use crate::key::{Sign, EDDSA};
use num_bigint::{BigInt};
use num_traits::{Zero};
use num_integer::{Integer};

pub type BabyJubjubPoint = Point<BabyJubjubField>;

mod add;
mod mul;

impl PartialEq for BabyJubjubPoint {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

lazy_static! {
    static ref A: BabyJubjubField = BabyJubjubField {
        v: BigInt::from(168700)
    };

    static ref D: BabyJubjubField = BabyJubjubField {
        v: BigInt::from(168696)
    };

    static ref ORIGIN: Point::<BabyJubjubField> = Point::<BabyJubjubField> {
        x: BabyJubjubField::new(&BN_0),
        y: BabyJubjubField::new(&BN_1),
    };

    static ref BASEPOINT: Point::<BabyJubjubField> = Point::<BabyJubjubField> {
        x: BabyJubjubField::new(
            &BigInt::parse_bytes(b"5299619240641551281634865583518297030282874472190772894086521144482721001553", 10).unwrap()
        ),
        y: BabyJubjubField::new(
            &BigInt::parse_bytes(b"16950150798460657717958625567821834550301663161624707787222815936182638968203", 10).unwrap()
        )
    };

    static ref ORDER: BigInt = BigInt::parse_bytes(
        b"21888242871839275222246405745257275088614511777268538073601725287587578984328",
        10,
    )
    .unwrap();

    static ref F_ONE: BabyJubjubField = BabyJubjubField::new(&BN_1);
}

impl BabyJubjubPoint {
    fn _mul(&self, k: &BigInt) -> BabyJubjubPoint {
        let mut base = self.clone();
        let mut acc = BabyJubjubPoint::get_origin().clone();
        let mut k = k.clone();

        while !k.is_zero() {
            if k.is_odd() {
                acc = &acc + &base;
            }
            base = &base + &base;
            k = k >> 1u32;
        }

        acc
    }
}

impl Curve<BabyJubjubField> for BabyJubjubPoint {
    fn get_a() -> &'static BabyJubjubField {
        &A
    }

    fn get_d() -> &'static BabyJubjubField {
        &D
    }

    fn get_origin() -> &'static Point<BabyJubjubField> {
        &ORIGIN
    }

    fn get_basepoint() -> &'static Point<BabyJubjubField> {
        &BASEPOINT
    }

    fn get_order() -> &'static BigInt {
        &ORDER
    }

    fn encode(&self) -> [u8; 32] {
        let mut encode = self.y.encode();
        if self.x.v > BabyJubjubField::order() / 2 {
            encode[31] |= 0x80;
        }
        encode
    }
    
    fn decode(encode: &[u8]) -> Result<Self, Error> {
        let mut sign = false;
        let mut y = [0; 32];
        y[..].copy_from_slice(encode);
        if y[31] & 0x80 != 0 {
            sign = true;
        }
        y[31] &= 0x7f;
        let y = BabyJubjubField::decode(&y);
        let numerator = BabyJubjubField::new(&BN_1) - &y * &y;
        let denominator = (Self::get_a() - Self::get_d() * &y * &y).inv();
        let mut x = (numerator * denominator).sqrt()?.v;
        if (sign && (x <= BabyJubjubField::order() / 2))
            || (!sign && (x > BabyJubjubField::order() / 2))
        {
            x = BabyJubjubField::order() - x;
        }
        let x = BabyJubjubField::new(&x);
        Ok(Point::<BabyJubjubField> { x, y })
    }
}
