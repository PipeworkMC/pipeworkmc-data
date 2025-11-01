//! Client settings.


use crate::bounded_string::BoundedString;
use core::{
    fmt::{ self, Debug, Formatter },
    num::NonZeroU8
};
use netzer::prelude::*;


/// Client settings.
#[derive(Clone, Debug, NetEncode, NetDecode)]
#[cfg_attr(feature = "bevy", derive(bevy_ecs::component::Component))]
pub struct ClientInfo {
    /// Selected language code.
    pub locale             : BoundedString<16>,
    /// View distance in chunks.
    pub view_dist          : NonZeroU8,
    /// Enabled chat message types.
    pub chat_mode          : ChatMode,
    /// Whether chat colours are enabled.
    pub chat_colours       : bool,
    /// Skin layers to display.
    pub skin_layers        : SkinLayers,
    /// Whether to use left as main hand.
    pub left_handed        : bool,
    /// Whether text on signs and in book titles are filtered.
    pub text_filtered      : bool,
    /// Whether this player should be shown in server list.
    pub allow_motd_listing : bool,
    /// Amount of particles which will be displayed.
    pub particle_status    : ParticleStatus
}
impl Default for ClientInfo {
    fn default() -> Self { Self {
        locale             : unsafe { BoundedString::new_unchecked("en_us") },
        view_dist          : unsafe { NonZeroU8::new_unchecked(2) },
        chat_mode          : ChatMode::Enabled,
        chat_colours       : false,
        skin_layers        : SkinLayers::ALL,
        left_handed        : false,
        text_filtered      : false,
        allow_motd_listing : false,
        particle_status    : ParticleStatus::All
    } }
}


/// Enable chat message types.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, NetEncode, NetDecode)]
#[netzer(ordinal, convert = "VarInt<u32>", try_from)]
#[repr(u8)]
pub enum ChatMode {
    /// Chat is fully enabled.
    Enabled      = 0,
    /// Only command feedback is shown.
    CommandsOnly = 1,
    /// All chat is hidden.
    Hidden       = 2
}


/// Skin layers to display.
#[derive(Clone, Copy, PartialEq, Eq, Hash, NetEncode, NetDecode)]
#[repr(transparent)]
pub struct SkinLayers(u8);
impl SkinLayers {

    /// All skin layers shown.
    pub const ALL  : Self = Self(0b01111111);
    /// No skin layers shown.
    pub const NONE : Self = Self(0b00000000);

    /// Return the inner value as a primitive type.
    #[inline]
    pub const fn as_byte(&self) -> u8 { self.0 }

    /// The cape byte flag.
    pub const CAPE : u8 = 0b00000001;
    /// Returns whether the cape flag is enabled.
    pub const fn cape(&self) -> bool { self.get(Self::CAPE) }
    /// Sets whether the cape flag is enabled.
    pub const fn set_cape(&mut self, enabled : bool) { self.set(Self::CAPE, enabled); }

    /// The jacket byte flag.
    pub const JACKET : u8 = 0b00000010;
    /// Returns whether the jacket flag is enabled.
    pub const fn jacket(&self) -> bool { self.get(Self::JACKET) }
    /// Sets whether the jacket flag is enabled.
    pub const fn set_jacket(&mut self, enabled : bool) { self.set(Self::JACKET, enabled); }

    /// The left sleeve byte flag.
    pub const LEFT_SLEEVE : u8 = 0b00000100;
    /// Returns whether the left sleeve flag is enabled.
    pub const fn left_sleeve(&self) -> bool { self.get(Self::LEFT_SLEEVE) }
    /// Sets whether the left sleeve flag is enabled.
    pub const fn set_left_sleeve(&mut self, enabled : bool) { self.set(Self::LEFT_SLEEVE, enabled); }

    /// The right sleeve byte flag.
    pub const RIGHT_SLEEVE : u8 = 0b00001000;
    /// Returns whether the right sleeve flag is enabled.
    pub const fn right_sleeve(&self) -> bool { self.get(Self::RIGHT_SLEEVE) }
    /// Sets whether the right sleeve flag is enabled.
    pub const fn set_right_sleeve(&mut self, enabled : bool) { self.set(Self::RIGHT_SLEEVE, enabled); }

    /// The left pants leg byte flag.
    pub const LEFT_PANTS_LEG : u8 = 0b00010000;
    /// Returns whether the left pants leg flag is enabled.
    pub const fn left_pants_leg(&self) -> bool { self.get(Self::LEFT_PANTS_LEG) }
    /// Sets whether the left pants leg flag is enabled.
    pub const fn set_left_pants_leg(&mut self, enabled : bool) { self.set(Self::LEFT_PANTS_LEG, enabled); }

    /// The right pants leg byte flag.
    pub const RIGHT_PANTS_LEG : u8 = 0b00100000;
    /// Returns whether the right pants leg flag is enabled.
    pub const fn right_pants_leg(&self) -> bool { self.get(Self::RIGHT_PANTS_LEG) }
    /// Sets whether the right pants leg flag is enabled.
    pub const fn set_right_pants_leg(&mut self, enabled : bool) { self.set(Self::RIGHT_PANTS_LEG, enabled); }

    /// The hat byte flag.
    pub const HAT : u8 = 0b01000000;
    /// Returns whether the hat flag is enabled.
    pub const fn hat(&self) -> bool { self.get(Self::HAT) }
    /// Sets whether the hat flag is enabled.
    pub const fn set_hat(&mut self, enabled : bool) { self.set(Self::HAT, enabled); }

    #[inline]
    const fn get(&self, flag : u8) -> bool { (self.0 & flag) != 0 }
    #[inline]
    const fn set(&mut self, flag : u8, enabled : bool) {
        if (enabled) {
            self.0 |= flag;
        } else {
            self.0 &= ! flag;
        }
    }

}
impl Debug for SkinLayers {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("SkinLayers")
            .field("cape", &self.cape())
            .field("jacket", &self.jacket())
            .field("left_sleeve", &self.left_sleeve())
            .field("right_sleeve", &self.right_sleeve())
            .field("left_pants_leg", &self.left_pants_leg())
            .field("right_pants_leg", &self.right_pants_leg())
            .field("hat", &self.hat())
            .finish()
    }
}


/// Amount of particles which will be displayed.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, NetEncode, NetDecode)]
#[netzer(ordinal, convert = "VarInt<u32>", try_from)]
#[repr(u8)]
pub enum ParticleStatus {
    /// All particles shown.
    All       = 0,
    /// Decreased particles shown.
    Decreased = 1,
    /// Minimal particles shown.
    Minimal   = 2
}
