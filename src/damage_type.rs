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


#[derive(Ser, Deser, Debug, SynDebug)]
#[serde(deny_unknown_fields)]
pub struct DamageType<'l> {
    pub message_id    : Cow<'l, str>,
    pub scaling       : DamageTypeScaling,
    pub exhaustion    : f32,
    #[serde(skip_serializing_if = "is_default", default)]
    pub effects       : DamageTypeEffects,
    #[serde(rename = "death_message_type", skip_serializing_if = "is_default", default)]
    pub death_message : DamageTypeDeathMessage
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Ser, Deser, Debug, SynDebug)]
#[serde(deny_unknown_fields)]
pub enum DamageTypeScaling {
    #[serde(rename = "never")]
    Never,
    #[serde(rename = "when_caused_by_living_non_player")]
    WhenByEnemy,
    #[serde(rename = "always")]
    Always
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Ser, Deser, Debug, SynDebug)]
#[serde(deny_unknown_fields)]
pub enum DamageTypeEffects {
    #[default]
    #[serde(rename = "hurt")]
    Hurt,
    #[serde(rename = "thorns")]
    Thorns,
    #[serde(rename = "drowning")]
    Drowning,
    #[serde(rename = "burning")]
    Burning,
    #[serde(rename = "poking")]
    Poking,
    #[serde(rename = "freezing")]
    Freezing
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Ser, Deser, Debug, SynDebug)]
#[serde(deny_unknown_fields)]
pub enum DamageTypeDeathMessage {
    #[default]
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "fall_variants")]
    FallVariants,
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
