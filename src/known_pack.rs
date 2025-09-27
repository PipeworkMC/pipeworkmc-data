//! Data packs present in the game.


use pipeworkmc_codec::{
    decode::{
        PacketDecode,
        DecodeIter,
        string::StringDecodeError
    },
    encode::{
        PacketEncode,
        EncodeBuf
    }
};
use core::fmt::{ self, Display, Formatter };
use std::borrow::Cow;


/// A data pack present in the game.
#[derive(Debug, Clone)]
pub struct KnownPack<'l> {
    /// The namespace of the pack.
    pub namespace : Cow<'l, str>,
    /// The ID of the pack.
    pub id        : Cow<'l, str>,
    /// The game version of the pack.
    pub version   : Cow<'l, str>
}


impl PacketDecode for KnownPack<'_> {
    type Error = KnownPackDecodeError;

    fn decode<I>(iter : &mut DecodeIter<I>) -> Result<Self, Self::Error>
    where
        I : ExactSizeIterator<Item = u8>
    { Ok(Self {
        namespace : Cow::Owned(<_>::decode(iter).map_err(KnownPackDecodeError::Namespace)?),
        id        : Cow::Owned(<_>::decode(iter).map_err(KnownPackDecodeError::Id)?),
        version   : Cow::Owned(<_>::decode(iter).map_err(KnownPackDecodeError::Version)?)
    }) }
}


unsafe impl PacketEncode for KnownPack<'_> {

    fn encode_len(&self) -> usize {
        self.namespace.encode_len()
        + self.id.encode_len()
        + self.version.encode_len()
    }

    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        self.namespace.encode(buf);
        self.id.encode(buf);
        self.version.encode(buf);
    } }

}


/// Returned by packet decoders when a `KnownPack` was not decoded successfully.
#[derive(Debug)]
pub enum KnownPackDecodeError {
    /// The namespace failed to decode.
    Namespace(StringDecodeError),
    /// The ID failed to decode.
    Id(StringDecodeError),
    /// The version failed to decode.
    Version(StringDecodeError)
}
impl Display for KnownPackDecodeError {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result { match (self) {
        Self::Namespace(err) => write!(f, "namespace {err}"),
        Self::Id(err)        => write!(f, "id {err}"),
        Self::Version(err)   => write!(f, "version {err}")
    } }
}
