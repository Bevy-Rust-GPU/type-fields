mod fold;
mod fold_map;
mod foldl;
mod foldr;

pub use fold::*;
pub use fold_map::*;
pub use foldl::*;
pub use foldr::*;

pub trait Foldable {
    type FoldMapped<F>
    where
        Self: FoldMap<F>;
    type Foldred<F, Z>
    where
        Self: Foldr<F, Z>;

    fn fold_map<F>(self, f: F) -> Self::FoldMapped<F>
    where
        Self: FoldMap<F>;
}

impl<T> Foldable for T {
    type FoldMapped<F> = T::FoldMap
    where
        T: FoldMap<F>;

    type Foldred<F, Z> = T::Foldr
    where
        T: Foldr<F, Z>;

    fn fold_map<F>(self, f: F) -> Self::FoldMapped<F>
    where
        Self: FoldMap<F>,
    {
        self.fold_map(f)
    }
}

#[cfg(test)]
mod test {
    use crate::t_funk::{
        closure::Compose, tlist::ToHList, Closure, Curry, Dual, Endo, Flip, FoldMap, PointF, Sub,
    };

    // 'default' impl of Foldl with respect to Dual / Endo.
    //
    // May be useful for derives, but no compatible types outside of the already-specialized
    // HList have been implemented yet.
    #[test]
    fn test_derived_foldl() {
        let t = (1, 2, 3).to_hlist();

        let Dual(Endo(f)) = t.fold_map(
            PointF::<Dual<_>>::default()
                .compose(PointF::<Endo<_>>::default())
                .compose(Sub.flip().curry()),
        );

        let res = f.call(0);

        assert_eq!(res, 0 - 1 - 2 - 3);
    }
}
