//! # <T>-Funk
//! ## (Or <F>-Funk, <G>-Funk, <P>-Funk, Type-Land, etc.)
//!
//! Type-level functional programming toolset for Rust.
//!
//! ## Idioms
//!
//! ### Function
//! ex. fmap, apply, chain
//! A pure (no-self) function. First-order extension class.
//! Encoded via trait with a generic input parameter to allow for heteromorphism and infix notation.
//!
//! ### Closure
//! ex. compose, curry, flip
//! A function that closes over some scope via self parameter. Second-order extension class.
//! First-order function interface for generic code.
//! `Function`s are lifted into `Closure`s via derive.
//!
//! ### Typeclass
//! ex. Functor, Applicative, Monad
//! A collection of [`Function`]s satisfying some laws.
//! Implemented as a blanket trait with a GAT representing each function w.r.t. its output type.
//!
//! ### Newtype
//! ex. Endo, Sum, Product, Any, All
//! A wrapper type that extends some inner type with additional semantics,
//! ex. applying addition or multiplication semantics to numeric association via mappend.
//!
//! ### Laws
//! A set of conditions that validate a given algebra.
//! Currently loose - implemented as test helper code.
//! Should probably be encoded as a tagged trait that can be impl'd over `Typeclass`.
//!
//! ## Limitations
//!
//! Typeclass traits are currently looser than they should be,
//! as implementation of a given `Function` trait can only be asserted
//! with respect to its output type.
//!
//! As is, this positions `Typeclass`es as pleasant interface primitives for concrete code,
//! with strong-typing deferred to the implementation of each `Function`,
//! and trait bounds as a purely informational conceit.
//! i.e. You can use the `Applicative` interface with a type that
//! implements `Apply` but not `Pure`, since `Apply` can't reason about its sibling `Pure` impl.
//!
//! This could be improved with higher-kinded type bounds
//! (i.e. `for<T> U: Apply<T> + Pure<T>`)
//! since it would allow `Typeclass` traits to assert implementation
//! of all their component functions before any method can be called.
//!

pub mod applicative;
pub mod arrow;
pub mod bifunctor;
pub mod category;
pub mod closure;
pub mod copointed;
pub mod foldable;
pub mod function;
pub mod functor;
pub mod list;
pub mod monad;
pub mod monoid;
pub mod pointed;
pub mod semigroup;
pub mod traversable;
pub mod lens;

pub use applicative::*;
pub use arrow::*;
pub use bifunctor::*;
pub use category::*;
pub use closure::*;
pub use copointed::*;
pub use foldable::*;
pub use foldable::*;
pub use function::*;
pub use functor::*;
pub use lens::*;
pub use list::*;
pub use monad::*;
pub use monoid::*;
pub use pointed::*;
pub use semigroup::*;
pub use traversable::*;
