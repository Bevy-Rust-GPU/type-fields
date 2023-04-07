use crate::hlist::tuple::{TupleGets, TupleRef};

use super::instruction::Instruction;

/// Given an instruction, fetch its inputs from `Self`
pub trait InstructionInputs<'a, Inst, Path>: TupleRef {
    type Input;
    fn instruction_inputs(&'a self) -> Self::Input;
}

impl<'a, T, Inst, Path> InstructionInputs<'a, Inst, Path> for T
where
    T: TupleRef + 'a,
    T::TupleRef<'a>: TupleGets<Inst::Input<'a>, Path, Input = Inst::Input<'a>>,
    Inst: Instruction + 'a,
{
    type Input = Inst::Input<'a>;

    fn instruction_inputs(&'a self) -> Self::Input {
        self.tuple_ref().tuple_gets()
    }
}
