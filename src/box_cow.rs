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


#[derive(Debug, Clone)]
pub enum BoxCow<'l, B>
where
    B : Clone + ?Sized + 'l
{
    Borrowed(&'l B),
    Owned(Box<B>)
}


impl<'l, B> BoxCow<'l, B>
where
    B : Clone + ?Sized + 'l
{

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
            T      : Clone + ?Sized,
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
    B : Clone + ?Sized + 'l
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
            B     : Clone + ?Sized,
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
    B      : Clone + ?Sized,
    Box<B> : Deser<'de>
{
    #[inline]
    fn deserialize<D>(deserer : D) -> Result<Self, D::Error>
    where
        D : Deserer<'de>
    { Ok(Self::Owned(Box::<B>::deserialize(deserer)?)) }
}
