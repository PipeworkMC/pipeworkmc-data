//! [`Cow`](std::borrow::Cow)-like, but within a [`Box`].


use core::{
    fmt::{ self, Formatter },
    hint::unreachable_unchecked,
    ops::Deref
};
use serde::{
    Serialize as Ser,
    Serializer as Serer,
    Deserialize as Deser,
    Deserializer as Deserer
};
use syndebug::SynDebug;


/// Similar to [`Cow`](std::borrow::Cow), but within a [`Box`].
#[derive(Debug, Clone)]
pub enum BoxCow<'l, B, O = B>
where
    B : ?Sized,
    O : ?Sized
{
    /// Borrowed data.
    Borrowed(&'l B),
    /// Owned data.
    Owned(Box<O>)
}


impl<'l, B> BoxCow<'l, B>
where
    B : Clone + 'l
{

    /// Acquires a mutable reference to the owned form of the data.
    ///
    /// Clones the data if it is not already owned.
    pub fn to_mut(&mut self) -> &mut B {
        match (self) {
            Self::Borrowed(b)  => {
                *self = Self::Owned(Box::new(b.clone()));
                let Self::Owned(o) = self
                    else { unsafe { unreachable_unchecked() } };
                o
            },
            Self::Owned(o) => o,
        }
    }

    /// Extracts the owned data.
    ///
    /// Clones the data if it is not already owned.
    #[inline]
    pub fn into_owned(self) -> Box<B> {
        match (self) {
            Self::Borrowed(b) => Box::new(b.clone()),
            Self::Owned(o)    => o
        }
    }

}


impl<T> SynDebug for BoxCow<'_, T>
where
            T      : Clone,
    for<'l> &'l T  : SynDebug,
            Box<T> : SynDebug
{
    fn fmt(&self, f : &mut Formatter<'_>, const_like : bool) -> fmt::Result {
        if (const_like) {
            write!(f, "BoxCow::Borrowed( ")?;
            <&T as SynDebug>::fmt(&&**self, f, true)?;
            write!(f, ", )")?;
            Ok(())
        } else {
            match (self) {
                Self::Borrowed(inner) => {
                    write!(f, "BoxCow::Borrowed( ")?;
                    <&T as SynDebug>::fmt(inner, f, false)?;
                    write!(f, ", )")?;
                    Ok(())
                },
                Self::Owned(inner) => {
                    write!(f, "BoxCow::Owned( ")?;
                    <Box<T> as SynDebug>::fmt(inner, f, false)?;
                    write!(f, ", )")?;
                    Ok(())
                }
            }
        }
    }
}


impl<'l, B> Deref for BoxCow<'l, B>
where
    B : Clone + 'l
{
    type Target = B;

    #[inline]
    fn deref(&self) -> &Self::Target {
        match (self) {
            Self::Borrowed(b) => b,
            Self::Owned(o)    => o
        }
    }
}


impl<'l, B> Ser for BoxCow<'l, B>
where
            B     : Clone,
    for<'k> &'k B : Ser
{
    #[inline]
    fn serialize<S>(&self, serer : S) -> Result<S::Ok, S::Error>
    where
        S : Serer
    { self.deref().serialize(serer) }
}

impl<'de, 'l, B> Deser<'de> for BoxCow<'l, B>
where
    B      : Clone,
    Box<B> : Deser<'de>
{
    #[inline]
    fn deserialize<D>(deserer : D) -> Result<Self, D::Error>
    where
        D : Deserer<'de>
    { Ok(Self::Owned(Box::<B>::deserialize(deserer)?)) }
}
