use crate::t_funk::{Closure, Foldl};

use super::{Cons, HList, Nil};

impl<Head, Tail, Z, F> Foldl<F, Z> for Cons<Head, Tail>
where
    Self: HList,
    Tail: Foldl<F, F::Output>,
    F: Clone + Closure<(Z, Head)>,
{
    type Foldl = Tail::Foldl;

    fn foldl(self, f: F, z: Z) -> Self::Foldl {
        self.1.foldl(f.clone(), f.call((z, self.0)))
    }
}

impl<F, T> Foldl<F, T> for Nil {
    type Foldl = T;

    fn foldl(self, _: F, z: T) -> Self::Foldl {
        z
    }
}

#[cfg(test)]
mod test {
    use crate::t_funk::{tlist::ToHList, Foldl, Sub};

    #[test]
    fn test_hlist_foldl() {
        let list = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10).to_hlist();
        let res = list.foldl(Sub, 0);
        assert_eq!(res, -1 - 2 - 3 - 4 - 5 - 6 - 7 - 8 - 9 - 10);
    }
}
