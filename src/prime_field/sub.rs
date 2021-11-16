use sp_std::ops::Sub;
use super::{Field, Order};

impl<'a, 'b> Sub<&'b Field> for &'a Field {
    type Output = Field;

    fn sub(self, other: &Field) -> Field {
        Field {
            v: (&self.v + Field::order() - &other.v) % Field::order(),
        }
    }
}

impl<'a> Sub<Field> for &'a Field {
    type Output = Field;

    fn sub(self, other: Field) -> Field {
        Field {
            v: (&self.v + Field::order() - other.v) % Field::order(),
        }
    }
}

impl<'b> Sub<&'b Field> for Field {
    type Output = Field;

    fn sub(self, other: &Field) -> Field {
        Field {
            v: (self.v + Field::order() - &other.v) % Field::order(),
        }
    }
}

impl Sub for Field {
    type Output = Field;

    fn sub(self, other: Field) -> Field {
        Field {
            v: (self.v + Field::order() - other.v) % Field::order(),
        }
    }
}