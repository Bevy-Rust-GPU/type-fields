mod add;
mod compose;
mod constant;
mod curry;
mod div;
mod id;
mod mul;
mod sub;
mod flip;

pub use add::*;
pub use compose::*;
pub use constant::*;
pub use curry::*;
pub use div::*;
pub use id::*;
pub use mul::*;
pub use sub::*;
pub use flip::*;

/// Abstract function trait.
///
/// Allows many signatures to be implemented on a single type,
/// unlike Fn* whose present implementation encodes a 1:1 coupling.
///
/// This allowing mapping over heterogenous lists.
pub trait Function<Inputs, Generics = ()> {
    type Output;

    fn call(self, input: Inputs) -> Self::Output;
}

