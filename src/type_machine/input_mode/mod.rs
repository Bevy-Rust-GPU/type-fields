mod input_get;
mod input_gets;
mod input_none;
mod input_ref_get;
mod input_ref_gets;

pub use input_get::*;
pub use input_gets::*;
pub use input_none::*;
pub use input_ref_get::*;
pub use input_ref_gets::*;

/// The operation used to fetch an [`Instruction`]'s inputs from its context.
pub trait InputMode<C, I, P> {
    fn fetch(context: C) -> I;
}
