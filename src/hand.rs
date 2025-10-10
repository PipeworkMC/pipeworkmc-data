//! Player game modes.


use netzer::prelude::*;


/// A player game mode.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, NetEncode, NetDecode)]
#[netzer(ordinal, convert = "VarInt<u32>", try_from)]
#[repr(u8)]
pub enum Hand {
    /// Main hand
    #[default]
    Main = 0,
    /// Off hand
    Off  = 1
}
