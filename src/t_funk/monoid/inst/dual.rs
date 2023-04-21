use type_fields_macros::{
    Apply, Chain, Copointed, Fmap, Fold, FoldMap, Foldl, Foldr, Mconcat, Mempty, Pointed, Pure,
    Replace, Then,
};

use type_fields::t_funk::Mappend;

use crate::t_funk::{Copointed, Pointed};

#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Pointed,
    Copointed,
    Fmap,
    Replace,
    Pure,
    Apply,
    Chain,
    Then,
    Mempty,
    Mconcat,
    FoldMap,
    Foldr,
    Foldl,
    Fold,
)]
pub struct Dual<T>(T);

impl<T, U> Mappend<Dual<U>> for Dual<T>
where
    U: Mappend<T>,
{
    type Mappend = Dual<U::Mappend>;

    fn mappend(self, t: Dual<U>) -> Self::Mappend {
        Pointed::point(t.copoint().mappend(self.copoint()))
    }
}

