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

#[cfg(test)]
mod test {
    use crate::hlist::type_machine::TypeMachine;

    #[test]
    fn test_type_machine() {
        use crate::{
            functional::{Copointed, Phantom, Pointed},
            hlist::{
                tuple::TupleGet,
                type_machine::{
                    input_mode::{InputGet, InputNone, InputRefGets},
                    instruction::{Instruction, PushBack, Set},
                    output_mode::{OutputNone, OutputSet},
                },
            },
        };

        /// Integer accumulator
        struct Accumulator;

        /// Example position struct
        #[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
        struct Position;

        /// Example velocity struct
        #[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
        struct Velocity;

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
            type Input<'a> = Phantom<Accumulator, usize>;
            type InputMode = InputGet;

            type Output = Phantom<Accumulator, usize>;
            type OutputMode = OutputSet;

            fn exec<'a>(self, input: Self::Input<'a>) -> Self::Output {
                let acc = input.unwrap();
                Phantom::<Accumulator, _>::of(acc + 1)
            }
        }

        /// Force integration `Instruction`
        #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        struct Integrate;

        impl Instruction for Integrate {
            type Input<'a> = (&'a Phantom<Position, f32>, &'a Phantom<Velocity, f32>);
            type InputMode = InputRefGets;

            type Output = Phantom<Position, f32>;
            type OutputMode = OutputSet;

            fn exec<'a>(self, (position, velocity): Self::Input<'a>) -> Self::Output {
                let (position, velocity) = (position.unwrap(), velocity.unwrap());
                Phantom::<Position, _>::of(position + velocity)
            }
        }

        let result = (
            PushBack(Phantom::<Position, _>::of(0.0)),
            PushBack(Phantom::<Velocity, _>::of(1.0)),
            PushBack(Phantom::<Accumulator, _>::of(0)),
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
            Set(Phantom::<Accumulator, _>::of(1)),
        )
            .run(());

        let _pos = result.get::<Phantom<Position, _>>();

        assert_eq!(
            result,
            (
                Phantom::<Position, _>::of(3.0),
                Phantom::<Velocity, _>::of(1.0),
                Phantom::<Accumulator, _>::of(1)
            )
        );
    }
}
