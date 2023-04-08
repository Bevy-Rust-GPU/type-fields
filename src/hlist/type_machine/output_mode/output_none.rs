use super::OutputMode;

/// Discard the output and do not modify the context.
pub struct OutputNone;

impl<C, O> OutputMode<C, O, ()> for OutputNone {
    type Output = C;

    fn apply(context: C, _: O) -> Self::Output {
        context
    }
}

