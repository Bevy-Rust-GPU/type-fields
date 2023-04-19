use crate::{t_funk::hlist::ToTList, t_funk::Fmap};

use super::TList;

pub trait Map<F>: TList {
    type TupleMap: TList;

    fn map(self, f: F) -> Self::TupleMap;
}

impl<T, F> Map<F> for T
where
    T: TList,
    T::HList: Fmap<F>,
    <T::HList as Fmap<F>>::Fmap: ToTList,
{
    type TupleMap = <<T::HList as Fmap<F>>::Fmap as ToTList>::TList;

    fn map(self, f: F) -> Self::TupleMap {
        self.to_hlist().fmap(f).to_tlist()
    }
}

#[cfg(test)]
mod test {
    use type_fields_macros::Closure;

    use crate::t_funk::{tlist::Map, Function};

    #[test]
    fn test_tuple_map() {
        #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
        struct Mul2;

        impl Function<u32> for Mul2 {
            type Output = u32;

            fn call(input: u32) -> u32 {
                input * 2
            }
        }

        impl Function<f32> for Mul2 {
            type Output = f32;

            fn call(input: f32) -> Self::Output {
                input * 2.0
            }
        }

        let list = (1, 2.0, 3);
        let list = list.map(Mul2);
        assert_eq!(list, (2, 4.0, 6))
    }
}
