mod fold;
mod fold_map;
mod foldr;

pub use fold::*;
pub use fold_map::*;
pub use foldr::*;

pub trait Foldable {
    type FoldMapped<F>
    where
        Self: FoldMap<F>;
    type Foldred<F, I1, I2>
    where
        Self: Foldr<F, I1, I2>;

    fn fold_map<F>(self, f: F) -> Self::FoldMapped<F>
    where
        Self: FoldMap<F>;
}

impl<T> Foldable for T {
    type FoldMapped<F> = T::FoldMap
    where
        T: FoldMap<F>;

    type Foldred<F, I1, I2> = T::Foldr
    where
        T: Foldr<F, I1, I2>;

    fn fold_map<F>(self, f: F) -> Self::FoldMapped<F>
    where
        Self: FoldMap<F>,
    {
        self.fold_map(f)
    }
}
