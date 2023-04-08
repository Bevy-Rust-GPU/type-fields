use super::instruction::{InputMode, Instruction};

/// Given an instruction, fetch its inputs from `Self` and execute it
pub trait RunInstruction<Inst, Path> {
    type Output;

    fn run_instruction<'a>(&'a self, instruction: Inst) -> Self::Output
    where
        Inst: 'a;
}

impl<T, Inst, Path> RunInstruction<Inst, Path> for T
where
    Inst: Instruction,
    Inst::InputMode: for<'a> InputMode<&'a T, Inst::Input<'a>, Path>,
{
    type Output = Inst::Output;

    fn run_instruction<'a>(&'a self, instruction: Inst) -> Inst::Output
    where
        Inst: 'a,
    {
        let inputs = <Inst::InputMode as InputMode<&T, Inst::Input<'a>, Path>>::fetch(self);
        instruction.exec(inputs)
    }
}
