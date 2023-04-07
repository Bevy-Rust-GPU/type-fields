pub mod apply_instruction;
pub mod apply_instructions;
pub mod apply_instructions_cons;
pub mod instruction;
pub mod instruction_inputs;
pub mod run_instruction;

#[test]
fn test_type_machine() {
    use crate::hlist::type_machine::{
        apply_instructions::ApplyInstructions, instruction::Instruction,
    };

    /// Example position struct
    #[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
    struct Position(f32);

    /// Example velocity struct
    #[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
    struct Velocity(f32);

    /// Force integration `Instruction`
    struct Integrate;

    impl Instruction for Integrate {
        type Input<'a> = (&'a Position, &'a Velocity);
        type Output = (Position,);

        fn exec<'a>(self, (position, velocity): Self::Input<'a>) -> Self::Output {
            (Position(position.0 + velocity.0),)
        }
    }

    /// Debug logging `Instruction`
    struct NoOp;

    impl Instruction for NoOp {
        type Input<'a> = (&'a Position, &'a Velocity);
        type Output = (Position, Velocity);

        fn exec<'a>(self, (pos, vel): Self::Input<'a>) -> Self::Output {
            (*pos, *vel)
        }
    }

    let context = (Position(0.0), Velocity(1.0));
    let instructions = (Integrate, Integrate, NoOp, Integrate, NoOp);
    let context = context.apply_instructions(instructions);
    assert_eq!(context, (Position(3.0), Velocity(1.0)));
}