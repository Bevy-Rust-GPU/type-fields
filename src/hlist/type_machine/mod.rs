use self::apply_instructions::ApplyInstructions;

use super::tuple::TupleList;

pub mod apply_instruction;
pub mod apply_instructions;
pub mod apply_instructions_cons;
pub mod input_mode;
pub mod instruction;
pub mod output_mode;
pub mod run_instruction;

pub trait TypeMachine<Context, PathGets, PathSets>: Sized {
    type Output;

    fn run(self, context: Context) -> Self::Output;
}

impl<Inst, Context, PathGets, PathSets> TypeMachine<Context, PathGets, PathSets> for Inst
where
    Inst: TupleList,
    Context: ApplyInstructions<Inst, PathGets, PathSets>,
{
    type Output = <Context as ApplyInstructions<Inst, PathGets, PathSets>>::AppliedInstructions;

    fn run(self, context: Context) -> Self::Output {
        context.apply_instructions(self)
    }
}

#[test]
fn test_type_machine() {
    use crate::hlist::{
        tuple::TupleGet,
        type_machine::{
            input_mode::{InputGet, InputNone, InputRefGets},
            instruction::{Instruction, PushBack, Set},
            output_mode::{OutputNone, OutputSet},
        },
    };

    /// Integer accumulator
    #[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
    struct Acc(usize);

    /// Example position struct
    #[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
    struct Position(f32);

    /// Example velocity struct
    #[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
    struct Velocity(f32);

    /// Unit instruction with no input or output
    #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct NoOp;

    impl Instruction for NoOp {
        type Input<'a> = ();
        type InputMode = InputNone;

        type Output = ();
        type OutputMode = OutputNone;

        fn exec<'a>(self, _: Self::Input<'a>) -> Self::Output {}
    }

    /// Accumulator increment
    #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct Inc;

    impl Instruction for Inc {
        type Input<'a> = Acc;
        type InputMode = InputGet;

        type Output = Acc;
        type OutputMode = OutputSet;

        fn exec<'a>(self, Acc(input): Self::Input<'a>) -> Self::Output {
            Acc(input + 1)
        }
    }

    /// Force integration `Instruction`
    #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct Integrate;

    impl Instruction for Integrate {
        type Input<'a> = (&'a Position, &'a Velocity);
        type InputMode = InputRefGets;

        type Output = Position;
        type OutputMode = OutputSet;

        fn exec<'a>(self, (position, velocity): Self::Input<'a>) -> Self::Output {
            Position(position.0 + velocity.0)
        }
    }

    let result = (
        PushBack(Position(0.0)),
        PushBack(Velocity(1.0)),
        PushBack(Acc(0)),
        Inc,
        Integrate,
        Inc,
        NoOp,
        Inc,
        Integrate,
        Inc,
        NoOp,
        Inc,
        NoOp,
        Integrate,
        Set(Acc(1)),
    )
        .run(());

    let _pos = result.get::<Position>();

    assert_eq!(result, (Position(3.0), Velocity(1.0), Acc(1)));
}
