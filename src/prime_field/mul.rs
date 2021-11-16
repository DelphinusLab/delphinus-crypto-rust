use sp_std::ops::Mul;
use super::{Field, Order};

impl<'a, 'b> Mul<&'b Field> for &'a Field {
    type Output = Field;

    fn mul(self, other: &Field) -> Field {
        Field {
            v: (&self.v * &other.v) % Field::order(),
        }
    }
}

impl<'a> Mul<Field> for &'a Field {
    type Output = Field;

    fn mul(self, other: Field) -> Field {
        Field {
            v: (&self.v * other.v) % Field::order(),
        }
    }
}

impl<'b> Mul<&'b Field> for Field {
    type Output = Field;

    fn mul(self, other: &Field) -> Field {
        Field {
            v: (self.v * &other.v) % Field::order(),
        }
    }
}

impl Mul for Field {
    type Output = Field;

    fn mul(self, other: Field) -> Field {
        Field {
            v: (self.v * other.v) % Field::order(),
        }
    }
}