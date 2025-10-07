//! World difficulties.


use pipeworkmc_codec::decode::{
    PacketDecode,
    DecodeIter,
    IncompleteDecodeError
};
use core::fmt::{ self, Display, Formatter };


/// A world difficulty.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
#[repr(u8)]
pub enum Difficulty {
    /// Peaceful
    Peaceful = 0,
    /// Easy
    Easy     = 1,
    /// Normal
    #[default]
    Normal   = 2,
    /// Hard
    Hard     = 3
}


impl PacketDecode for Difficulty {
    type Error = DifficultyDecodeError;

    fn decode<I>(iter : &mut DecodeIter<I>) -> Result<Self, Self::Error>
    where
        I : ExactSizeIterator<Item = u8>
    {
        Ok(match (<u8>::decode(iter).map_err(DifficultyDecodeError::Difficulty)?) {
            0 => Self::Peaceful,
            1 => Self::Easy,
            2 => Self::Normal,
            3 => Self::Hard,
            v => { return Err(DifficultyDecodeError::UnknownDifficulty(v))?; }
        })
    }
}

/// Returned by packet decoders when a `Difficulty` was not decoded successfully.
#[derive(Debug)]
pub enum DifficultyDecodeError {
    /// The difficulty failed to decode.
    Difficulty(IncompleteDecodeError),
    /// An unknown difficulty was found.
    UnknownDifficulty(u8)
}
impl Display for DifficultyDecodeError {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result { match (self) {
        Self::Difficulty(err)      => write!(f, "difficulty {err}"),
        Self::UnknownDifficulty(v) => write!(f, "unknown difficulty {v}")
    } }
}
