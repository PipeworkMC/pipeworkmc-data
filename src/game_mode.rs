//! Player game modes.


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
