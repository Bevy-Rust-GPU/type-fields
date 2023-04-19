use super::HList;

pub trait Append<T>: HList {
    type Appended;

    fn append(self, t: T) -> Self::Appended;
}

impl<Head, Tail, T> Append<T> for (Head, Tail)
where
    Self: HList,
    Tail: Append<T>,
{
    type Appended = (Head, Tail::Appended);

    fn append(self, t: T) -> Self::Appended {
        (self.0, self.1.append(t))
    }
}

impl<T> Append<T> for ()
where
    Self: HList,
    T: HList,
{
    type Appended = T;

    fn append(self, t: T) -> Self::Appended {
        t
    }
}

#[cfg(test)]
mod test {
    use crate::t_funk::{
        hlist::{Append, ToTList},
        tlist::ToHList,
    };

    #[test]
    fn test_cons_append() {
        let list_a = (1, 2, 3).to_hlist();
        let list_b = (4, 5, 6).to_hlist();
        let list_c = list_a.append(list_b);
        assert_eq!(list_c.to_tlist(), (1, 2, 3, 4, 5, 6));
    }
}