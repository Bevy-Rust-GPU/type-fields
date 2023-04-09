use crate::hlist::{cons::ConsListMut, tuple::ConsRef};

/// Mutably borrow a cons list from a flat tuple.
/// ex. `&mut (1, 2, 3, 4)` -> `(&mut 1, (&mut 2, (&mut 3, (&mut 4, ()))))`
pub trait ConsMut: ConsRef {
    type ConsMut<'a>: ConsListMut<'a> where Self: 'a;

    fn cons_mut<'a>(&'a mut self) -> Self::ConsMut<'a>;
}

impl ConsMut for () {
    type ConsMut<'a> = ();

    fn cons_mut<'a>(&'a mut self) -> Self::ConsMut<'a> {
        ()
    }
}
