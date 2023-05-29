mod inst;
pub use inst::*;

/// A pure function that takes input and returns output.
///
/// Allows many signatures to be implemented on a single type,
/// unlike Fn* whose implementation encodes a 1:1 coupling.
pub trait Function<Inputs> {
    type Output;

    fn call(input: Inputs) -> Self::Output;
}

pub type OutputT<F, T> = <F as Function<T>>::Output;

impl Function<()> for () {
    type Output = ();

    fn call((): ()) -> Self::Output {
        ()
    }
}
