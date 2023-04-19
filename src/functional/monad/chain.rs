use type_fields_macros::functions;

/// A type that can flat-map a function over its wrapped value
///
/// To be definition-correct, `Monad` types must also implement `Applicative`,
/// but this cannot be strongly modeled without higher-ranked type bounds.
#[functions]
pub trait Chain<F> {
    type Chain;

    fn chain(self, f: F) -> Self::Chain;
}

impl<F> Chain<F> for () {
    type Chain = ();

    fn chain(self, _: F) -> Self::Chain {
        self
    }
}

