use crate::{functional::Functor, hlist::cons::Uncons};

use super::TupleList;

pub trait TupleMap<F>: TupleList {
    type TupleMap: TupleList;

    fn map(self, f: F) -> Self::TupleMap;
}

impl<T, F> TupleMap<F> for T
where
    T: TupleList,
    T::Cons: Functor<F>,
    <T::Cons as Functor<F>>::Mapped: Uncons,
{
    type TupleMap = <<T::Cons as Functor<F>>::Mapped as Uncons>::Uncons;

    fn map(self, f: F) -> Self::TupleMap {
        self.cons().fmap(f).uncons()
    }
}

#[cfg(test)]
mod test {
    use crate::{functional::Function, hlist::tuple::TupleMap};

    #[test]
    fn test_tuple_map() {
        #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        struct Mul;

        impl Function<u32> for Mul {
            type Output = u32;

            fn call(self, input: u32) -> u32 {
                input * 2
            }
        }

        impl Function<f32> for Mul {
            type Output = f32;

            fn call(self, input: f32) -> Self::Output {
                input * 2.0
            }
        }

        let list = (1, 2.0, 3);
        let list = list.map(Mul);
        assert_eq!(list, (2, 4.0, 6))
    }
}
