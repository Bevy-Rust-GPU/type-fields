use type_fields_macros::functions;

use crate::t_funk::Id;

use super::{Split, Splitted};

#[functions]
pub trait Second {
    type Second;

    fn second(self) -> Self::Second;
}

impl<T> Second for T
where
    T: Split<Id>,
{
    type Second = Splitted<Id, Self>;

    fn second(self) -> Self::Second {
        Id.split(self)
    }
}

