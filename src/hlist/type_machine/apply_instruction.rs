use crate::hlist::tuple::TupleSets;

use super::run_instruction::RunInstruction;

/// Given an instruction, run it, and apply the output to `Self`
pub trait ApplyInstruction<Inst, PathGets, PathSets>:
    RunInstruction<Inst, PathGets> + TupleSets<Self::Output, PathSets>
{
    type AppliedInstruction;
    fn apply_instruction(self, inst: Inst) -> Self::AppliedInstruction;
}

impl<T, Inst, PathGets, PathSets> ApplyInstruction<Inst, PathGets, PathSets> for T
where
    T: RunInstruction<Inst, PathGets> + TupleSets<Self::Output, PathSets>,
{
    type AppliedInstruction = Self;

    fn apply_instruction(self, inst: Inst) -> Self::AppliedInstruction {
        let output = self.run_instruction(inst);
        self.tuple_sets(output)
    }
}
