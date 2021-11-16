use super::*;
use sp_std::ops::Add;

impl<'a, 'b> Add<&'b BabyJubjubPoint> for &'a BabyJubjubPoint {
    type Output = BabyJubjubPoint;

    fn add(self, other: &BabyJubjubPoint) -> BabyJubjubPoint {
        let x1 = &self.x;
        let y1 = &self.y;
        let x2 = &other.x;
        let y2 = &other.y;
        let a = BabyJubjubPoint::get_a();
        let d = BabyJubjubPoint::get_d();
        let one: &BabyJubjubField = &F_ONE;

        let tmp = &(d * x1 * x2 * y1 * y2);

        // ref: https://eips.ethereum.org/EIPS/eip-2494
        BabyJubjubPoint {
            x: (x1 * y2 + y1 * x2) / (one + tmp),
            y: (y1 * y2 - a * x1 * x2) / (one - tmp),
        }
    }
}

impl<'b> Add<&'b BabyJubjubPoint> for BabyJubjubPoint {
    type Output = BabyJubjubPoint;

    fn add(self, other: &BabyJubjubPoint) -> BabyJubjubPoint {
        &self + other
    }
}

impl<'a> Add<BabyJubjubPoint> for &'a BabyJubjubPoint {
    type Output = BabyJubjubPoint;

    fn add(self, other: BabyJubjubPoint) -> BabyJubjubPoint {
        self + &other
    }
}

impl Add<BabyJubjubPoint> for BabyJubjubPoint {
    type Output = BabyJubjubPoint;

    fn add(self, other: BabyJubjubPoint) -> BabyJubjubPoint {
        &self + &other
    }
}

