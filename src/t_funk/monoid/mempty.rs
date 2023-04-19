use type_fields_macros::functions;

/// A type that can provide a neutral element.
///
/// To be definition-correct, `Monoid` types must also implement `Semigroup`,
/// but this cannot be strongly modeled without higher-ranked type bounds.
#[functions]
pub trait Mempty {
    type Mempty;

    fn mempty() -> Self::Mempty;
}

