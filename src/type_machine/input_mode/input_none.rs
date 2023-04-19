use crate::type_machine::input_mode::InputMode;

/// Fetch no input
pub struct InputNone;

impl<C> InputMode<C, (), ()> for InputNone {
    fn fetch(_: C) -> () {
        ()
    }
}

