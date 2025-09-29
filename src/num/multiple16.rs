//! Integer which must be a multiple of 16.


use core::fmt::{ self, Debug, Display, Formatter };
#[cfg(doc)]
use core::hint::unreachable_unchecked;
use serde::{
    Serialize as Ser,
    Serializer as Serer,
    Deserialize as Deser,
    Deserializer as Deserer
};
use syndebug::SynDebug;
use disqualified::ShortName;


/// An integer which must be a multiple of 16.
#[repr(transparent)]
pub struct Multiple16<T>(T)
where
    T : Multiple16ablePrimitive;

#[cfg(doc)]
impl<T> Multiple16<T>
where
    T : Multiple16ablePrimitive
{

    /// Creates a new [`Multiple16`] if the given value is a multiple of 16.
    pub const fn new(n : T) -> Option<Self> { unsafe { unreachable_unchecked() } }

    /// Creates a new [`Multiple16`] without checking that the given value is a multiple of 16.
    ///
    /// ### Safety
    /// The given value must be a multiple of 16.
    pub const unsafe fn new_unchecked(n : T) -> Self { unsafe { unreachable_unchecked() } }

    /// Returns the contained value as a primitive type.
    pub const fn get(self) -> T { unsafe { unreachable_unchecked() } }

}

impl<T> Debug for Multiple16<T>
where
    T : Multiple16ablePrimitive
{
    #[inline]
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result {
        <T as Debug>::fmt(&self.0, f)
    }
}

impl<T> Display for Multiple16<T>
where
    T : Multiple16ablePrimitive
{
    #[inline]
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result {
        <T as Display>::fmt(&self.0, f)
    }
}

impl<T> SynDebug for Multiple16<T>
where
    T : Multiple16ablePrimitive
{
    #[inline]
    fn fmt(&self, f : &mut Formatter<'_>, _const_like : bool) -> fmt::Result {
        write!(f, "Multiple16::<{}>::new({:?}).unwrap()", ShortName::of::<T>(), self.0)
    }
}

impl<T> Ser for Multiple16<T>
where
    T : Multiple16ablePrimitive
{
    #[inline]
    fn serialize<S>(&self, serer : S) -> Result<S::Ok, S::Error>
    where
        S : Serer
    { <T as Ser>::serialize(&self.0, serer) }
}

impl<'de, T> Deser<'de> for Multiple16<T>
where
    T : Multiple16ablePrimitive
{
    #[inline]
    fn deserialize<D>(deserer : D) -> Result<Self, D::Error>
    where
        D : Deserer<'de>
    { Ok(Self(<T as Deser>::deserialize(deserer)?)) }
}



/// An integer type which can be used in [`Multiple16`].
#[allow(private_bounds)]
pub trait Multiple16ablePrimitive
where
    for<'de> Self : Copy + Ser + Deser<'de> + Debug + Display + Sized + Sealed
{ }
trait Sealed { }

macro_rules! impl_multiple_16able_primitive_for {
    ($ty:ty, $ident:ident $(,)?) => {
        #[doc = concat!("A [`Multiple16`] with inner type ", stringify!($ty), ".")]
        pub type $ident = Multiple16<$ty>;

        impl Multiple16ablePrimitive for $ty { }

        #[doc(hidden)]
        impl $ident {

            #[inline]
            pub const fn new(n : $ty) -> Option<Self> {
                if (n.rem_euclid(16) == 0) {
                    Some(Self(n))
                } else { None }
            }

            #[inline]
            pub const unsafe fn new_unchecked(n : $ty) -> Self {
                Self(n)
            }

            #[inline]
            pub const fn get(self) -> $ty { self.0 }

        }

        impl Sealed for $ty { }
    }
}
impl_multiple_16able_primitive_for!(u8, Multiple16U8);
impl_multiple_16able_primitive_for!(i8, Multiple16I8);
impl_multiple_16able_primitive_for!(u16, Multiple16U16);
impl_multiple_16able_primitive_for!(i16, Multiple16I16);
impl_multiple_16able_primitive_for!(u32, Multiple16U32);
impl_multiple_16able_primitive_for!(i32, Multiple16I32);
impl_multiple_16able_primitive_for!(u64, Multiple16U64);
impl_multiple_16able_primitive_for!(i64, Multiple16I64);
impl_multiple_16able_primitive_for!(u128, Multiple16U128);
impl_multiple_16able_primitive_for!(i128, Multiple16I128);
