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

impl Monoid for () {
    type Identity = ();

    fn mempty() -> Self::Identity {
        ()
    }
}

impl Monoid for u32 {
    type Identity = Self;

    fn mempty() -> Self::Identity {
        0
    }
}

impl Monoid for i32 {
    type Identity = Self;

    fn mempty() -> Self::Identity {
        0
    }
}

impl Monoid for f32 {
    type Identity = Self;

    fn mempty() -> Self::Identity {
        0.0
    }
}

impl Monoid for &str {
    type Identity = Self;

    fn mempty() -> Self::Identity {
        ""
    }
}
