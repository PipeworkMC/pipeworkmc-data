//! Player operator permission levels.


use bevy_ecs::component::Component;


/// A player's operator permission level.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Component)]
#[repr(u8)]
pub enum OperatorLevel {
    /// No permissions.
    All        = 0,
    /// Bypass spawn permission.
    Moderator  = 1,
    /// Use most singleplayer commands.
    Gamemaster = 2,
    /// Use player management commands.
    Admin      = 3,
    /// Use server lifecycle commands.
    Owner      = 4
}
