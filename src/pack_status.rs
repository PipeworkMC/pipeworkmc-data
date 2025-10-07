//! Resource pack status updates sent by the client.


use pipeworkmc_codec::{
    decode::{
        PacketDecode,
        DecodeIter
    },
    varint::{
        VarInt,
        VarIntDecodeError
    }
};
use core::fmt::{ self,
    Display,
    Formatter
};


/// Resource pack status updates sent by the client.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PackStatus {
    /// The client has successfully loaded the resource pack.
    Loaded,
    /// The client explicitely declined the resource pack by clicking on the reject button.
    Declined,
    /// The client failed to download the resource pack.
    DownloadFailed,
    /// The client has explicitely accepted the resource pack by clicking on the accept button, but has not yet loaded it.
    Accepted,
    /// The client has downloaded the resource pack, but has not yet loaded it.
    Downloaded,
    /// The resource pack URL was invalid.
    InvalidURL,
    /// The client failed to reload its resource packs.
    FailedToReload,
    /// The client has discarded the resource pack.
    Discarded
}


impl PacketDecode for PackStatus {
    type Error = PackStatusDecodeError;

    fn decode<I>(iter : &mut DecodeIter<I>) -> Result<Self, Self::Error>
    where
        I : ExactSizeIterator<Item = u8>
    {
        let v = VarInt::<u32>::decode(iter).map_err(PackStatusDecodeError::Status)?.0;
        Ok(match (v) {
            0 => Self::Loaded,
            1 => Self::Declined,
            2 => Self::DownloadFailed,
            3 => Self::Accepted,
            4 => Self::Downloaded,
            5 => Self::InvalidURL,
            6 => Self::FailedToReload,
            7 => Self::Discarded,
            _ => { return Err(PackStatusDecodeError::UnknownStatus(v)); }
        })
    }
}

/// Returned by packet decoders when a `PackStatus` was not decoded successfully.
#[derive(Debug)]
pub enum PackStatusDecodeError {
    /// The status failed to decode.
    Status(VarIntDecodeError),
    /// An unknown status was found.
    UnknownStatus(u32)
}
impl Display for PackStatusDecodeError {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result { match (self) {
        Self::Status(err)      => write!(f, "status {err}"),
        Self::UnknownStatus(v) => write!(f, "unknown status {v}")
    } }
}
