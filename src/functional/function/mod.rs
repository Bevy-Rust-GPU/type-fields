mod add;
mod compose;
mod constant;
mod curry;
mod curry_n;
mod div;
mod flip;
mod id;
mod mul;
mod replicate_m;
mod spread;
mod sub;

pub use add::*;
pub use compose::*;
pub use constant::*;
pub use curry::*;
pub use curry_n::*;
pub use div::*;
pub use flip::*;
pub use id::*;
pub use mul::*;
pub use replicate_m::*;
pub use spread::*;
pub use sub::*;

use crate::derive_closure;

/// A pure function that takes input and returns output.
///
/// Allows many signatures to be implemented on a single type,
/// unlike Fn* whose implementation encodes a 1:1 coupling.
pub trait Function<Inputs> {
    type Output;

    fn call(input: Inputs) -> Self::Output;
}

