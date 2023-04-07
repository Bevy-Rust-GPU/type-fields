use super::apply_instruction::ApplyInstruction;

/// Given a cons list of instructions, apply them to `Self`
pub trait ApplyInstructionsCons<Inst, PathGets, PathSets> {
    type AppliedInstructionsCons;
    fn apply_instructions_cons(self, inst: Inst) -> Self::AppliedInstructionsCons;
}

impl<T, Head, Tail, GetsHead, GetsTail, SetsHead, SetsTail>
    ApplyInstructionsCons<(Head, Tail), (GetsHead, GetsTail), (SetsHead, SetsTail)> for T
where
    T: ApplyInstruction<Head, GetsHead, SetsHead>,
    T::AppliedInstruction: ApplyInstructionsCons<Tail, GetsTail, SetsTail>,
{
    type AppliedInstructionsCons = <T::AppliedInstruction as ApplyInstructionsCons<Tail, GetsTail, SetsTail>>::AppliedInstructionsCons;

    fn apply_instructions_cons(self, (head, tail): (Head, Tail)) -> Self::AppliedInstructionsCons {
        self.apply_instruction(head).apply_instructions_cons(tail)
    }
}

impl<T> ApplyInstructionsCons<(), (), ()> for T {
    type AppliedInstructionsCons = T;

    fn apply_instructions_cons(self, _inst: ()) -> Self::AppliedInstructionsCons {
        self
    }
}
