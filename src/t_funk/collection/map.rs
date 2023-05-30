//! Type-level map, which associates type keys with values

use type_fields_macros::functions;

use crate::t_funk::peano::*;

#[functions]
pub trait Get<T> {
    type Get;

    fn get(self) -> Self::Get;
}

#[functions]
pub trait Set<K, V> {
    type Set;

    fn set(self, t: V) -> Self::Set;
}

#[functions]
pub trait Remove<K> {
    type Remove;

    fn remove(self) -> Self::Remove;
}

impl<T, K> Remove<K> for T
where
    T: Set<K, ()>,
{
    type Remove = T::Set;

    fn remove(self) -> Self::Remove {
        self.set(())
    }
}

macro_rules! impl_get {
    (($($ts:tt)*) => ($p:ident, $i:ident)) => {
        impl<$($ts)*> Get<$p> for ($($ts)*) {
            type Get = $i;

            #[allow(non_snake_case)]
            #[allow(unused_variables)]
            fn get(self) -> Self::Get {
                let ($($ts)*) = self;
                $i
            }
        }
    };
}

impl_get!((A,) => (P0, A));

impl_get!((A, B) => (P0, A));
impl_get!((A, B) => (P1, B));

impl_get!((A, B, C) => (P0, A));
impl_get!((A, B, C) => (P1, B));
impl_get!((A, B, C) => (P2, C));

impl_get!((A, B, C, D) => (P0, A));
impl_get!((A, B, C, D) => (P1, B));
impl_get!((A, B, C, D) => (P2, C));
impl_get!((A, B, C, D) => (P3, D));

impl_get!((A, B, C, D, E) => (P0, A));
impl_get!((A, B, C, D, E) => (P1, B));
impl_get!((A, B, C, D, E) => (P2, C));
impl_get!((A, B, C, D, E) => (P3, D));
impl_get!((A, B, C, D, E) => (P4, E));

impl_get!((A, B, C, D, E, F) => (P0, A));
impl_get!((A, B, C, D, E, F) => (P1, B));
impl_get!((A, B, C, D, E, F) => (P2, C));
impl_get!((A, B, C, D, E, F) => (P3, D));
impl_get!((A, B, C, D, E, F) => (P4, E));
impl_get!((A, B, C, D, E, F) => (P5, F));

#[cfg(test)]
mod test {
    use super::*;

    enum Int {}
    enum Float {}
    enum Char {}
    enum String {}

    #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct Context<A, B, C, D> {
        int: A,
        float: B,
        char: C,
        string: D,
    }

    impl<A, B, C, D> Get<Int> for Context<A, B, C, D> {
        type Get = A;

        fn get(self) -> Self::Get {
            self.int
        }
    }

    impl<A, B, C, D> Get<Float> for Context<A, B, C, D> {
        type Get = B;

        fn get(self) -> Self::Get {
            self.float
        }
    }

    impl<A, B, C, D> Get<Char> for Context<A, B, C, D> {
        type Get = C;

        fn get(self) -> Self::Get {
            self.char
        }
    }

    impl<A, B, C, D> Get<String> for Context<A, B, C, D> {
        type Get = D;

        fn get(self) -> Self::Get {
            self.string
        }
    }

    impl<A, B, C, D, T> Set<Int, T> for Context<A, B, C, D> {
        type Set = Context<T, B, C, D>;

        fn set(self, t: T) -> Self::Set {
            Context {
                int: t,
                float: self.float,
                char: self.char,
                string: self.string,
            }
        }
    }

    impl<A, B, C, D, T> Set<Float, T> for Context<A, B, C, D> {
        type Set = Context<A, T, C, D>;

        fn set(self, t: T) -> Self::Set {
            Context {
                int: self.int,
                float: t,
                char: self.char,
                string: self.string,
            }
        }
    }

    impl<A, B, C, D, T> Set<Char, T> for Context<A, B, C, D> {
        type Set = Context<A, B, T, D>;

        fn set(self, t: T) -> Self::Set {
            Context {
                int: self.int,
                float: self.float,
                char: t,
                string: self.string,
            }
        }
    }

    impl<A, B, C, D, T> Set<String, T> for Context<A, B, C, D> {
        type Set = Context<A, B, C, T>;

        fn set(self, t: T) -> Self::Set {
            Context {
                int: self.int,
                float: self.float,
                char: self.char,
                string: t,
            }
        }
    }

    #[test]
    fn test_map() {
        let context = Context {
            int: 1,
            float: 2.0,
            char: '3',
            string: "4",
        };

        let int = Get::<Int>::get(context);
        let float = Get::<Float>::get(context);
        let char = Get::<Char>::get(context);
        let string = Get::<String>::get(context);

        assert_eq!(int, 1);
        assert_eq!(float, 2.0);
        assert_eq!(char, '3');
        assert_eq!(string, "4");

        let context = Set::<Int, i32>::set(context, 2);
        let context = Set::<Float, _>::set(context, 3.0);
        let context = Set::<Char, _>::set(context, '4');
        let context = Set::<String, _>::set(context, "5");

        assert_eq!(context.int, 2);
        assert_eq!(context.float, 3.0);
        assert_eq!(context.char, '4');
        assert_eq!(context.string, "5");

        let context = Remove::<Int>::remove(context);
        let context = Remove::<Float>::remove(context);
        let context = Remove::<Char>::remove(context);
        let context = Remove::<String>::remove(context);

        assert_eq!(context.int, ());
        assert_eq!(context.float, ());
        assert_eq!(context.char, ());
        assert_eq!(context.string, ());
    }
}
