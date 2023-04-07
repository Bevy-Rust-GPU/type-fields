//! Traits implemented over types that can be converted into a `ConsList`.

mod cons;
mod cons_mut;
mod cons_ref;
mod tuple_get;
mod tuple_gets;
mod tuple_length;
mod tuple_list;
mod tuple_list_mut;
mod tuple_list_ref;
mod tuple_mut;
mod tuple_push_back;
mod tuple_push_front;
mod tuple_ref;
mod tuple_set;
mod tuple_sets;

pub use cons::*;
pub use cons_mut::*;
pub use cons_ref::*;
pub use tuple_get::*;
pub use tuple_gets::*;
pub use tuple_length::*;
pub use tuple_list::*;
pub use tuple_mut::*;
pub use tuple_push_back::*;
pub use tuple_push_front::*;
pub use tuple_ref::*;
pub use tuple_set::*;
pub use tuple_sets::*;

mod type_machine {
    use super::{TupleGets, TupleList, TupleSets};

    trait Instruction {
        type Input: TupleList;
        type Output: TupleList;

        fn exec(self, input: Self::Input) -> Self::Output;
    }

    impl<In, Out> Instruction for fn(In) -> Out
    where
        In: TupleList,
        Out: TupleList,
    {
        type Input = In;
        type Output = Out;

        fn exec(self, input: Self::Input) -> Self::Output {
            self(input)
        }
    }

    #[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
    struct Position(f32);

    #[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
    struct Velocity(f32);

    /// Force integration `Instruction`
    struct Integrate;

    impl Instruction for Integrate {
        type Input = (Position, Velocity);
        type Output = (Position,);

        fn exec(self, (position, velocity): Self::Input) -> Self::Output {
            (Position(position.0 + velocity.0),)
        }
    }

    /// Debug logging `Instruction`
    struct NoOp;

    impl Instruction for NoOp {
        type Input = (Position, Velocity);
        type Output = (Position, Velocity);

        fn exec(self, (pos, vel): Self::Input) -> Self::Output {
            (pos, vel)
        }
    }

    /// Given an instruction, fetch its inputs from `Self`
    trait InstructionGets<Inst, Path>: TupleGets<Self::Input, Path> {
        type Input;
        fn instruction_gets(self) -> Self::Input;
    }

    impl<T, Inst, Path> InstructionGets<Inst, Path> for T
    where
        T: TupleGets<Inst::Input, Path>,
        Inst: Instruction,
    {
        type Input = Inst::Input;

        fn instruction_gets(self) -> Self::Input {
            self.tuple_gets()
        }
    }

    /// Given an instruction, fetch its inputs from `Self` and execute it
    trait RunInstruction<Inst, Path>: InstructionGets<Inst, Path> {
        type Output;

        fn run_instruction(self, instruction: Inst) -> Self::Output;
    }

    impl<T, Inst, Path> RunInstruction<Inst, Path> for T
    where
        Self: InstructionGets<Inst, Path, Input = Inst::Input>,
        Inst: Instruction,
    {
        type Output = Inst::Output;

        fn run_instruction(self, instruction: Inst) -> Inst::Output {
            instruction.exec(self.instruction_gets())
        }
    }

    /// Given an instruction, run it, and apply the output to `Self`
    trait ApplyInstruction<Inst, PathGets, PathSets>:
        RunInstruction<Inst, PathGets> + TupleSets<Self::Output, PathSets>
    {
        fn apply_instruction(self, inst: Inst) -> Self;
    }

    impl<T, Inst, PathGets, PathSets> ApplyInstruction<Inst, PathGets, PathSets> for T
    where
        T: Clone + RunInstruction<Inst, PathGets> + TupleSets<Self::Output, PathSets>,
    {
        fn apply_instruction(self, inst: Inst) -> Self {
            let output = self.clone().run_instruction(inst);
            self.tuple_sets(output)
        }
    }

    /// Given a cons list of instructions, apply them to `Self`
    trait ApplyInstructionsCons<Inst, PathGets, PathSets> {
        fn apply_instructions_cons(self, inst: Inst) -> Self;
    }

    impl<T, Head, Tail, GetsHead, GetsTail, SetsHead, SetsTail>
        ApplyInstructionsCons<(Head, Tail), (GetsHead, GetsTail), (SetsHead, SetsTail)> for T
    where
        T: Clone
            + ApplyInstruction<Head, GetsHead, SetsHead>
            + ApplyInstructionsCons<Tail, GetsTail, SetsTail>,
    {
        fn apply_instructions_cons(self, (head, tail): (Head, Tail)) -> Self {
            self.apply_instruction(head).apply_instructions_cons(tail)
        }
    }

    impl<T> ApplyInstructionsCons<(), (), ()> for T {
        fn apply_instructions_cons(self, _inst: ()) -> Self {
            self
        }
    }

    /// Given a tuple list of instructions, apply them to `Self`
    trait ApplyInstructions<Inst, PathGets, PathSets> {
        fn apply_instructions(self, inst: Inst) -> Self;
    }

    impl<T, Inst, PathGets, PathSets> ApplyInstructions<Inst, PathGets, PathSets> for T
    where
        T: ApplyInstructionsCons<Inst::Cons, PathGets, PathSets>,
        Inst: TupleList,
    {
        fn apply_instructions(self, inst: Inst) -> Self {
            self.apply_instructions_cons(inst.cons())
        }
    }

    #[test]
    fn test_type_machine() {
        let context = (Position(0.0), Velocity(1.0));
        let instructions = (Integrate, Integrate, NoOp, Integrate, NoOp);
        let context = context.apply_instructions(instructions);
        assert_eq!(context, (Position(3.0), Velocity(1.0)));
    }
}
