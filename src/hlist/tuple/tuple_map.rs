use crate::hlist::cons::{ConsMap, Uncons};

use super::TupleList;

pub trait TupleMap<F>: TupleList {
    type TupleMap: TupleList;

    fn map(self, f: F) -> Self::TupleMap;
}

impl<T, F> TupleMap<F> for T
where
    T: TupleList,
    T::Cons: ConsMap<F>,
    <T::Cons as ConsMap<F>>::ConsMap: Uncons,
{
    type TupleMap = <<T::Cons as ConsMap<F>>::ConsMap as Uncons>::Uncons;

    fn map(self, f: F) -> Self::TupleMap {
        self.cons().map(f).uncons()
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
