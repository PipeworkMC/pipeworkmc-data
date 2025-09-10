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
pub struct ChickenVariant {
    #[serde(rename = "asset_id")]
    pub texture_asset    : Ident,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model            : Option<ChickenVariantModel>,
    #[serde(skip_serializing)]
    pub spawn_conditions : IgnoredAny
}

#[derive(Ser, Deser, Debug, SynDebug)]
#[serde(deny_unknown_fields)]
pub enum ChickenVariantModel {
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
