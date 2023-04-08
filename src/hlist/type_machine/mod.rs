
pub mod apply_instruction;
pub mod apply_instructions;
pub mod apply_instructions_cons;
pub mod instruction;
pub mod run_instruction;

#[test]
fn test_register_machine() {
    use crate::hlist::{
        tuple::TuplePushBack,
        type_machine::{
            apply_instructions::ApplyInstructions,
            instruction::{InputNone, InputRefGets, Instruction, OutputNone, OutputSet, InputRefGet, InputGet},
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
        type InputMode = InputRefGets;
        type Input<'a> = (&'a Position, &'a Velocity);
        type Output = Position;
        type OutputMode = OutputSet;

        fn exec<'a>(self, (position, velocity): Self::Input<'a>) -> Self::Output {
            Position(position.0 + velocity.0)
        }
    }

    let context =
        ().tuple_push_back(Position(0.0))
            .tuple_push_back(Velocity(1.0))
            .tuple_push_back(Acc(0));

    let instructions: (
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
    ) = Default::default();

    let context = context.apply_instructions(instructions);
    assert_eq!(context, (Position(3.0), Velocity(1.0), Acc(5)));
}
