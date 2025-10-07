//! Player game modes.


use pipeworkmc_codec::decode::{
    PacketDecode,
    DecodeIter,
    IncompleteDecodeError
};
use core::fmt::{ self, Display, Formatter };
use bevy_ecs::component::Component;


/// A player game mode.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, Component)]
#[repr(u8)]
pub enum GameMode {
    /// Survival
    Survival  = 0,
    /// Creative
    Creative  = 1,
    /// Adventure
    #[default]
    Adventure = 2,
    /// Spectator
    Spectator = 3
}


impl PacketDecode for GameMode {
    type Error = GameModeDecodeError;

    fn decode<I>(iter : &mut DecodeIter<I>) -> Result<Self, Self::Error>
    where
        I : ExactSizeIterator<Item = u8>
    {
        Ok(match (<u8>::decode(iter).map_err(GameModeDecodeError::GameMode)?) {
            0 => Self::Survival,
            1 => Self::Creative,
            2 => Self::Adventure,
            3 => Self::Spectator,
            v => { return Err(GameModeDecodeError::UnknownGameMode(v))?; }
        })
    }
}

/// Returned by packet decoders when a `GameMode` was not decoded successfully.
#[derive(Debug)]
pub enum GameModeDecodeError {
    /// The game mode failed to decode.
    GameMode(IncompleteDecodeError),
    /// An unknown game mode was found.
    UnknownGameMode(u8)
}
impl Display for GameModeDecodeError {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result { match (self) {
        Self::GameMode(err)      => write!(f, "game mode {err}"),
        Self::UnknownGameMode(v) => write!(f, "unknown game mode {v}")
    } }
}
