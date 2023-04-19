use type_fields_macros::functions;

use crate::t_funk::{Closure, Compose, Composed, Copointed, Endo, PointF};

use super::FoldMap;

#[functions]
pub trait Foldr<F, I1, I2> {
    type Foldr;

    fn foldr(self, f: F, z: I2) -> Self::Foldr;
}

impl<T, F, I1, I2> Foldr<F, I1, I2> for T
where
    T: FoldMap<Composed<F, PointF<Endo<F::Output>>>>,
    F: Closure<I1>,
    T::FoldMap: Copointed,
    <T::FoldMap as Copointed>::Copointed: Closure<I2>,
{
    type Foldr = <<T::FoldMap as Copointed>::Copointed as Closure<I2>>::Output;

    fn foldr(self, f: F, z: I2) -> Self::Foldr {
        self.fold_map(f.compose(PointF::default()))
            .copoint()
            .call(z)
    }
}
