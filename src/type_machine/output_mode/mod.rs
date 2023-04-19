mod output_none;
mod output_pop_back;
mod output_pop_front;
mod output_push_back;
mod output_push_front;
mod output_remove;
mod output_set;
mod output_sets;

pub use output_none::*;
pub use output_pop_back::*;
pub use output_pop_front::*;
pub use output_push_back::*;
pub use output_push_front::*;
pub use output_remove::*;
pub use output_set::*;
pub use output_sets::*;

/// The operation used to apply the output of an [`Instruction`] to its context.
pub trait OutputMode<C, O, P> {
    type Output;

    fn apply(context: C, output: O) -> Self::Output;
}
