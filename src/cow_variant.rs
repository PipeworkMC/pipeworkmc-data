//! Cow variant registry entries.


use crate::{
    ident::Ident,
    nbt::to_network as to_network_nbt,
    registry_entry::*
};
use std::io::Write;
use serde::{
    Serialize as Ser,
    Deserialize as Deser,
    de::IgnoredAny
};
use syndebug::SynDebug;


/// A cow variant registry entry.
#[derive(Ser, Deser, Debug, SynDebug)]
#[serde(deny_unknown_fields)]
pub struct CowVariant {
    /// The resource ID of the texture asset.
    #[serde(rename = "asset_id")]
    pub texture_asset    : Ident,
    /// The model to display as.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model            : Option<CowVariantModel>,
    /// Conditions required to spawn.
    ///
    /// Unused by this library.
    #[serde(skip_serializing)]
    pub spawn_conditions : IgnoredAny
}

/// Cow model.
#[derive(Ser, Deser, Debug, SynDebug)]
#[serde(deny_unknown_fields)]
pub enum CowVariantModel {
    /// Cold.
    #[serde(rename = "cold")]
    Cold,
    /// Warm.
    #[serde(rename = "warm")]
    Warm
}


#[cfg(feature = "generated")]
include!("../../pipeworkmc-vanilla-datagen/output/generated/cow_variant.rs");


impl RegistryEntryType for CowVariant {
    const REGISTRY_ID : Ident = Ident::new("minecraft:cow_variant");

    fn to_network_nbt<W>(&self, writer : W) -> bool
    where
        W : Write
    {
        to_network_nbt(writer, self).unwrap();
        true
    }
}
