use crate::functional::Closure;

use super::ConsList;

pub trait ConsFoldLeft<T, F>: ConsList {
    type Folded;

    fn cons_fold_left(self, acc: T, f: F) -> Self::Folded;
}

impl<Head, Tail, T, F> ConsFoldLeft<T, F> for (Head, Tail)
where
    Self: ConsList,
    Tail: ConsFoldLeft<F::Output, F>,
    F: Clone + Closure<(T, Head)>,
{
    type Folded = Tail::Folded;

    fn cons_fold_left(self, acc: T, f: F) -> Self::Folded {
        self.1.cons_fold_left(f.clone().call((acc, self.0)), f)
    }
}

impl<T, F> ConsFoldLeft<T, F> for () {
    type Folded = T;

    fn cons_fold_left(self, acc: T, _: F) -> Self::Folded {
        acc
    }
}

#[cfg(test)]
mod test {
    use super::ConsFoldLeft;
    use crate::{functional::Sub, hlist::tuple::Cons};

    #[test]
    fn test_cons_fold() {
        let list = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10).cons();
        let res = list.cons_fold_left(0, Sub);
        assert_eq!(res, -1 - 2 - 3 - 4 - 5 - 6 - 7 - 8 - 9 - 10);
    }
}
