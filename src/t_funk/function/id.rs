use type_fields_macros::Closure;

use super::Function;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
pub struct Id;

impl<T> Function<T> for Id {
    type Output = T;

    fn call(input: T) -> Self::Output {
        input
    }
}
