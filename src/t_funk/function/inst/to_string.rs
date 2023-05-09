use crate::{macros::Closure, t_funk::Function};

extern crate alloc;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
pub struct ToString;

impl<T> Function<T> for ToString
where
    T: alloc::string::ToString,
{
    type Output = alloc::string::String;

    fn call(input: T) -> Self::Output {
        input.to_string()
    }
}

