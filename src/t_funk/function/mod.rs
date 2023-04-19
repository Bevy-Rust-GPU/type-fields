mod add;
mod constant;
mod div;
mod id;
mod mul;
mod replicate_m;
mod sub;

pub use add::*;
pub use constant::*;
pub use div::*;
pub use id::*;
pub use mul::*;
pub use replicate_m::*;
pub use sub::*;

/// A pure function that takes input and returns output.
///
/// Allows many signatures to be implemented on a single type,
/// unlike Fn* whose implementation encodes a 1:1 coupling.
pub trait Function<Inputs> {
    type Output;

    fn call(input: Inputs) -> Self::Output;
}
