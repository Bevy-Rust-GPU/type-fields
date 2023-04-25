use crate::macros::{
    category::{Compose, Id},
    Copointed, Pointed,
};

use crate::t_funk::{Closure, Curried, Fmap};

/// A lens over getter `G` and setter `S`
pub type Lens<G, S> = Curried<Lensed<G, Curried<S>>>;

/// Construct a lens given instances of getter `G` and setter `S`
pub const fn lens<G, S>(get: G, set: S) -> Lens<G, S> {
    Curried(Lensed(get, Curried(set)))
}

/// Closure constructing a lens from a getter and setter
#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Pointed,
    Copointed,
    Id,
    Compose,
)]
pub struct Lensed<G, S>(pub G, pub S);

impl<G, S, F, T> Closure<(F, T)> for Lensed<G, S>
where
    T: Clone,
    G: Closure<T>,
    S: Closure<T>,
    F: Closure<G::Output>,
    F::Output: Fmap<S::Output>,
{
    type Output = <F::Output as Fmap<S::Output>>::Fmap;

    fn call(self, (f, t): (F, T)) -> Self::Output {
        f.call(self.0.call(t.clone())).fmap(self.1.call(t))
    }
}

#[cfg(test)]
mod test {
    use crate::macros::Lenses;

    use crate::t_funk::{
        closure::Compose, function::Const, functor::Const as FConst, Closure, Curry, Identity,
    };

    #[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd, Lenses)]
    struct Point {
        x: f64,
        y: f64,
    }

    #[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd, Lenses)]
    struct Atom {
        element: &'static str,
        point: Point,
    }

    #[test]
    fn test_lens() {
        let atom = Atom {
            element: "foo",
            point: Point { x: 0.0, y: 0.0 },
        };

        let point = Point { x: 1.0, y: 2.0 };

        let identity = Atom::point.call(Identity).call(atom);
        let get = Atom::point.call(FConst).call(atom);
        let set = Atom::point
            .call(Identity.compose(Const.prefix(point)))
            .call(atom);

        assert_eq!(identity, Identity(atom));
        assert_eq!(get, FConst(Point { x: 0.0, y: 0.0 }));
        assert_eq!(
            set,
            Identity(Atom {
                element: "foo",
                point
            })
        );
    }

    #[test]
    fn test_lens_composition() {
        let atom = Atom {
            element: "foo",
            point: Point { x: 0.0, y: 0.0 },
        };

        let atom_point_x_lens = Atom::point.compose(Point::x);

        let identity = atom_point_x_lens.call(Identity).call(atom);
        let get = atom_point_x_lens.call(FConst).call(atom);
        let set = atom_point_x_lens
            .call(Identity.compose(Const.prefix(3.0)))
            .call(atom);

        assert_eq!(identity, Identity(atom));
        assert_eq!(get, FConst(0.0));
        assert_eq!(
            set,
            Identity(Atom {
                element: "foo",
                point: Point { x: 3.0, y: 0.0 }
            })
        );
    }
}
