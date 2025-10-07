use super::tag;
use core::{
    error::Error as StdError,
    fmt::{ self, Display, Formatter }
};
use std::io::{ self, Read };
use serde::de::{
    Error as DeserError,
    Deserialize as Deser
};
use cesu8::Cesu8DecodingError;


mod root;
use root::{ NbtRootDeserer, TagRead };


/// Errors emitted while deserialising NBT as a value.
#[derive(Debug)]
pub enum NbtDeserError {
    /// An io error occured.
    Io(io::Error),
    /// A value deserialiser returned a custom error.
    Custom(String),

    /// An unknown NBT element tag was found.
    UnknownTag(u8),
    /// A badly encoded string element was found.
    BadString(Cesu8DecodingError),
    /// A badly encoded char element was found.
    BadChar
}
impl From<io::Error> for NbtDeserError {
    #[inline]
    fn from(err : io::Error) -> Self {
        Self::Io(err)
    }
}
impl From<Cesu8DecodingError> for NbtDeserError {
    #[inline]
    fn from(err : Cesu8DecodingError) -> Self {
        Self::BadString(err)
    }
}

impl Display for NbtDeserError {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result { match (self) {
        Self::Io(err)        => write!(f, "{err}"),
        Self::Custom(err)    => write!(f, "{err}"),
        Self::UnknownTag(v)  => write!(f, "unknown tag 0x{v:0>2X}"),
        Self::BadString(err) => write!(f, "bad string {err}"),
        Self::BadChar        => write!(f, "bad char")
    } }
}
impl StdError for NbtDeserError { }
impl DeserError for NbtDeserError {
    fn custom<T>(msg : T) -> Self
    where
        T : Display
    {
        Self::Custom(format!("{msg}"))
    }
}


/// Deserialise network NBT as a value.
pub fn from_network<R, T>(mut reader : R) -> Result<T, NbtDeserError>
where
            R : Read,
    for<'l> T : Deser<'l>
{
    T::deserialize(&mut NbtRootDeserer::new(&mut reader, TagRead::Tag))
}
