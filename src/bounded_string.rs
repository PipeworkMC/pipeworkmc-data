//! No-alloc string with maximum length.


use crate::Minecraft;
use core::{
    error::Error as StdError,
    fmt::{ self, Debug, Display, Formatter },
    ops::Deref,
    ptr
};
use netzer::prelude::*;
use serde::{
    ser::{
        Serialize as Ser,
        Serializer as Serer
    },
    de::{
        Deserialize as Deser,
        Deserializer as Deserer,
        Error as _
    }
};


/// A no-alloc [`String`]-like data structure with a maximum length.
#[derive(Clone)]
pub struct BoundedString<const MAX_LEN : usize> {
    data : [u8; MAX_LEN],
    len  : usize
}

impl<const MAX_LEN : usize> BoundedString<MAX_LEN> {
    /// Returns a new [`BoundedString`] without checking its length.
    ///
    /// ### Safety
    /// The caller is responsible for ensuring that the given [`&str`](str) is not longer than `MAX_LEN`.
    pub unsafe fn new_unchecked(s : &str) -> Self {
        let mut data = [0u8; MAX_LEN];
        unsafe { ptr::copy_nonoverlapping(s.as_ptr(), data.as_mut_ptr(), s.len()); }
        Self { data, len : s.len() }
    }
}

/// The string was too long.
#[derive(Debug)]
pub struct BoundedStringTooLong(pub usize);
impl StdError for BoundedStringTooLong { }
impl Display for BoundedStringTooLong {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result {
        write!(f, "leb128 too long")
    }
}

impl<const MAX_LEN : usize> NetEncode<Minecraft> for BoundedString<MAX_LEN> {
    async fn encode<W : netzer::AsyncWrite>(&self, w : W) -> netzer::Result {
        <str as NetEncode<Minecraft>>::encode(self, w).await
    }
}

impl<const MAX_LEN : usize> NetDecode<Minecraft> for BoundedString<MAX_LEN> {
    async fn decode<R : netzer::AsyncRead>(mut r : R) -> netzer::Result<Self> {
        let len = usize::try_from(<VarInt<u32> as NetDecode<Minecraft>>::decode(&mut r).await?.0)?;
        if (len > MAX_LEN) {
            return Err(BoundedStringTooLong(len).into());
        }
        let mut out = BoundedString { data : [0u8; MAX_LEN], len };
        let     buf = &mut out.data[..len];
        r.read_exact(buf).await?;
        _ = str::from_utf8(buf)?; // Check that the string is valid UTF8.
        Ok(out)
    }
}


impl<const MAX_LEN : usize> TryFrom<&str> for BoundedString<MAX_LEN> {
    type Error = BoundedStringTooLong;
    fn try_from(value : &str) -> Result<Self, Self::Error> {
        if (value.len() > MAX_LEN) {
            Err(BoundedStringTooLong(value.len()))
        } else {
            let mut bytes = [0u8; MAX_LEN];
            unsafe { ptr::copy_nonoverlapping(
                value.as_ptr(),
                bytes.as_mut_ptr(),
                value.len()
            ); }
            Ok(Self {
                data : bytes,
                len  : value.len()
            })
        }
    }
}

impl<const MAX_LEN : usize> Ser for BoundedString<MAX_LEN> {
    fn serialize<S>(&self, serer : S) -> Result<S::Ok, S::Error>
    where
        S : Serer
    { <&str as Ser>::serialize(&&**self, serer) }
}

impl<'de, const MAX_LEN : usize> Deser<'de> for BoundedString<MAX_LEN> {
    fn deserialize<D>(deserer : D) -> Result<Self, D::Error>
    where
        D : Deserer<'de>
    {
        Self::try_from(<&str>::deserialize(deserer)?)
            .map_err(D::Error::custom)
    }
}


impl<const MAX_LEN : usize> Deref for BoundedString<MAX_LEN> {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        // SAFETY: `self.len` is always less than `self.data.len()`.
        unsafe { str::from_utf8_unchecked(&self.data[0..self.len]) }
    }
}

impl<const MAX_LEN : usize> Debug for BoundedString<MAX_LEN> {
    #[inline]
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(self as &str, f)
    }
}
impl<const MAX_LEN : usize> Display for BoundedString<MAX_LEN> {
    #[inline]
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self as &str, f)
    }
}
