use self::apply_instructions::ApplyInstructions;

use crate::t_funk::tlist::TList;

pub mod action;
pub mod apply_instruction;
pub mod apply_instructions;
pub mod apply_instructions_cons;
pub mod input;
pub mod input_mode;
pub mod instruction;
pub mod output;
pub mod output_mode;
pub mod run_instruction;

pub trait TypeMachine<Context, PathGets, PathSets>: Sized {
    type Output;

    fn run(self, context: Context) -> Self::Output;
}

impl<Inst, Context, PathGets, PathSets> TypeMachine<Context, PathGets, PathSets> for Inst
where
    Inst: TList,
    Context: ApplyInstructions<Inst, PathGets, PathSets>,
{
    type Output = <Context as ApplyInstructions<Inst, PathGets, PathSets>>::AppliedInstructions;

    fn run(self, context: Context) -> Self::Output {
        context.apply_instructions(self)
    }
}

#[allow(non_snake_case)]
pub fn Do<Inst, PathGets, PathSets>(
    inst: Inst,
) -> <() as ApplyInstructions<Inst, PathGets, PathSets>>::AppliedInstructions
where
    Inst: TList,
    (): ApplyInstructions<Inst, PathGets, PathSets>,
{
    ().apply_instructions(inst)
}

#[cfg(test)]
mod test {
    use crate::{
        t_funk::{tlist::Map, CopointF, Copointed, Pointed, Tagged},
        type_machine::{
            input::{GetOf, InputOf},
            input_mode::InputGets,
            instruction::{Const, Instruction, NoOp},
            output::{DefOf, SetOf},
            Do,
        },
    };

    #[test]
    fn test_type_machine() {
        /// Accumulator increment
        #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        struct Increment;

        impl Instruction for Increment {
            type Input<'a> = usize;
            type Output = usize;

            fn exec<'a>(self, input: Self::Input<'a>) -> Self::Output {
                input + 1
            }
        }

        /// Force integration `Instruction`
        #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        struct Integrate;

        impl Instruction for Integrate {
            type Input<'a> = (Tagged<Pos, f32>, Tagged<Vel, f32>);
            type Output = f32;

            fn exec<'a>(self, (position, velocity): Self::Input<'a>) -> Self::Output {
                position.copoint() + velocity.copoint()
            }
        }

        enum Acc {}
        enum Pos {}
        enum Vel {}

        let ctx = Do((
            Pos::def() << Const(0.0),
            Vel::def() << Const(1.0),
            Acc::def() << Const(0),
            NoOp,
            Acc::set() << Increment << Acc::get(),
            Pos::set() << Integrate << InputGets::<(Pos, Vel)>::input(),
            Acc::set() << Increment << Acc::get(),
            NoOp,
            Acc::set() << Increment << Acc::get(),
            Pos::set() << Integrate << InputGets::<(Pos, Vel)>::input(),
            Acc::set() << Increment << Acc::get(),
            NoOp,
            Acc::set() << Increment << Acc::get(),
            Pos::set() << Integrate << InputGets::<(Pos, Vel)>::input(),
            NoOp,
            Acc::set() << Const(1),
        ));

        assert_eq!(ctx.map(CopointF), (3.0, 1.0, 1));
    }
}
