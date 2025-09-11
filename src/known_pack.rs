use pipeworkmc_codec::{
    decode::{
        PacketDecode,
        DecodeBuf,
        string::StringDecodeError
    },
    encode::{
        PacketEncode,
        EncodeBuf
    }
};
use core::fmt::{ self, Display, Formatter };
use std::borrow::Cow;


#[derive(Debug, Clone)]
pub struct KnownPack<'l> {
    pub namespace : Cow<'l, str>,
    pub id        : Cow<'l, str>,
    pub version   : Cow<'l, str>
}


impl PacketDecode for KnownPack<'_> {
    type Error = KnownPackDecodeError;

    fn decode(buf : &mut DecodeBuf<'_>)
        -> Result<Self, Self::Error>
    { Ok(Self {
        namespace : Cow::Owned(<_>::decode(buf).map_err(KnownPackDecodeError::Namespace)?),
        id        : Cow::Owned(<_>::decode(buf).map_err(KnownPackDecodeError::Id)?),
        version   : Cow::Owned(<_>::decode(buf).map_err(KnownPackDecodeError::Version)?)
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


#[derive(Debug)]
pub enum KnownPackDecodeError {
    Namespace(StringDecodeError),
    Id(StringDecodeError),
    Version(StringDecodeError)
}
impl Display for KnownPackDecodeError {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result { match (self) {
        Self::Namespace(err) => write!(f, "namespace {err}"),
        Self::Id(err)        => write!(f, "id {err}"),
        Self::Version(err)   => write!(f, "version {err}")
    } }
}
