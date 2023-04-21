use crate::t_funk::{Splitted, Spread, Spreaded};

use super::{Arr, Split};

pub trait Fanout<F> {
    type Fanout;

    fn fanout(self, f: F) -> Self::Fanout;
}

impl<T, F> Fanout<F> for T
where
    T: Split<F>,
    Spreaded<Splitted<T, F>>: Arr,
{
    type Fanout = <Spreaded<Splitted<T, F>> as Arr>::Arr;

    fn fanout(self, f: F) -> Self::Fanout {
        self.split(f).spread().arr()
    }
}

