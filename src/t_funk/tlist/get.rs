use crate::t_funk::hlist::Get as HListGet;

use super::TList;

pub trait GetImpl<T, P>: TList {
    fn get_impl(self) -> T;
}

impl<T, P, In> GetImpl<In, P> for T
where
    T: TList,
    T::HList: HListGet<In, P>,
{
    fn get_impl(self) -> In {
        self.to_hlist().get()
    }
}

pub trait Get<P>: TList {
    fn get<T>(self) -> T
    where
        Self: GetImpl<T, P>;
}

impl<T, P> Get<P> for T
where
    T: TList,
{
    fn get<In>(self) -> In
    where
        T: GetImpl<In, P>,
    {
        self.get_impl()
    }
}

#[cfg(test)]
mod tests {
    use crate::t_funk::tlist::GetImpl;

    #[test]
    fn test_tuple_get() {
        let list = (1, 2.0, "three");
        let int: usize = list.get_impl();
        let float: f32 = list.get_impl();
        let string: &str = list.get_impl();
        assert_eq!((1, 2.0, "three"), (int, float, string));
    }
}
