pub use crate::finite_field::{Encode, Field, FiniteField, Order, Random};
use num_bigint::BigInt;

pub type BabyJubjubField = Field;

impl Order for BabyJubjubField {
    fn order() -> Self {
        Self {
            v: BigInt::parse_bytes(
                b"21888242871839275222246405745257275088548364400416034343698204186575808495617",
                10,
            )
            .unwrap(),
        }
    }

    fn suborder() -> Self {
        Self {
            v: BigInt::parse_bytes(
                b"2736030358979909402780800718157159386076813972158567259200215660948447373041",
                10,
            )
            .unwrap(),
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn babyjubjub_add() {
        let a = BabyJubjubField::new(BigInt::parse_bytes(b"1", 10).unwrap());
        let b = BabyJubjubField::new(BigInt::parse_bytes(b"2", 10).unwrap());
        let c = BabyJubjubField::new(BigInt::parse_bytes(b"3", 10).unwrap());
        assert_eq!(a + b, c);

        let a = BabyJubjubField::new(BigInt::parse_bytes(b"1", 10).unwrap());
        let b = BabyJubjubField::new(
            BigInt::parse_bytes(
                b"21888242871839275222246405745257275088548364400416034343698204186575808495616",
                10,
            )
            .unwrap(),
        );
        let c = BabyJubjubField::new(BigInt::parse_bytes(b"0", 10).unwrap());
        assert_eq!(a + b, c);
    }

    #[test]
    fn babyjubjub_mul() {
        let a = BabyJubjubField::new(BigInt::parse_bytes(b"1", 10).unwrap());
        let b = BabyJubjubField::new(BigInt::parse_bytes(b"2", 10).unwrap());
        let c = BabyJubjubField::new(BigInt::parse_bytes(b"2", 10).unwrap());
        assert_eq!(a * b, c);

        let a = BabyJubjubField::new(BigInt::parse_bytes(b"100", 10).unwrap());
        let b = BabyJubjubField::new(BigInt::parse_bytes(b"100", 10).unwrap());
        let c = BabyJubjubField::new(BigInt::parse_bytes(b"10000", 10).unwrap());
        assert_eq!(a * b, c);
    }

    #[test]
    fn babyjubjub_div() {
        let a = BabyJubjubField::new(BigInt::parse_bytes(b"1", 10).unwrap());
        let b = BabyJubjubField::new(BigInt::parse_bytes(b"1", 10).unwrap());
        let c = BabyJubjubField::new(BigInt::parse_bytes(b"1", 10).unwrap());
        assert_eq!(a / b, c);

        let a = BabyJubjubField::new(BigInt::parse_bytes(b"5", 10).unwrap());
        let b = BabyJubjubField::new(BigInt::parse_bytes(b"2", 10).unwrap());
        let c = BabyJubjubField::new(
            BigInt::parse_bytes(
                b"10944121435919637611123202872628637544274182200208017171849102093287904247811",
                10,
            )
            .unwrap(),
        );
        assert_eq!(a / b, c);
    }

    #[test]
    fn babyjubjub_inv() {
        let a = BabyJubjubField::new(BigInt::parse_bytes(b"1", 10).unwrap());
        assert_eq!(a.inv(), a);

        let a = BabyJubjubField::new(BigInt::parse_bytes(b"2", 10).unwrap());
        let b = BabyJubjubField::new(
            BigInt::parse_bytes(
                b"10944121435919637611123202872628637544274182200208017171849102093287904247809",
                10,
            )
            .unwrap(),
        );
        assert_eq!(a.inv(), b);

        let a = BabyJubjubField::new(BigInt::parse_bytes(b"3", 10).unwrap());
        let b = BabyJubjubField::new(
            BigInt::parse_bytes(
                b"14592161914559516814830937163504850059032242933610689562465469457717205663745",
                10,
            )
            .unwrap(),
        );
        assert_eq!(a.inv(), b);
    }
}
