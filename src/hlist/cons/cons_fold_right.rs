use crate::functional::Function;

use super::ConsList;

pub trait ConsFoldRight<T, F>: ConsList {
    type Folded;

    fn cons_fold_right(self, acc: T, f: F) -> Self::Folded;
}

impl<Head, Tail, T, F> ConsFoldRight<T, F> for (Head, Tail)
where
    Self: ConsList,
    Tail: ConsFoldRight<T, F>,
    F: Clone + Function<(Tail::Folded, Head)>,
{
    type Folded = F::Output;

    fn cons_fold_right(self, acc: T, f: F) -> Self::Folded {
        f.clone().call((self.1.cons_fold_right(acc, f), self.0))
    }
}

impl<T, F> ConsFoldRight<T, F> for () {
    type Folded = T;

    fn cons_fold_right(self, acc: T, _: F) -> Self::Folded {
        acc
    }
}

#[cfg(test)]
mod test {
    use super::ConsFoldRight;
    use crate::{functional::Function, hlist::tuple::Cons};

    #[derive(Clone)]
    struct Sub;

    impl Function<(i32, i32)> for Sub {
        type Output = i32;

        fn call(self, (lhs, rhs): (i32, i32)) -> Self::Output {
            lhs - rhs
        }
    }

    #[test]
    fn test_cons_fold_right() {
        let list = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10).cons();
        let res = list.cons_fold_right(0, Sub);
        assert_eq!(res, -10 - 9 - 8 - 7 - 6 - 5 - 4 - 3 - 2 - 1);
    }
}
