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


#[derive(Ser, Deser, Debug, SynDebug)]
#[serde(deny_unknown_fields)]
pub struct WolfVariant<'l> {
    pub assets           : WolfVariantAssets,
    #[serde(default)]
    pub biomes           : Cow<'l, [Ident]>,
    #[serde(skip_serializing)]
    pub spawn_conditions : IgnoredAny
}

#[derive(Ser, Deser, Debug, SynDebug)]
#[serde(deny_unknown_fields)]
pub struct WolfVariantAssets {
    pub wild  : Ident,
    pub tame  : Ident,
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
