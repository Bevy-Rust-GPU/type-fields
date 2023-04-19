mod inst;

pub use inst::*;

use type_fields_macros::functions;

/// A [`Function`] that closes over some external scope via `self` parameter.
#[functions]
pub trait Closure<Input>: Sized {
    type Output;

    fn call(self, input: Input) -> Self::Output;
}
