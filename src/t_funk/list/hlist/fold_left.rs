use crate::t_funk::Closure;

use super::HList;

pub trait FoldLeft<T, F>: HList {
    type Folded;

    fn fold_left(self, acc: T, f: F) -> Self::Folded;
}

impl<Head, Tail, T, F> FoldLeft<T, F> for (Head, Tail)
where
    Self: HList,
    Tail: FoldLeft<F::Output, F>,
    F: Clone + Closure<(T, Head)>,
{
    type Folded = Tail::Folded;

    fn fold_left(self, acc: T, f: F) -> Self::Folded {
        self.1.fold_left(f.clone().call((acc, self.0)), f)
    }
}

impl<T, F> FoldLeft<T, F> for () {
    type Folded = T;

    fn fold_left(self, acc: T, _: F) -> Self::Folded {
        acc
    }
}

#[cfg(test)]
mod test {
    use super::FoldLeft;
    use crate::{t_funk::Sub, t_funk::tlist::ToHList};

    #[test]
    fn test_cons_fold() {
        let list = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10).to_hlist();
        let res = list.fold_left(0, Sub);
        assert_eq!(res, -1 - 2 - 3 - 4 - 5 - 6 - 7 - 8 - 9 - 10);
    }
}
