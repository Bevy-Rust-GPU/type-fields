use super::Function;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id;

impl<T> Function<T> for Id {
    type Output = T;

    fn call(self, input: T) -> Self::Output {
        input
    }
}

