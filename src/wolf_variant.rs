//! Wolf variant registry entries.


use crate::{
    ident::Ident,
    nbt::to_network as to_network_nbt,
    registry_entry::*
};
use std::{
    borrow::Cow,
    io::Write
};
use serde::{
    Serialize as Ser,
    Deserialize as Deser,
    de::IgnoredAny
};
use syndebug::SynDebug;


/// A wolf variant registry entry.
#[derive(Ser, Deser, Debug, SynDebug)]
#[serde(deny_unknown_fields)]
pub struct WolfVariant<'l> {
    /// Resource IDs of the texture assets.
    pub assets           : WolfVariantAssets,
    /// Biomes which the wolf spawns in.
    #[serde(default)]
    pub biomes           : Cow<'l, [Ident]>,
    /// Conditions required to spawn.
    ///
    /// Unused by this library.
    #[serde(skip_serializing)]
    pub spawn_conditions : IgnoredAny
}

/// Resource IDs of a wolf variant's texture assets.
#[derive(Ser, Deser, Debug, SynDebug)]
#[serde(deny_unknown_fields)]
pub struct WolfVariantAssets {
    /// The resource ID of the wild texture asset.
    pub wild  : Ident,
    /// The resource ID of the tame texture asset.
    pub tame  : Ident,
    /// The resource ID of the angry texture asset.
    pub angry : Ident
}


#[cfg(feature = "generated")]
include!("../../pipeworkmc-vanilla-datagen/output/generated/wolf_variant.rs");


impl RegistryEntryType for WolfVariant<'_> {
    const REGISTRY_ID : Ident = Ident::new("minecraft:wolf_variant");

    fn to_network_nbt<W>(&self, writer : W) -> bool
    where
        W : Write
    {
        to_network_nbt(writer, self).unwrap();
        true
    }
}
