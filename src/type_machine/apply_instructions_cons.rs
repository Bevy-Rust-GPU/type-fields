use crate::t_funk::hlist::{Cons, Nil};

use super::apply_instruction::ApplyInstruction;

/// Given a cons list of instructions, apply them to `Self`
pub trait ApplyInstructionsCons<Inst, PathGets, PathSets> {
    type AppliedInstructionsCons;
    fn apply_instructions_cons(self, inst: Inst) -> Self::AppliedInstructionsCons;
}

impl<T, Head, Tail, GetsHead, GetsTail, SetsHead, SetsTail>
    ApplyInstructionsCons<Cons<Head, Tail>, Cons<GetsHead, GetsTail>, Cons<SetsHead, SetsTail>> for T
where
    T: ApplyInstruction<Head, GetsHead, SetsHead>,
    T::AppliedInstruction: ApplyInstructionsCons<Tail, GetsTail, SetsTail>,
{
    type AppliedInstructionsCons = <T::AppliedInstruction as ApplyInstructionsCons<Tail, GetsTail, SetsTail>>::AppliedInstructionsCons;

    fn apply_instructions_cons(self, Cons(head, tail): Cons<Head, Tail>) -> Self::AppliedInstructionsCons {
        self.apply_instruction(head).apply_instructions_cons(tail)
    }
}

impl<T> ApplyInstructionsCons<Nil, Nil, Nil> for T {
    type AppliedInstructionsCons = T;

    fn apply_instructions_cons(self, _inst: Nil) -> Self::AppliedInstructionsCons {
        self
    }
}
