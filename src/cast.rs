use std::str::FromStr;

pub trait CastFrom<F>: Sized {
    fn cast_from(input: F) -> Option<Self>;
}

pub trait Castable: Sized {
    fn cast_to<T>(self) -> Option<T>
    where
        T: CastFrom<Self>,
    {
        T::cast_from(self)
    }
}
impl<F> Castable for F {}

impl<'a, T> CastFrom<&'a str> for T
where
    T: FromStr,
{
    fn cast_from(input: &str) -> Option<Self> {
        input.parse().ok()
    }
}

impl<'a, T> CastFrom<&'a String> for T
where
    T: FromStr,
{
    fn cast_from(input: &String) -> Option<Self> {
        input.parse().ok()
    }
}
