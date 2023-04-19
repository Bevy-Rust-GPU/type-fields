use crate::t_funk::tlist::TList;

use super::apply_instructions_cons::ApplyInstructionsCons;

/// Given a tuple list of instructions, apply them to `Self`
pub trait ApplyInstructions<Inst, PathGets, PathSets> {
    type AppliedInstructions;

    fn apply_instructions(self, inst: Inst) -> Self::AppliedInstructions;
}

impl<T, Inst, PathGets, PathSets> ApplyInstructions<Inst, PathGets, PathSets> for T
where
    T: ApplyInstructionsCons<Inst::HList, PathGets, PathSets>,
    Inst: TList,
{
    type AppliedInstructions = T::AppliedInstructionsCons;

    fn apply_instructions(self, inst: Inst) -> Self::AppliedInstructions {
        self.apply_instructions_cons(inst.to_hlist())
    }
}
