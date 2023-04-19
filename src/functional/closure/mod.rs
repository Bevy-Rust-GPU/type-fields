mod spread;
mod compose;
mod curry;
mod curry_n;
mod flip;

pub use spread::*;
pub use compose::*;
pub use curry::*;
pub use curry_n::*;
pub use flip::*;

use type_fields_macros::functions;

/// A [`Function`] that closes over some external scope via `self` parameter.
#[functions]
pub trait Closure<Input>: Sized {
    type Output;

    fn call(self, input: Input) -> Self::Output;
}
