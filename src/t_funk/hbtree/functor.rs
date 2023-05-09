use crate::t_funk::{Branch, Closure, Fmap, Leaf, Replace};

impl<T, F> Fmap<F> for Leaf<T>
where
    T: Fmap<F>,
{
    type Fmap = Leaf<T::Fmap>;

    fn fmap(self, _: F) -> Self::Fmap {
        Leaf::default()
    }
}

impl<T, U> Replace<U> for Leaf<T> {
    type Replace = Leaf<U>;

    fn replace(self, _: U) -> Self::Replace {
        Leaf::default()
    }
}

impl<L, T, R, F> Fmap<F> for Branch<L, T, R>
where
    L: Fmap<F>,
    F: Clone + Closure<T>,
    R: Fmap<F>,
{
    type Fmap = Branch<L::Fmap, F::Output, R::Fmap>;

    fn fmap(self, f: F) -> Self::Fmap {
        Branch(
            self.0.fmap(f.clone()),
            f.clone().call(self.1),
            self.2.fmap(f),
        )
    }
}

impl<L, T, R, U> Replace<U> for Branch<L, T, R>
where
    L: Replace<U>,
    R: Replace<U>,
    U: Clone,
{
    type Replace = Branch<L::Replace, U, R::Replace>;

    fn replace(self, u: U) -> Self::Replace {
        Branch(
            self.0.replace(u.clone()),
            u.clone(),
            self.2.replace(u.clone()),
        )
    }
}
