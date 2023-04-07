use super::{instruction::Instruction, instruction_inputs::InstructionInputs};

/// Given an instruction, fetch its inputs from `Self` and execute it
pub trait RunInstruction<Inst, Path> {
    type Output;

    fn run_instruction<'a>(&'a self, instruction: Inst) -> Self::Output;
}

impl<T, Inst, Path> RunInstruction<Inst, Path> for T
where
    Self: for<'a> InstructionInputs<'a, Inst, Path, Input = Inst::Input<'a>>,
    Inst: Instruction,
{
    type Output = Inst::Output;

    fn run_instruction(&self, instruction: Inst) -> Inst::Output {
        instruction.exec(self.instruction_inputs())
    }
}
