//! Type-level field access.

#![no_std]

extern crate self as type_fields;

pub mod cons;
pub mod field;
pub mod path;

#[cfg(test)]
pub mod tests {
    use crate::{cons::Cons, field::Field};
    pub use type_fields_macros::Field;

    #[derive(Default, Field)]
    struct Wrapper<Sdf> {
        pub sdf: Sdf,
    }

    #[derive(Default, Field)]
    struct Leaf {
        pub radius: f32,
    }

    #[test]
    fn test_parameters() {
        let sdf = Wrapper::<()>::SDF;
        let radius = Leaf::RADIUS;

        Wrapper::<Leaf>::default().with(sdf, Leaf { radius: 1.0 });
        Wrapper::<Wrapper<Leaf>>::default().with((sdf, sdf).cons(), Leaf { radius: 1.0 });
        Wrapper::<Wrapper<Leaf>>::default().with((sdf, sdf, radius).cons(), 1.0);
    }
}
