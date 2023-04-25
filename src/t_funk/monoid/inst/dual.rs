use crate::macros::{
    applicative::{Apply, Pure},
    foldable::{Fold, FoldMap, Foldl, Foldr},
    functor::{Fmap, Replace},
    monad::{Chain, Then},
    monoid::{Mconcat, Mempty},
    Copointed, Pointed,
};

use type_fields::t_funk::Mappend;

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
pub struct Dual<T>(pub T);

impl<T, U> Mappend<Dual<U>> for Dual<T>
where
    U: Mappend<T>,
{
    type Mappend = Dual<U::Mappend>;

    fn mappend(self, t: Dual<U>) -> Self::Mappend {
        Dual(t.0.mappend(self.0))
    }
}
