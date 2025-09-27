//! Chicken variant registry entries.


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


/// A chicken variant registry entry.
#[derive(Ser, Deser, Debug, SynDebug)]
#[serde(deny_unknown_fields)]
pub struct ChickenVariant {
    /// The resource ID of the texture asset.
    #[serde(rename = "asset_id")]
    pub texture_asset    : Ident,
    /// The model to display as.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model            : Option<ChickenVariantModel>,
    /// Conditions required to spawn.
    ///
    /// Unused by this library.
    #[serde(skip_serializing)]
    pub spawn_conditions : IgnoredAny
}

/// Chicken model.
#[derive(Ser, Deser, Debug, SynDebug)]
#[serde(deny_unknown_fields)]
pub enum ChickenVariantModel {
    /// Cold.
    #[serde(rename = "cold")]
    Cold
}


#[cfg(feature = "generated")]
include!("../../pipeworkmc-vanilla-datagen/output/generated/chicken_variant.rs");


impl RegistryEntryType for ChickenVariant {
    const REGISTRY_ID : Ident = Ident::new("minecraft:chicken_variant");

    fn to_network_nbt<W>(&self, writer : W) -> bool
    where
        W : Write
    {
        to_network_nbt(writer, self).unwrap();
        true
    }
}
