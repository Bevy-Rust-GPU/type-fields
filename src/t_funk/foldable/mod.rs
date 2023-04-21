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
