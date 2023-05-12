pub use type_fields_macros::{functions, Closure, Copointed, Field, Lenses, Pointed};

pub mod applicative {
    pub use type_fields_macros::{Applicative, Apply, Pure};
}

pub mod arrow {
    pub use type_fields_macros::{
        Arrow, Arr, ArrowFirst as First, ArrowSecond as Second, Fanout, Split,
    };
}

pub mod bifunctor {}

pub mod category {
    pub use type_fields_macros::{Category, Compose, Id};
}

pub mod foldable {
    pub use type_fields_macros::{Foldable, Fold, FoldMap, Foldl, Foldr};
}

pub mod functor {
    pub use type_fields_macros::{Functor, Fmap, Replace};
}

pub mod monad {
    pub use type_fields_macros::{Monad, Chain, Then};
}

pub mod monoid {
    pub use type_fields_macros::{Monoid, Mconcat, Mempty};
}

pub mod semigroup {
    pub use type_fields_macros::{Semigroup, Mappend};
}

pub mod traversable {}

/// Convert `do` notation into the equivalent
/// `x.chain(|y| z(y).chain(...))` composition.
///
/// Variable binding via `<-` uses unnameable closures,
/// so is not suitable for use in generic contexts.
#[macro_export]
macro_rules! do_monad {
    ($bind:ident <- $expr:expr; $($tt:tt)*) => {
        $expr.chain(move |$bind| do_monad!($($tt)*))
    };
    ($expr:expr; $($tt:tt)*) => {
        $expr.then(do_monad!($($tt)*))
    };
    ($expr:expr) => {
        $expr
    };
}
