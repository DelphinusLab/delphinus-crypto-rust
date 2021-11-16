use sp_std::ops::Div;
use super::{Field, PrimeField};

impl<'a, 'b> Div<&'b Field> for &'a Field {
    type Output = Field;

    fn div(self, other: &Field) -> Field {
        self * other.inv()
    }
}

impl<'a> Div<Field> for &'a Field {
    type Output = Field;

    fn div(self, other: Field) -> Field {
        self * other.inv()
    }
}

impl<'b> Div<&'b Field> for Field {
    type Output = Field;

    fn div(self, other: &Field) -> Field {
        self * other.inv()
    }
}

impl Div for Field {
    type Output = Field;

    fn div(self, other: Field) -> Field {
        self * other.inv()
    }
}