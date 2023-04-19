use crate::t_funk::Closure;

use super::HList;

pub trait FoldRight<T, F>: HList {
    type Folded;

    fn fold_right(self, acc: T, f: F) -> Self::Folded;
}

impl<Head, Tail, T, F> FoldRight<T, F> for (Head, Tail)
where
    Self: HList,
    Tail: FoldRight<T, F>,
    F: Clone + Closure<(Tail::Folded, Head)>,
{
    type Folded = F::Output;

    fn fold_right(self, acc: T, f: F) -> Self::Folded {
        f.clone().call((self.1.fold_right(acc, f), self.0))
    }
}

impl<T, F> FoldRight<T, F> for () {
    type Folded = T;

    fn fold_right(self, acc: T, _: F) -> Self::Folded {
        acc
    }
}

#[cfg(test)]
mod test {
    use super::FoldRight;
    use crate::t_funk::{tlist::ToHList, Sub};

    #[test]
    fn test_cons_fold_right() {
        let list = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10).to_hlist();
        let res = list.fold_right(0, Sub);
        assert_eq!(res, -10 - 9 - 8 - 7 - 6 - 5 - 4 - 3 - 2 - 1);
    }
}
