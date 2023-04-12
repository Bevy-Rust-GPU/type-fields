mod endo;
pub use endo::*;

/// A type that can provide a neutral element.
///
/// To be definition-correct, `Monoid` types must also implement `Semigroup`,
/// but this cannot be strongly modeled without higher-ranked type bounds.
pub trait Monoid {
    type Identity;

    fn mempty() -> Self::Identity;
}
