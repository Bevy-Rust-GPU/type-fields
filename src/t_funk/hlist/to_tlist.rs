use crate::{t_funk::tlist::TList, t_funk::Function};

/// A cons list that can be converted into a flat tuple,
/// ex. `(1, (2, (3, (4, ())))) -> (1, 2, 3, 4)`
///
/// This is a special case, in that it must be implemented via macro
/// for the sake of having a known fixed-arity tuple type to return.
pub trait ToTList {
    type TList: TList<HList = Self>;

    fn to_tlist(self) -> Self::TList;
}

impl ToTList for () {
    type TList = ();

    fn to_tlist(self) -> Self::TList {
        ()
    }
}

#[cfg(test)]
mod tests {
    use crate::t_funk::{hlist::ToTList, tlist::ToHList};

    #[test]
    fn test_uncons() {
        let consable = (1, 2, 3);
        assert_eq!(consable, consable.to_hlist().to_tlist());
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct UnconsF;

impl<I> Function<I> for UnconsF
where
    I: ToTList,
{
    type Output = I::TList;

    fn call(input: I) -> Self::Output {
        input.to_tlist()
    }
}
