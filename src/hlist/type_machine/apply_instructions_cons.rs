use super::apply_instruction::ApplyInstruction;

/// Given a cons list of instructions, apply them to `Self`
pub trait ApplyInstructionsCons<Inst, PathGets, PathSets> {
    type Output;
    fn apply_instructions_cons(self, inst: Inst) -> Self::Output;
}

impl<T, Head, Tail, GetsHead, GetsTail, SetsHead, SetsTail>
    ApplyInstructionsCons<(Head, Tail), (GetsHead, GetsTail), (SetsHead, SetsTail)> for T
where
    T: ApplyInstruction<Head, GetsHead, SetsHead>,
    T::TupleSets: ApplyInstructionsCons<Tail, GetsTail, SetsTail>,
{
    type Output = <T::TupleSets as ApplyInstructionsCons<Tail, GetsTail, SetsTail>>::Output;

    fn apply_instructions_cons(self, (head, tail): (Head, Tail)) -> Self::Output {
        self.apply_instruction(head).apply_instructions_cons(tail)
    }
}

impl<T> ApplyInstructionsCons<(), (), ()> for T {
    type Output = T;

    fn apply_instructions_cons(self, _inst: ()) -> Self::Output {
        self
    }
}
