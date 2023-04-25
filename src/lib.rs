//! Type-level field access.

#![no_std]

extern crate self as type_fields;

pub mod macros {
    pub use type_fields_macros::{functions, Closure, Copointed, Field, Lenses, Pointed};

    pub mod applicative {
        pub use type_fields_macros::{Apply, Pure};
    }

    pub mod arrow {
        pub use type_fields_macros::{
            Arr, ArrowFirst as First, ArrowSecond as Second, Fanout, Split,
        };
    }

    pub mod bifunctor {}

    pub mod category {
        pub use type_fields_macros::{Compose, Id};
    }

    pub mod foldable {
        pub use type_fields_macros::{Fold, FoldMap, Foldl, Foldr};
    }

    pub mod functor {
        pub use type_fields_macros::{Fmap, Replace};
    }

    pub mod monad {
        pub use type_fields_macros::{Chain, Then};
    }

    pub mod monoid {
        pub use type_fields_macros::{Mconcat, Mempty};
    }

    pub mod semigroup {
        pub use type_fields_macros::Mappend;
    }

    pub mod traversable {}
}

pub mod field;
pub mod path;
pub mod t_funk;
pub mod type_machine;

#[cfg(test)]
pub mod tests {
    use crate::{field::Field, macros::Field};

    #[derive(Default, Field)]
    struct Wrapper<Sdf> {
        pub sdf: Sdf,
    }

    #[derive(Default, Field)]
    struct Leaf {
        pub radius: f32,
    }

    #[test]
    fn test_parameters() {
        Wrapper::<Leaf>::default().with(Wrapper::sdf, Leaf { radius: 1.0 });
        Wrapper::<Wrapper<Leaf>>::default()
            .with((Wrapper::sdf, Wrapper::sdf), Leaf { radius: 1.0 });
        Wrapper::<Wrapper<Leaf>>::default().with((Wrapper::sdf, Wrapper::sdf, Leaf::radius), 1.0);
    }
}
