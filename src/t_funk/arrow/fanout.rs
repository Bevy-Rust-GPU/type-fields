use type_fields_macros::functions;

use crate::t_funk::{Splitted, Spread, Spreaded};

use super::{Arr, Split};

#[functions]
pub trait Fanout<F> {
    type Fanout;

    fn fanout(self, f: F) -> Self::Fanout;
}

impl<T, F> Fanout<F> for T
where
    T: Split<F>,
    Spreaded<Splitted<T, F>>: Arr,
{
    type Fanout = Spreaded<<T as Split<F>>::Split>;

    fn fanout(self, f: F) -> Self::Fanout {
        self.split(f).spread().arr()
    }
}
