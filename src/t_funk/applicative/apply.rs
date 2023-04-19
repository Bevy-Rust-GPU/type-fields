use type_fields_macros::functions;

/// A type that can take a wrapped function,
/// map it over a provided value, and wrap the result
///
/// To be definition-correct, `Applicative` types must also implement `Functor`,
/// but this cannot be strongly modeled without higher-ranked type bounds.
#[functions]
pub trait Apply<T> {
    type Apply;

    /// <*>
    fn apply(self, a: T) -> Self::Apply;
}

impl<T> Apply<T> for () {
    type Apply = ();

    fn apply(self, _: T) -> Self::Apply {
        ()
    }
}

