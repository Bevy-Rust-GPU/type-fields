use crate::t_funk::{Branch, Closure, FoldMap, Foldl, Foldr, Leaf, Mempty, Mappend};

impl<T, F, Z> Foldr<F, Z> for Leaf<T> {
    type Foldr = Z;

    fn foldr(self, _: F, z: Z) -> Self::Foldr {
        z
    }
}

impl<T, F, Z> Foldl<F, Z> for Leaf<T> {
    type Foldl = Z;

    fn foldl(self, _: F, z: Z) -> Self::Foldl {
        z
    }
}

impl<T, F> FoldMap<F> for Leaf<T>
where
    T: Mempty,
    F: Closure<T::Mempty>,
{
    type FoldMap = F::Output;

    fn fold_map(self, f: F) -> Self::FoldMap {
        f.call(T::mempty())
    }
}

impl<L, T, R, F, Z> Foldr<F, Z> for Branch<L, T, R>
where
    R: Foldr<F, Z>,
    F: Clone + Closure<T>,
    F::Output: Closure<R::Foldr>,
    L: Foldr<F, <F::Output as Closure<R::Foldr>>::Output>,
{
    type Foldr = <L as Foldr<
        F,
        <<F as Closure<T>>::Output as Closure<<R as Foldr<F, Z>>::Foldr>>::Output,
    >>::Foldr;

    fn foldr(self, f: F, z: Z) -> Self::Foldr {
        self.0
            .foldr(f.clone(), f.clone().call(self.1).call(self.2.foldr(f, z)))
    }
}

impl<L, T, R, F, Z> Foldl<F, Z> for Branch<L, T, R>
where
    L: Foldl<F, Z>,
    F: Clone + Closure<L::Foldl>,
    F::Output: Closure<T>,
    R: Foldl<F, <F::Output as Closure<T>>::Output>,
{
    type Foldl = <R as Foldl<
        F,
        <<F as Closure<<L as Foldl<F, Z>>::Foldl>>::Output as Closure<T>>::Output,
    >>::Foldl;

    fn foldl(self, f: F, z: Z) -> Self::Foldl {
        self.2
            .foldl(f.clone(), f.clone().call(self.0.foldl(f, z)).call(self.1))
    }
}

impl<L, T, R, F> FoldMap<F> for Branch<L, T, R>
where
    L: FoldMap<F>,
    L::FoldMap: Mappend<<<F as Closure<T>>::Output as Mappend<R::FoldMap>>::Mappend>,
    F: Clone + Closure<T>,
    F::Output: Mappend<R::FoldMap>,
    R: FoldMap<F>,
{
    type FoldMap = <<L as FoldMap<F>>::FoldMap as Mappend<
        <<F as Closure<T>>::Output as Mappend<R::FoldMap>>::Mappend,
    >>::Mappend;

    fn fold_map(self, f: F) -> Self::FoldMap {
        self.0
            .fold_map(f.clone())
            .mappend(f.clone().call(self.1).mappend(self.2.fold_map(f)))
    }
}
