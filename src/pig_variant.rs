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


#[derive(Ser, Deser, Debug, SynDebug)]
#[serde(deny_unknown_fields)]
pub struct PigVariant {
    #[serde(rename = "asset_id")]
    pub texture_asset    : Ident,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model            : Option<PigVariantModel>,
    #[serde(skip_serializing)]
    pub spawn_conditions : IgnoredAny
}

#[derive(Ser, Deser, Debug, SynDebug)]
#[serde(deny_unknown_fields)]
pub enum PigVariantModel {
    #[serde(rename = "cold")]
    Cold
}


#[cfg(feature = "generated")]
include!("../../pipeworkmc-vanilla-datagen/output/generated/pig_variant.rs");


impl RegistryEntryType for PigVariant {
    const REGISTRY_ID : Ident = Ident::new("minecraft:pig_variant");

    fn to_network_nbt<W>(&self, writer : W) -> bool
    where
        W : Write
    {
        to_network_nbt(writer, self).unwrap();
        true
    }
}
