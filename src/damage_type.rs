//! Damage type registry entries.


use crate::{
    ident::Ident,
    nbt::to_network as to_network_nbt,
    registry_entry::*
};
use crate::is_default;
use std::{
    borrow::Cow,
    io::Write
};
use serde::{
    Serialize as Ser,
    Deserialize as Deser
};
use syndebug::SynDebug;


/// A damage type registry entry.
#[derive(Ser, Deser, Debug, SynDebug)]
#[serde(deny_unknown_fields)]
pub struct DamageType<'l> {
    /// The ID of the message to use when constructing death messages.
    pub message_id    : Cow<'l, str>,
    /// When damage is scaled based on difficulty.
    ///
    /// Unused by this library.
    pub scaling       : DamageTypeScaling,
    /// How much exhaustion is added when damage of this type is applied..
    pub exhaustion    : f32,
    /// The sound effect played when damage of this type is applied.
    #[serde(skip_serializing_if = "is_default", default)]
    pub effects       : DamageTypeEffects,
    /// Special formatting when constructing a death message.
    #[serde(rename = "death_message_type", skip_serializing_if = "is_default", default)]
    pub death_message : DamageTypeDeathMessage
}

/// When damage is scaled based on difficulty
#[derive(Clone, Copy, PartialEq, Eq, Hash, Ser, Deser, Debug, SynDebug)]
#[serde(deny_unknown_fields)]
pub enum DamageTypeScaling {
    /// Damage amount is never scaled based on difficulty.
    #[serde(rename = "never")]
    Never,
    /// Damage amount is scaled based on difficulty when a living, non-player character is the attacker.
    #[serde(rename = "when_caused_by_living_non_player")]
    WhenByEnemy,
    /// Damage amount is always scaled based on difficulty.
    #[serde(rename = "always")]
    Always
}

/// Sound effect played when damage of this type is applied.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Ser, Deser, Debug, SynDebug)]
#[serde(deny_unknown_fields)]
pub enum DamageTypeEffects {
    /// Default hurt sound.
    #[default]
    #[serde(rename = "hurt")]
    Hurt,
    /// Thorns sound (Cacti).
    #[serde(rename = "thorns")]
    Thorns,
    /// Drowning sound (Underwater)
    #[serde(rename = "drowning")]
    Drowning,
    /// Burning sound (Fire, lava, magma block)
    #[serde(rename = "burning")]
    Burning,
    /// Poking sound (Berry bush)
    #[serde(rename = "poking")]
    Poking,
    /// Freezing sound (Powder snow)
    #[serde(rename = "freezing")]
    Freezing
}

/// Special formatting when constructing a death message.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Ser, Deser, Debug, SynDebug)]
#[serde(deny_unknown_fields)]
pub enum DamageTypeDeathMessage {
    /// No special effects.
    #[default]
    #[serde(rename = "default")]
    Default,
    /// Fall variants.
    #[serde(rename = "fall_variants")]
    FallVariants,
    /// Intentional game design (bed/respawn anchor explosion).
    #[serde(rename = "intentional_game_design")]
    IntentionalGameDesign
}


#[cfg(feature = "generated")]
include!("../../pipeworkmc-vanilla-datagen/output/generated/damage_type.rs");


impl RegistryEntryType for DamageType<'_> {
    const REGISTRY_ID : Ident = Ident::new("minecraft:damage_type");

    fn to_network_nbt<W>(&self, writer : W) -> bool
    where
        W : Write
    {
        to_network_nbt(writer, self).unwrap();
        true
    }
}
