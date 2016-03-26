use std::num::Wrapping;

pub trait IntoWrapping<T>: Sized {
    fn into_wrapping(self) -> Wrapping<T>;
}

macro_rules! impl_into_wrapping {
    ($T:ty) => (
        impl IntoWrapping<$T> for $T {
            fn into_wrapping(self) -> Wrapping<$T> { Wrapping(self) }
        }
    )
}

impl_into_wrapping!(u8);
impl_into_wrapping!(u16);
impl_into_wrapping!(u32);

impl_into_wrapping!(i8);
impl_into_wrapping!(i16);
impl_into_wrapping!(i32);

impl <T> IntoWrapping<T> for Wrapping<T> {
    fn into_wrapping(self) -> Wrapping<T> { self }
}
