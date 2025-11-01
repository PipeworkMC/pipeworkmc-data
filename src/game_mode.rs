//! Player game modes.


use netzer::prelude::*;


/// A player game mode.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, NetEncode, NetDecode)]
#[cfg_attr(feature = "bevy", derive(bevy_ecs::component::Component))]
#[netzer(ordinal)]
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


/// A player game mode.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, NetEncode, NetDecode)]
#[cfg_attr(feature = "bevy", derive(bevy_ecs::component::Component))]
#[netzer(ordinal)]
#[repr(i8)]
pub enum PreviousGameMode {
    /// None
    None      = -1,
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
