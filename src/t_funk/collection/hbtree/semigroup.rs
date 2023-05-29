use crate::t_funk::{Leaf, Mappend, Branch};

impl<T, U> Mappend<U> for Leaf<T> {
    type Mappend = U;

    fn mappend(self, u: U) -> Self::Mappend {
        u
    }
}

impl<L, T, R, U> Mappend<U> for Branch<L, T, R>
where
    R: Mappend<U>,
{
    type Mappend = Branch<L, T, R::Mappend>;

    fn mappend(self, t: U) -> Self::Mappend {
        Branch(self.0, self.1, self.2.mappend(t))
    }
}

