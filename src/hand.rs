//! Player game modes.


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
use core::fmt::{ self, Display, Formatter };


/// A player game mode.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
#[repr(u8)]
pub enum Hand {
    /// Main hand
    #[default]
    Main,
    /// Off hand
    Off
}


impl PacketDecode for Hand {
    type Error = HandDecodeError;

    fn decode<I>(iter : &mut DecodeIter<I>) -> Result<Self, Self::Error>
    where
        I : ExactSizeIterator<Item = u8>
    {
        Ok(match (<VarInt<u32>>::decode(iter).map_err(HandDecodeError::Hand)?.0) {
            0 => Self::Main,
            1 => Self::Off,
            v => { return Err(HandDecodeError::UnknownHand(v))?; }
        })
    }
}

/// Returned by packet decoders when a `GameMode` was not decoded successfully.
#[derive(Debug)]
pub enum HandDecodeError {
    /// The hand failed to decode.
    Hand(VarIntDecodeError),
    /// An unknown hand was found.
    UnknownHand(u32)
}
impl Display for HandDecodeError {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result { match (self) {
        Self::Hand(err)      => write!(f, "hand {err}"),
        Self::UnknownHand(v) => write!(f, "unknown hand {v}")
    } }
}
