use crate::t_funk::{Closure, Foldr};

use super::HList;

impl<Head, Tail, F, T> Foldr<F, T> for (Head, Tail)
where
    Self: HList,
    Tail: Foldr<F, T>,
    F: Clone + Closure<(Tail::Foldr, Head)>,
{
    type Foldr = F::Output;

    fn foldr(self, f: F, t: T) -> Self::Foldr {
        f.clone().call((self.1.foldr(f, t), self.0))
    }
}

impl<F, T> Foldr<F, T> for () {
    type Foldr = T;

    fn foldr(self, _: F, z: T) -> Self::Foldr {
        z
    }
}

#[cfg(test)]
mod test {
    use crate::t_funk::{tlist::ToHList, Foldr, Sub};

    #[test]
    fn test_cons_fold_right() {
        let list = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10).to_hlist();
        let res = list.foldr(Sub, 0);
        assert_eq!(res, -10 - 9 - 8 - 7 - 6 - 5 - 4 - 3 - 2 - 1);
    }
}
