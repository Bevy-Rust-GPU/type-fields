use crate::hlist::tuple::TupleList;

/// A cons list that can be converted into a flat tuple,
/// ex. `(1, (2, (3, (4, ())))) -> (1, 2, 3, 4)`
///
/// This is a special case, in that it must be implemented via macro
/// for the sake of having a known fixed-arity tuple type to return.
pub trait Uncons {
    type Uncons: TupleList<Cons = Self>;

    fn uncons(self) -> Self::Uncons;
}

impl Uncons for () {
    type Uncons = ();

    fn uncons(self) -> Self::Uncons {
        ()
    }
}

#[cfg(test)]
mod tests {
    use crate::hlist::{cons::Uncons, tuple::Cons};

    #[test]
    fn test_uncons() {
        let consable = (1, 2, 3);
        assert_eq!(consable, consable.cons().uncons());
    }
}
