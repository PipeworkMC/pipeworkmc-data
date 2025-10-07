use super::tag;
use core::{
    error::Error as StdError,
    fmt::{ self, Display, Formatter }
};
use std::io::{ self, Write };
use serde::ser::{
    Error as SerError,
    Serialize as Ser
};


mod root;
use root::{ NbtRootSerer, TagWrite };
mod seq;
use seq::NbtSeqSerer;
mod map;
use map::NbtMapSerer;
mod str;
use str::NbtStrSerer;

mod never;


/// Errors emitted while serialising a value as NBT.
#[derive(Debug)]
pub enum NbtSerError {
    /// An io error occured.
    Io(io::Error),
    /// A value serialiser returned a custom error.
    Custom(String)
}
impl From<io::Error> for NbtSerError {
    #[inline]
    fn from(err : io::Error) -> Self {
        Self::Io(err)
    }
}

impl Display for NbtSerError {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result { match (self) {
        Self::Io(err)     => write!(f, "{err}"),
        Self::Custom(err) => write!(f, "{err}")
    } }
}
impl StdError for NbtSerError { }
impl SerError for NbtSerError {
    fn custom<T>(msg : T) -> Self
    where
        T : Display
    {
        Self::Custom(format!("{msg}"))
    }
}


/// Serialise a value as network NBT.
pub fn to_network<W, T>(mut writer : W, value : &T) -> Result<(), NbtSerError>
where
    W : Write,
    T : Ser
{
    value.serialize(NbtRootSerer::new(&mut writer, TagWrite::Tag))?;
    Ok(())
}
