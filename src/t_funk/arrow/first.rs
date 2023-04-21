use type_fields_macros::functions;

use crate::t_funk::Id;

use super::{Split, Splitted};

#[functions]
pub trait First {
    type First;

    fn first(self) -> Self::First;
}

impl<T> First for T
where
    T: Split<Id>,
{
    type First = Splitted<Self, Id>;

    fn first(self) -> Self::First {
        self.split(Id)
    }
}

