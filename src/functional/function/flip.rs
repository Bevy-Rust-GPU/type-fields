use crate::{derive_copointed, derive_pointed};

use super::Function;

pub struct Flipped<F>(F);

derive_pointed!(Flipped<F>);
derive_copointed!(Flipped<F>);

impl<F, A, B> Function<(B, A)> for Flipped<F>
where
    F: Function<(A, B)>,
{
    type Output = F::Output;

    fn call(self, (b, a): (B, A)) -> Self::Output {
        self.0.call((a, b))
    }
}

