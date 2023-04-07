use crate::hlist::tuple::TupleList;

use super::apply_instructions_cons::ApplyInstructionsCons;

/// Given a tuple list of instructions, apply them to `Self`
pub trait ApplyInstructions<Inst, PathGets, PathSets> {
    type Output;

    fn apply_instructions(self, inst: Inst) -> Self::Output;
}

impl<T, Inst, PathGets, PathSets> ApplyInstructions<Inst, PathGets, PathSets> for T
where
    T: ApplyInstructionsCons<Inst::Cons, PathGets, PathSets>,
    Inst: TupleList,
{
    type Output = T::Output;

    fn apply_instructions(self, inst: Inst) -> Self::Output {
        self.apply_instructions_cons(inst.cons())
    }
}
