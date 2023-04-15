mod add;
mod compose;
mod constant;
mod curry;
mod curry_n;
mod div;
mod flip;
mod id;
mod mul;
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
pub use sub::*;

/// Abstract function trait.
///
/// Allows many signatures to be implemented on a single type,
/// unlike Fn* whose present implementation encodes a 1:1 coupling.
///
/// This allowing mapping over heterogenous lists.
pub trait Function<Inputs> {
    type Output;

    fn call(self, input: Inputs) -> Self::Output;
}

