use num_bigint::{BigInt, ToBigInt};
use sha2::{Digest, Sha512};
use std::ops::{Add, Mul};

mod babyjubjub;
mod curve;
mod finite_field;
mod key;

pub use crate::babyjubjub::BabyJubjubField;
pub use crate::curve::{Curve, Point};
pub use crate::finite_field::{bn_0, bn_1, bn_2, Encode, Field, FiniteField, Order};
pub use crate::key::{Sign, EDDSA};

type BabyJubjubPoint = Point<BabyJubjubField>;

impl PartialEq for BabyJubjubPoint {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Curve<BabyJubjubField> for BabyJubjubPoint {
    fn get_a() -> BabyJubjubField {
        BabyJubjubField::new(BigInt::parse_bytes(b"168700", 10).unwrap())
    }

    fn get_d() -> BabyJubjubField {
        BabyJubjubField::new(BigInt::parse_bytes(b"168696", 10).unwrap())
    }

    fn get_origin() -> Point<BabyJubjubField> {
        Point::<BabyJubjubField> {
            x: BabyJubjubField::new(bn_0()),
            y: BabyJubjubField::new(bn_1()),
        }
    }

    fn get_basepoint() -> Point<BabyJubjubField> {
        Point::<BabyJubjubField> {
            x: BabyJubjubField::new(BigInt::parse_bytes(b"5299619240641551281634865583518297030282874472190772894086521144482721001553", 10).unwrap()),
            y: BabyJubjubField::new(BigInt::parse_bytes(b"16950150798460657717958625567821834550301663161624707787222815936182638968203", 10).unwrap())
        }
    }

    fn get_order() -> BabyJubjubField {
        BabyJubjubField::new(
            BigInt::parse_bytes(
                b"21888242871839275222246405745257275088614511777268538073601725287587578984328",
                10,
            )
            .unwrap(),
        )
    }

    fn encode(&self) -> [u8; 32] {
        let mut encode = self.y.encode();
        if self.x.v > BabyJubjubField::order().v / 2 {
            encode[31] |= 0x80;
        }
        encode
        // BabyJubjubField::new((self.x.clone().v & bn_1() << 255) | self.y.clone().v).encode()
    }

    fn mul_scalar(self, k: BigInt) -> Self {
        if k == bn_0() {
            BabyJubjubPoint::get_origin()
        } else if k == bn_1() {
            self
        } else if k.clone() % bn_2() == bn_1() {
            self.clone() + self.mul_scalar(k - 1)
        } else {
            let p = self.clone() + self;
            p.mul_scalar(k >> 1)
        }
    }
}

impl Add for BabyJubjubPoint {
    type Output = BabyJubjubPoint;

    fn add(self, other: Self) -> Self::Output {
        let x1 = self.x;
        let y1 = self.y;
        let x2 = other.x.clone();
        let y2 = other.y.clone();
        let a = Self::get_a();
        let d = Self::get_d();
        let one: BabyJubjubField = BabyJubjubField::new(bn_1());

        let tmp = d.clone() * x1.clone() * x2.clone() * y1.clone() * y2.clone();

        // ref: https://eips.ethereum.org/EIPS/eip-2494
        //   x3 = (x1*y2 + y1*x2)/(1 + d*x1*x2*y1*y2)
        //   y3 = (y1*y2 - a*x1*x2)/(1 - d*x1*x2*y1*y2)
        // let tmp1 = (y1.clone() * y2.clone() - a.clone() * x1.clone() * x2.clone());
        // let tmp2 = (one.clone() - tmp.clone());
        // let tmp3 = tmp1.clone() / tmp2.clone();
        Self {
            x: (x1.clone() * y2.clone() + y1.clone() * x2.clone()) / (one.clone() + tmp.clone()),
            y: (y1.clone() * y2.clone() - a * x1.clone() * x2.clone())
                / (one.clone() - tmp.clone()),
        }
    }
}

impl Mul<BabyJubjubField> for BabyJubjubPoint {
    type Output = BabyJubjubPoint;

    fn mul(self, k: BabyJubjubField) -> Self {
        if k.clone().v == bn_0() {
            BabyJubjubPoint::get_origin()
        } else if k.clone().v == bn_1() {
            self.clone()
        } else if k.clone().v % bn_2() == bn_1() {
            self.clone() + self.clone().mul(BabyJubjubField::new(k.clone().v - 1))
        } else {
            let p = self.clone() + self.clone();
            p.mul(BabyJubjubField::new(k.clone().v / 2))
        }
    }
}

trait EllipticCurve<T> {}

type BabyJubjub = EllipticCurve<BabyJubjubPoint>;

impl EDDSA<BabyJubjubField, BabyJubjubPoint> for BabyJubjub {
    fn secret_scalar(secret_key: &BabyJubjubField) -> BigInt {
        let mut s = [0u8; 32];
        let (_, s_be) = secret_key.v.to_bytes_be();
        s[(32 - s_be.len())..32].copy_from_slice(&s_be[..]);
        let h = Self::hash(&s);
        let mut h = h[..32].to_vec();

        h[0] &= 0xF8;
        h[31] &= 0x7F;
        h[31] |= 0x40;

        let s = BigInt::from_bytes_le(num_bigint::Sign::Plus, h.as_slice());
        // FIXME: why
        s >> 3
    }

    // https://datatracker.ietf.org/doc/html/rfc8032#section-5.1.5
    fn pubkey_from_secretkey(
        secret_key: &BabyJubjubField,
    ) -> <Point<BabyJubjubField> as Mul<BabyJubjubField>>::Output {
        let scalar_key = Self::secret_scalar(secret_key);

        BabyJubjubPoint::get_basepoint() * BabyJubjubField::new(scalar_key)
    }

    fn verify(data: &[u8], signature: Sign<BabyJubjubField>, public_key: BabyJubjubPoint) -> bool {
        let h = Self::hash(
            [&signature.r.encode(), &public_key.encode(), data]
                .concat()
                .as_slice(),
        );
        let concat = BigInt::from_bytes_le(num_bigint::Sign::Plus, h.as_slice());

        let l = BabyJubjubPoint::get_basepoint() * signature.s;
        let r1 = public_key.mul_scalar(8 * concat);
        let r2 = signature.r + r1.clone();

        l == r2
    }

    fn sign(data: &[u8], secret_key: BabyJubjubField) -> Sign<BabyJubjubField> {
        let h = Self::hash(&secret_key.to_array());
        let h = h.as_slice();
        let pk = Self::pubkey_from_secretkey(&secret_key);

        let mut s_bytes = [0u8; 32];
        s_bytes[..].copy_from_slice(&h[..32]);
        s_bytes[0] &= 0xF8;
        s_bytes[31] &= 0x7F;
        s_bytes[31] |= 0x40;

        let s = BigInt::from_bytes_le(num_bigint::Sign::Plus, &s_bytes);

        let r = Self::hash(&[&h[32..], data].concat().as_slice());
        let r = BigInt::from_bytes_le(num_bigint::Sign::Plus, r.as_slice())
            % BabyJubjubField::suborder().v;

        let R = BabyJubjubPoint::get_basepoint() * BabyJubjubField::new(r.clone());

        let concat = [&R.encode(), &pk.encode(), data].concat();
        let hash_concat = Self::hash(concat.as_slice());
        let concat = BigInt::from_bytes_le(num_bigint::Sign::Plus, hash_concat.as_slice());

        let S =
            BabyJubjubField::new((r.clone() + concat.clone() * s) % BabyJubjubField::suborder().v);
        Sign::<BabyJubjubField> {
            r: R.clone(),
            s: S.clone(),
        }
    }

    fn hash(data: &[u8]) -> Vec<u8> {
        let mut hasher = Sha512::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn babyjubjub_point_add() {
        let p1 = Point::<BabyJubjubField> {
            x: BabyJubjubField::new(BigInt::parse_bytes(b"17777552123799933955779906779655732241715742912184938656739573121738514868268", 10).unwrap()),
            y: BabyJubjubField::new(BigInt::parse_bytes(b"2626589144620713026669568689430873010625803728049924121243784502389097019475", 10).unwrap())
        };
        let p2 = Point::<BabyJubjubField> {
            x: BabyJubjubField::new(BigInt::parse_bytes(b"16540640123574156134436876038791482806971768689494387082833631921987005038935", 10).unwrap()),
            y: BabyJubjubField::new(BigInt::parse_bytes(b"20819045374670962167435360035096875258406992893633759881276124905556507972311", 10).unwrap())
        };
        let p3 = Point::<BabyJubjubField> {
            x: BabyJubjubField::new(BigInt::parse_bytes(b"7916061937171219682591368294088513039687205273691143098332585753343424131937", 10).unwrap()),
            y: BabyJubjubField::new(BigInt::parse_bytes(b"14035240266687799601661095864649209771790948434046947201833777492504781204499", 10).unwrap())
        };

        let p1_double = Point::<BabyJubjubField> {
            x: BabyJubjubField::new(
                BigInt::parse_bytes(
                    b"6890855772600357754907169075114257697580319025794532037257385534741338397365",
                    10,
                )
                .unwrap(),
            ),
            y: BabyJubjubField::new(
                BigInt::parse_bytes(
                    b"4338620300185947561074059802482547481416142213883829469920100239455078257889",
                    10,
                )
                .unwrap(),
            ),
        };

        let id = Point::<BabyJubjubField> {
            x: BabyJubjubField::new(BigInt::parse_bytes(b"0", 10).unwrap()),
            y: BabyJubjubField::new(BigInt::parse_bytes(b"1", 10).unwrap()),
        };

        assert_eq!(p1.clone() + p2, p3);
        assert_eq!(id.clone() + id.clone(), id.clone());
        assert_eq!(p1.clone() + p1.clone(), p1_double);
    }

    #[test]
    fn babyjubjub_point_mul() {
        let p1 = Point::<BabyJubjubField>::get_basepoint();

        let p2 = Point::<BabyJubjubField> {
            x: BabyJubjubField::new(BigInt::parse_bytes(b"0", 10).unwrap()),
            y: BabyJubjubField::new(BigInt::parse_bytes(b"1", 10).unwrap()),
        };
        let l = BabyJubjubField::new(
            BigInt::parse_bytes(
                b"2736030358979909402780800718157159386076813972158567259200215660948447373041",
                10,
            )
            .unwrap(),
        );

        let z = BabyJubjubField::new(BigInt::parse_bytes(b"0", 10).unwrap());
        let o = BabyJubjubField::new(BigInt::parse_bytes(b"1", 10).unwrap());
        assert_eq!(p1.clone().mul(z), BabyJubjubPoint::get_origin().clone());
        assert_eq!(p1.clone().mul(o), p1.clone());
        assert_eq!(
            p1.clone() + p1.clone(),
            p1.clone()
                .mul(BabyJubjubField::new(BigInt::parse_bytes(b"2", 10).unwrap()))
        );
        assert_eq!(
            p1.clone() + p1.clone() + p1.clone(),
            p1.clone()
                .mul(BabyJubjubField::new(BigInt::parse_bytes(b"3", 10).unwrap()))
        );
        assert_eq!(
            p1.clone() + p1.clone() + p1.clone() + p1.clone(),
            p1.clone()
                .mul(BabyJubjubField::new(BigInt::parse_bytes(b"4", 10).unwrap()))
        );
        assert_eq!(
            p1.clone() + p1.clone() + p1.clone() + p1.clone() + p1.clone(),
            p1.clone()
                .mul(BabyJubjubField::new(BigInt::parse_bytes(b"5", 10).unwrap()))
        );
        assert_eq!(
            p1.clone() + p1.clone() + p1.clone() + p1.clone() + p1.clone() + p1.clone(),
            p1.clone()
                .mul(BabyJubjubField::new(BigInt::parse_bytes(b"6", 10).unwrap()))
        );
        assert_eq!(
            p1.clone()
                + p1.clone()
                + p1.clone()
                + p1.clone()
                + p1.clone()
                + p1.clone()
                + p1.clone(),
            p1.clone()
                .mul(BabyJubjubField::new(BigInt::parse_bytes(b"7", 10).unwrap()))
        );
        assert_eq!(
            p1.clone()
                + p1.clone()
                + p1.clone()
                + p1.clone()
                + p1.clone()
                + p1.clone()
                + p1.clone()
                + p1.clone(),
            p1.clone()
                .mul(BabyJubjubField::new(BigInt::parse_bytes(b"8", 10).unwrap()))
        );
        assert_eq!(p1.clone().mul(l.clone()), p2.clone());

        let p = Point::<BabyJubjubField> {
            x: BabyJubjubField::new(BigInt::parse_bytes(b"17777552123799933955779906779655732241715742912184938656739573121738514868268", 10).unwrap()),
            y: BabyJubjubField::new(BigInt::parse_bytes(b"2626589144620713026669568689430873010625803728049924121243784502389097019475", 10).unwrap())
        };
        let p = p * BabyJubjubField::new(3.to_bigint().unwrap());
        assert_eq!(p.x, BabyJubjubField::new(BigInt::parse_bytes(b"19372461775513343691590086534037741906533799473648040012278229434133483800898", 10).unwrap()));
        assert_eq!(
            p.y,
            BabyJubjubField::new(
                BigInt::parse_bytes(
                    b"9458658722007214007257525444427903161243386465067105737478306991484593958249",
                    10
                )
                .unwrap()
            )
        );

        let r = BabyJubjubField::new(
            BigInt::parse_bytes(
                b"998509002261817064039893525009363315223455691288800741227950990424097427109",
                10,
            )
            .unwrap(),
        );
        let p = p1.clone();
        let p = p * r;
        assert_eq!(
            p.x,
            BabyJubjubField::new(
                BigInt::parse_bytes(
                    b"192b4e51adf302c8139d356d0e08e2404b5ace440ef41fc78f5c4f2428df0765",
                    16
                )
                .unwrap()
            )
        );
        assert_eq!(
            p.y,
            BabyJubjubField::new(
                BigInt::parse_bytes(
                    b"2202bebcf57b820863e0acc88970b6ca7d987a0d513c2ddeb42e3f5d31b4eddf",
                    16
                )
                .unwrap()
            )
        );

        let p = Point::<BabyJubjubField> {
            x: BabyJubjubField::new(BigInt::parse_bytes(b"13277427435165878497778222415993513565335242147425444199013288855685581939618", 10).unwrap()),
            y: BabyJubjubField::new(BigInt::parse_bytes(b"13622229784656158136036771217484571176836296686641868549125388198837476602820", 10).unwrap()),
        };
        let e = Point::<BabyJubjubField> {
            x: BabyJubjubField::new(BigInt::parse_bytes(b"19785544666114077538072763147637156577218052709759168023462664489457396832522", 10).unwrap()),
            y: BabyJubjubField::new(BigInt::parse_bytes(b"16614218219728383555739033070527024479890982726267810004762494735367721033618", 10).unwrap()),
        };
        let scalar = 8.to_bigint().unwrap() * BigInt::parse_bytes(b"3555222839185221705021491425814961952405519748427783402552617991682219862759662171839441019252996282066424942781038390250059889384698141532893051346697349", 10).unwrap();

        assert_eq!(p.mul_scalar(scalar), e);
    }

    #[test]
    fn test_signal_verify() {
        let secret_key = BabyJubjubField::new(BigInt::parse_bytes(b"2", 10).unwrap());
        let public_key = BabyJubjub::pubkey_from_secretkey(&secret_key);
        let msg = [1 as u8; 3];

        let sign = BabyJubjub::sign(&msg, secret_key);
        let verify = BabyJubjub::verify(&msg, sign, public_key);
        assert!(verify)
    }

    #[test]
    fn test_sv() {
        let secret_key = BabyJubjubField::new(
            BigInt::parse_bytes(
                b"0001020304050607080900010203040506070809000102030405060708090001",
                16,
            )
            .unwrap(),
        );
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
        let sign = BabyJubjub::sign(&msg, secret_key);
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
