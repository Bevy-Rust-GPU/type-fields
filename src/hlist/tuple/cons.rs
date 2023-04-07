use crate::hlist::cons::ConsList;

/// Convert a flat tuple into a cons list.
/// ex. `(1, 2, 3, 4)` -> `(1, (2, (3, (4, ()))))`
pub trait Cons {
    type Cons: ConsList<Uncons = Self>;

    fn cons(self) -> Self::Cons;
}

impl Cons for () {
    type Cons = ();

    fn cons(self) -> Self::Cons {
        ()
    }
}
