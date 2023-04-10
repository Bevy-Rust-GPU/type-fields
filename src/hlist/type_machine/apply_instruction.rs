use super::{
    instruction::Instruction, output_mode::OutputMode,
    run_instruction::RunInstruction,
};

/// Given an instruction, run it, and apply the output to `Self`
pub trait ApplyInstruction<Inst, PathGets, PathSets>: RunInstruction<Inst, PathGets> {
    type AppliedInstruction;
    fn apply_instruction(self, inst: Inst) -> Self::AppliedInstruction;
}

impl<T, Inst, PathGets, PathSets> ApplyInstruction<Inst, PathGets, PathSets> for T
where
    T: RunInstruction<Inst, PathGets>,
    Inst: Instruction + OutputMode<T, <T as RunInstruction<Inst, PathGets>>::Output, PathSets>,
{
    type AppliedInstruction =
        <Inst as OutputMode<T, <T as RunInstruction<Inst, PathGets>>::Output, PathSets>>::Output;

    fn apply_instruction(self, inst: Inst) -> Self::AppliedInstruction {
        let output = self.run_instruction(inst);
        <Inst as OutputMode<T, <T as RunInstruction<Inst, PathGets>>::Output, PathSets>>::apply(
            self, output,
        )
    }
}
