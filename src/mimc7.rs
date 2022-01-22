extern crate num_bigint;
use num_bigint::{BigInt, Sign};
use tiny_keccak::Keccak;

const SEED: &str = "delphinus";
use crate::*;

pub struct Constants {
    n_rounds: usize,
    cts: Vec<BabyJubjubField>,
}

pub fn generate_constants(n_rounds: usize) -> Constants {
    let cts = get_constants(SEED, n_rounds);

    Constants {
        n_rounds: n_rounds,
        cts: cts,
    }
}

pub fn get_constants(seed: &str, n_rounds: usize) -> Vec<BabyJubjubField> {
    let mut cts: Vec<BabyJubjubField> = Vec::new();
    cts.push(BabyJubjubField::new(&BigInt::from(0)));

    let mut keccak = Keccak::new_keccak256();
    let mut h = [0u8; 32];
    keccak.update(seed.as_bytes());
    keccak.finalize(&mut h);

    let r: BigInt = BigInt::parse_bytes(
        b"21888242871839275222246405745257275088548364400416034343698204186575808495617",
        10,
    )
    .unwrap();

    let mut c = BigInt::from_bytes_be(Sign::Plus, &h);
    for _ in 1..n_rounds {
        let (_, c_bytes) = c.to_bytes_be();
        let mut c_bytes32: [u8; 32] = [0; 32];
        let diff = c_bytes32.len() - c_bytes.len();
        c_bytes32[diff..].copy_from_slice(&c_bytes[..]);

        let mut keccak = Keccak::new_keccak256();
        let mut h = [0u8; 32];
        keccak.update(&c_bytes[..]);
        keccak.finalize(&mut h);
        c = BigInt::from_bytes_be(Sign::Plus, &h);

        let n = modulus(&c, &r);
        cts.push(BabyJubjubField::new(&n));
    }
    cts
}

pub fn modulus(a: &BigInt, m: &BigInt) -> BigInt {
    ((a % m) + m) % m
}

pub struct Mimc7 {
    constants: Constants,
}

impl Mimc7 {
    pub fn new(n_rounds: usize) -> Mimc7 {
        Mimc7 {
            constants: generate_constants(n_rounds),
        }
    }

    pub fn hash(&self, x_in: &BabyJubjubField, k: &BabyJubjubField) -> BabyJubjubField {
        let mut h = BabyJubjubField::new(&BigInt::from(0));
        for i in 0..self.constants.n_rounds {
            let mut t: BabyJubjubField;
            if i == 0 {
                t = x_in.clone();
                t = t + k;
            } else {
                t = h.clone();
                t = t + k + &self.constants.cts[i];
            }
            let mut t2 = t.clone();
            t2 = &t2 * &t2;
            let mut t7 = t2.clone();
            t7 = &t7 * &t7;
            t7 = &t7 * &t2;
            t7 = &t7 * &t;
            h = t7.clone();
        }
        h + k
    }

    pub fn multi_hash(&self, arr: Vec<BabyJubjubField>, key: &BabyJubjubField) -> BabyJubjubField {
        let mut r = key.clone();
        for i in 0..arr.len() {
            let h = self.hash(&arr[i], &r);
            r = r + &arr[i] + h;
        }
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_constants() {
        let constants = generate_constants(91);
        assert_eq!(
            constants.cts[1].v,
            BigInt::parse_bytes(b"21F808158DA8EC947458FC528A11979536679BD0E4C4DCD6D863A80BF60B23C8", 16).unwrap()
        );
    }

    #[test]
    fn test_mimc() {
        let b1 = BabyJubjubField::new(&BigInt::parse_bytes(b"1", 10).unwrap());
        let b2 = BabyJubjubField::new(&BigInt::parse_bytes(b"2", 10).unwrap());
        let mimc7 = Mimc7::new(91);
        let h1 = mimc7.hash(&b1, &b2);
        assert_eq!(
            h1.v,
            BigInt::parse_bytes(b"18ADE4CF70372A00640612BE58DF799D651C64CC78E4AA21DFE0B0193F72AF4C", 16).unwrap()
        );

        let b3 = BabyJubjubField::new(&BigInt::parse_bytes(b"3", 10).unwrap());
        let mut arr: Vec<BabyJubjubField> = Vec::new();
        arr.push(b1.clone());
        arr.push(b2.clone());
        arr.push(b3.clone());
        let h1 = mimc7.multi_hash(arr, &BabyJubjubField::new(&BigInt::from(0)));
        assert_eq!(
            h1.v,
            BigInt::parse_bytes(b"20F519F4D47AA89678AA3B4F2FE8433A60B6B83AD9EADE8019A73A749BB9F2C0", 16).unwrap()
        );

        let b12 = BabyJubjubField::new(&BigInt::parse_bytes(b"12", 10).unwrap());
        let b45 = BabyJubjubField::new(&BigInt::parse_bytes(b"45", 10).unwrap());
        let b78 = BabyJubjubField::new(&BigInt::parse_bytes(b"78", 10).unwrap());
        let b41 = BabyJubjubField::new(&BigInt::parse_bytes(b"41", 10).unwrap());

        let mut big_arr1: Vec<BabyJubjubField> = Vec::new();
        big_arr1.push(b12.clone());
        let mimc7 = Mimc7::new(91);
        let h1 = mimc7.multi_hash(big_arr1, &BabyJubjubField::new(&BigInt::from(0)));
        assert_eq!(
            h1.v,
            BigInt::parse_bytes(b"1F9374E085CE2B17592B9A47F83765A4800671DF0E7B985D242B09D61977BC24", 16).unwrap()
        );

        let mh2 = mimc7.hash(&b12, &b45);
        assert_eq!(
            mh2.v,
            BigInt::parse_bytes(b"2DC528E4A736C82FE638E27E3C84E49E7D3BD254986860FDE37BE0C558F8226E", 16).unwrap()
        );

        let mut big_arr1: Vec<BabyJubjubField> = Vec::new();
        big_arr1.push(b78.clone());
        big_arr1.push(b41.clone());
        let h2 = mimc7.multi_hash(big_arr1, &BabyJubjubField::new(&BigInt::from(0)));
        assert_eq!(
            h2.v,
            BigInt::parse_bytes(b"136CEBF896357BC3B479DC31093118BB94C91A947CD0B4E42103735BC8B5028E", 16).unwrap()
        );

        let mut big_arr1: Vec<BabyJubjubField> = Vec::new();
        big_arr1.push(b12.clone());
        big_arr1.push(b45.clone());
        let h1 = mimc7.multi_hash(big_arr1, &BabyJubjubField::new(&BigInt::from(0)));
        assert_eq!(
            h1.v,
            BigInt::parse_bytes(b"2317ED4F4FCE153F7CCCBF7D85554605460B8B2862E4CA73C44944A4F652060C", 16).unwrap()
        );

        let mut big_arr1: Vec<BabyJubjubField> = Vec::new();
        big_arr1.push(b12.clone());
        big_arr1.push(b45.clone());
        big_arr1.push(b78.clone());
        big_arr1.push(b41.clone());
        let mimc7 = Mimc7::new(91);
        let h1 = mimc7.multi_hash(big_arr1, &BabyJubjubField::new(&BigInt::from(0)));
        assert_eq!(
            h1.v,
            BigInt::parse_bytes(b"184CB92F873F46CF2A61524C749EEDDBE16968A50605E38E12964363C49893EC", 16).unwrap()
        );

        let r_1 = BabyJubjubField::new(
            &BigInt::parse_bytes(
                b"21888242871839275222246405745257275088548364400416034343698204186575808495616",
                10).unwrap());

        let mut big_arr1: Vec<BabyJubjubField> = Vec::new();
        big_arr1.push(r_1.clone());
        let mimc7 = Mimc7::new(91);
        let h1 = mimc7.multi_hash(big_arr1, &BabyJubjubField::new(&BigInt::from(0)));
        assert_eq!(
            h1.v,
            BigInt::parse_bytes(b"103F732399198F6973BC97A701297E284AF659277F90363916B6FC4C49E0D204", 16).unwrap()
        );
    }
}