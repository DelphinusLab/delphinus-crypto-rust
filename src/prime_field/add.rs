use sp_std::ops::Add;
use super::{Field, Order};

impl<'a, 'b> Add<&'b Field> for &'a Field {
    type Output = Field;

    fn add(self, other: &Field) -> Field {
        Field {
            v: (&self.v + &other.v) % Field::order(),
        }
    }
}

impl<'a> Add<Field> for &'a Field {
    type Output = Field;

    fn add(self, other: Field) -> Field {
        Field {
            v: (&self.v + other.v) % Field::order(),
        }
    }
}

impl<'b> Add<&'b Field> for Field {
    type Output = Field;

    fn add(self, other: &Field) -> Field {
        Field {
            v: (self.v + &other.v) % Field::order(),
        }
    }
}

impl Add for Field {
    type Output = Field;

    fn add(self, other: Field) -> Field {
        Field {
            v: (self.v + other.v) % Field::order(),
        }
    }
}