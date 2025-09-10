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
pub struct CatVariant {
    #[serde(rename = "asset_id")]
    pub texture_asset    : Ident,
    #[serde(skip_serializing)]
    pub spawn_conditions : IgnoredAny
}


#[cfg(feature = "generated")]
include!("../../pipeworkmc-vanilla-datagen/output/generated/cat_variant.rs");


impl RegistryEntryType for CatVariant {
    const REGISTRY_ID : Ident = Ident::new("minecraft:cat_variant");

    fn to_network_nbt<W>(&self, writer : W) -> bool
    where
        W : Write
    {
        to_network_nbt(writer, self).unwrap();
        true
    }
}
