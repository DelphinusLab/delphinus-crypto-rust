#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
extern crate lazy_static;

use num_bigint::BigInt;
use sha2::{Digest, Sha256, Sha512};

mod babyjubjub;
mod babyjubjub_point;
mod curve;
mod key;
mod prime_field;

pub use crate::babyjubjub::BabyJubjubField;
pub use crate::babyjubjub_point::BabyJubjubPoint;
pub use crate::curve::{Curve, Point};
pub use crate::key::{Sign, EDDSA};
pub use crate::prime_field::{Encode, Field, Order, PrimeField, BN_0, BN_1, BN_2};

pub trait EllipticCurve<T> {}

pub type BabyJubjub = dyn EllipticCurve<BabyJubjubPoint>;

impl EDDSA<BabyJubjubField, BabyJubjubPoint> for BabyJubjub {
    fn secret_scalar(secret_key: &[u8]) -> BigInt {
        let mut h = Self::hash_key(&secret_key);

        h[0] &= 0xF8;
        h[31] &= 0x7F;
        h[31] |= 0x40;

        let s = BigInt::from_bytes_le(num_bigint::Sign::Plus, &h[..32]);
        // FIXME: why
        s >> 3
    }

    // https://datatracker.ietf.org/doc/html/rfc8032#section-5.1.5
    fn pubkey_from_secretkey(secret_key: &[u8]) -> Point<BabyJubjubField> {
        let scalar_key = Self::secret_scalar(secret_key);

        BabyJubjubPoint::get_basepoint() * scalar_key
    }

    fn verify(data: &[u8], signature: Sign<BabyJubjubField>, public_key: BabyJubjubPoint) -> bool {
        let h = Self::hash_msg(&([&signature.r.encode(), &public_key.encode(), data].concat()));
        let concat = BigInt::from_bytes_le(num_bigint::Sign::Plus, &h);

        let l = BabyJubjubPoint::get_basepoint() * &signature.s.v;
        let r1 = public_key * (8 * concat);
        let r2 = signature.r + r1;

        l == r2
    }

    fn sign(data: &[u8], secret_key: &[u8]) -> Sign<BabyJubjubField> {
        let h = Self::hash_key(secret_key);
        let pk = Self::pubkey_from_secretkey(&secret_key);

        let mut s_bytes = [0u8; 32];
        s_bytes[..].copy_from_slice(&h[..32]);
        s_bytes[0] &= 0xF8;
        s_bytes[31] &= 0x7F;
        s_bytes[31] |= 0x40;

        let s = BigInt::from_bytes_le(num_bigint::Sign::Plus, &s_bytes);

        let r = Self::hash_key(&[&h[32..], data].concat());
        let r = BigInt::from_bytes_le(num_bigint::Sign::Plus, &r) % BabyJubjubField::suborder();

        let sig_r = BabyJubjubPoint::get_basepoint() * &r;

        let concat = [&sig_r.encode(), &pk.encode(), data].concat();
        let hash_concat = Self::hash_msg(&concat);
        let concat = BigInt::from_bytes_le(num_bigint::Sign::Plus, &hash_concat);

        let sig_s = BabyJubjubField::new(&((r + concat * s) % BabyJubjubField::suborder()));
        Sign::<BabyJubjubField> { r: sig_r, s: sig_s }
    }

    fn hash_key(data: &[u8]) -> [u8; 64] {
        let mut res = [0u8; 64];
        let mut hasher = Sha512::new();
        hasher.update(data);
        res.copy_from_slice(&hasher.finalize());
        res
    }

    fn hash_msg(data: &[u8]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize().into()
    }
}

#[cfg(feature = "std")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "std")]
#[wasm_bindgen]
pub fn sign(msg: &[u8], secret_key: &[u8]) -> Vec<u8> {
    let sign = BabyJubjub::sign(msg, secret_key);
    [sign.r.encode(), sign.s.encode()].concat()
}

#[cfg(feature = "std")]
#[wasm_bindgen]
pub fn verify(msg: &[u8], signature: &[u8], public_key: &[u8]) -> bool {
    let r = BabyJubjubPoint::decode(&signature[..32]);
    let s = BabyJubjubField::decode(&signature[32..]);
    let pk = BabyJubjubPoint::decode(public_key);

    match (r, pk) {
        (Ok(r), Ok(pk)) => {
            let sig = Sign::<BabyJubjubField> { r, s };
            BabyJubjub::verify(msg, sig, pk)
        }
        _ => false,
    }
}

#[cfg(feature = "std")]
#[wasm_bindgen]
pub fn derive_private_key(msg: &[u8], derive_key: &[u8]) -> Vec<u8> {
    let mut seed = Sha256::new();
    seed.update(msg);
    seed.update(derive_key);

    seed.finalize().to_vec()
}

#[cfg(feature = "std")]
#[wasm_bindgen]
pub fn get_public_key(secret_key: &[u8]) -> Vec<u8> {
    BabyJubjub::pubkey_from_secretkey(&secret_key).encode().into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::ToBigInt;

    #[test]
    fn babyjubjub_point_add() {
        let p1 = Point::<BabyJubjubField> {
            x: BabyJubjubField::new(&BigInt::parse_bytes(b"17777552123799933955779906779655732241715742912184938656739573121738514868268", 10).unwrap()),
            y: BabyJubjubField::new(&BigInt::parse_bytes(b"2626589144620713026669568689430873010625803728049924121243784502389097019475", 10).unwrap())
        };
        let p2 = Point::<BabyJubjubField> {
            x: BabyJubjubField::new(&BigInt::parse_bytes(b"16540640123574156134436876038791482806971768689494387082833631921987005038935", 10).unwrap()),
            y: BabyJubjubField::new(&BigInt::parse_bytes(b"20819045374670962167435360035096875258406992893633759881276124905556507972311", 10).unwrap())
        };
        let p3 = Point::<BabyJubjubField> {
            x: BabyJubjubField::new(&BigInt::parse_bytes(b"7916061937171219682591368294088513039687205273691143098332585753343424131937", 10).unwrap()),
            y: BabyJubjubField::new(&BigInt::parse_bytes(b"14035240266687799601661095864649209771790948434046947201833777492504781204499", 10).unwrap())
        };

        let p1_double = Point::<BabyJubjubField> {
            x: BabyJubjubField::new(
                &BigInt::parse_bytes(
                    b"6890855772600357754907169075114257697580319025794532037257385534741338397365",
                    10,
                )
                .unwrap(),
            ),
            y: BabyJubjubField::new(
                &BigInt::parse_bytes(
                    b"4338620300185947561074059802482547481416142213883829469920100239455078257889",
                    10,
                )
                .unwrap(),
            ),
        };

        let id = Point::<BabyJubjubField> {
            x: BabyJubjubField::new(&BigInt::parse_bytes(b"0", 10).unwrap()),
            y: BabyJubjubField::new(&BigInt::parse_bytes(b"1", 10).unwrap()),
        };

        assert_eq!(p1.clone() + p2, p3);
        assert_eq!(id.clone() + id.clone(), id.clone());
        assert_eq!(p1.clone() + p1.clone(), p1_double);
    }

    #[test]
    fn babyjubjub_point_mul() {
        let p1 = Point::<BabyJubjubField>::get_basepoint();

        let p2 = Point::<BabyJubjubField> {
            x: BabyJubjubField::new(&BigInt::parse_bytes(b"0", 10).unwrap()),
            y: BabyJubjubField::new(&BigInt::parse_bytes(b"1", 10).unwrap()),
        };
        let l = BabyJubjubField::new(
            &BigInt::parse_bytes(
                b"2736030358979909402780800718157159386076813972158567259200215660948447373041",
                10,
            )
            .unwrap(),
        );

        assert_eq!(p1 * 0.to_bigint().unwrap(), *BabyJubjubPoint::get_origin());
        for i in 1..128 {
            assert_eq!(
                p1 * i.to_bigint().unwrap(),
                p1 * (i - 1).to_bigint().unwrap() + p1
            );
        }
        assert_eq!(p1 * &l.v, p2);

        let p = Point::<BabyJubjubField> {
            x: BabyJubjubField::new(&BigInt::parse_bytes(b"17777552123799933955779906779655732241715742912184938656739573121738514868268", 10).unwrap()),
            y: BabyJubjubField::new(&BigInt::parse_bytes(b"2626589144620713026669568689430873010625803728049924121243784502389097019475", 10).unwrap())
        };
        let p = p * &3.to_bigint().unwrap();
        assert_eq!(p.x, BabyJubjubField::new(&BigInt::parse_bytes(b"19372461775513343691590086534037741906533799473648040012278229434133483800898", 10).unwrap()));
        assert_eq!(
            p.y,
            BabyJubjubField::new(
                &BigInt::parse_bytes(
                    b"9458658722007214007257525444427903161243386465067105737478306991484593958249",
                    10
                )
                .unwrap()
            )
        );

        let r = BabyJubjubField::new(
            &BigInt::parse_bytes(
                b"998509002261817064039893525009363315223455691288800741227950990424097427109",
                10,
            )
            .unwrap(),
        );
        let p = p1.clone();
        let p = p * &r.v;
        assert_eq!(
            p.x,
            BabyJubjubField::new(
                &BigInt::parse_bytes(
                    b"192b4e51adf302c8139d356d0e08e2404b5ace440ef41fc78f5c4f2428df0765",
                    16
                )
                .unwrap()
            )
        );
        assert_eq!(
            p.y,
            BabyJubjubField::new(
                &BigInt::parse_bytes(
                    b"2202bebcf57b820863e0acc88970b6ca7d987a0d513c2ddeb42e3f5d31b4eddf",
                    16
                )
                .unwrap()
            )
        );

        let p = Point::<BabyJubjubField> {
            x: BabyJubjubField::new(&BigInt::parse_bytes(b"13277427435165878497778222415993513565335242147425444199013288855685581939618", 10).unwrap()),
            y: BabyJubjubField::new(&BigInt::parse_bytes(b"13622229784656158136036771217484571176836296686641868549125388198837476602820", 10).unwrap()),
        };
        let e = Point::<BabyJubjubField> {
            x: BabyJubjubField::new(&BigInt::parse_bytes(b"19785544666114077538072763147637156577218052709759168023462664489457396832522", 10).unwrap()),
            y: BabyJubjubField::new(&BigInt::parse_bytes(b"16614218219728383555739033070527024479890982726267810004762494735367721033618", 10).unwrap()),
        };
        let scalar = 8.to_bigint().unwrap() * BigInt::parse_bytes(b"3555222839185221705021491425814961952405519748427783402552617991682219862759662171839441019252996282066424942781038390250059889384698141532893051346697349", 10).unwrap();

        assert_eq!(p * &scalar, e);
    }

    #[test]
    fn test_signature_verify() {
        let secret_key = [2u8; 32];
        let public_key = BabyJubjub::pubkey_from_secretkey(&secret_key);
        let msg = [1 as u8; 3];

        let sign = BabyJubjub::sign(&msg, &secret_key);
        let verify = BabyJubjub::verify(&msg, sign, public_key);
        assert!(verify)
    }

    #[test]
    fn test_decode() {
        let (_, secret_key) = BigInt::parse_bytes(
            b"0001020304050607080900010203040506070809000102030405060708090001",
            16,
        )
        .unwrap().to_bytes_be();
        let public_key = BabyJubjub::pubkey_from_secretkey(&secret_key);
        assert_eq!(
            BabyJubjubPoint::decode(&public_key.encode()).unwrap(),
            public_key
        );
    }

    #[test]
    fn test_suborder() {
        assert_eq!(
            BabyJubjubPoint::get_basepoint() * BabyJubjubField::suborder(),
            BabyJubjubPoint::get_origin().clone()
        );
    }

    #[test]
    fn test_sv() {
        let (_, secret_key) = BigInt::parse_bytes(
            b"0001020304050607080900010203040506070809000102030405060708090001",
            16,
        )
        .unwrap().to_bytes_be();
        let public_key = BabyJubjub::pubkey_from_secretkey(&secret_key);

        /*
                assert_eq!(
                    public_key.x,
                    BabyJubjubField::new(
                        BigInt::parse_bytes(
                            b"1d5ac1f31407018b7d413a4f52c8f74463b30e6ac2238220ad8b254de4eaa3a2",
                            16
                        )
                        .unwrap()
                    )
                );

                assert_eq!(
                    public_key.y,
                    BabyJubjubField::new(
                        BigInt::parse_bytes(
                            b"1e1de8a908826c3f9ac2e0ceee929ecd0caf3b99b3ef24523aaab796a6f733c4",
                            16
                        )
                        .unwrap()
                    )
                );
        */
        let msg = [0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let sign = BabyJubjub::sign(&msg, &secret_key);
        /*
                assert_eq!(sign.r.x,
                BabyJubjubField::new(BigInt::parse_bytes(b"21253904451576600568378459528205653033385900307028841334532552830614710476912", 10).unwrap()));
                assert_eq!(sign.r.y,
                BabyJubjubField::new(BigInt::parse_bytes(b"20125634407542493427571099944365246191501563803226486072348038614369379124499", 10).unwrap()));
                assert_eq!(
                    sign.s,
                    BabyJubjubField::new(
                        BigInt::parse_bytes(
                            b"2184501368606900148220113640627284376983814550407071690196746129091140828676",
                            10
                        )
                        .unwrap()
                    )
                );
        */
        assert!(BabyJubjub::verify(&msg, sign, public_key));
    }
}
