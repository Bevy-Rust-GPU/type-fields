extern crate std;

use core::fmt::{Debug, Display};
use std::{format, println, string::String};

use crate::{macros::Closure, t_funk::Function};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
pub struct PrintLn;

impl<T> Function<T> for PrintLn
where
    T: Display,
{
    type Output = ();

    fn call(input: T) -> Self::Output {
        println!("{input:}");
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
pub struct DisplayF;

impl<T> Function<T> for DisplayF
where
    T: Display,
{
    type Output = String;

    fn call(input: T) -> Self::Output {
        format!("{input:}")
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
pub struct DebugF;

impl<T> Function<T> for DebugF
where
    T: Debug,
{
    type Output = String;

    fn call(input: T) -> Self::Output {
        format!("{input:?}")
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
pub struct DebugMultilineF;

impl<T> Function<T> for DebugMultilineF
where
    T: Debug,
{
    type Output = String;

    fn call(input: T) -> Self::Output {
        format!("{input:#?}")
    }
}
