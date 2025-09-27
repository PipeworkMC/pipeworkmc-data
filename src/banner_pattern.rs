//! Banner pattern registry entries.


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
    Deserialize as Deser
};
use syndebug::SynDebug;


/// A banner pattern registry entry.
#[derive(Ser, Deser, Debug, SynDebug)]
#[serde(deny_unknown_fields)]
pub struct BannerPattern<'l> {
    /// The resource ID of the pattern asset.
    #[serde(rename = "asset_id")]
    pub pattern_asset : Ident,
    /// The translation key of the tooltip shown on banner items.
    #[serde(rename = "translation_key")]
    pub tooltip_key   : Cow<'l, str>
}


#[cfg(feature = "generated")]
include!("../../pipeworkmc-vanilla-datagen/output/generated/banner_pattern.rs");


impl RegistryEntryType for BannerPattern<'_> {
    const REGISTRY_ID : Ident = Ident::new("minecraft:banner_pattern");

    fn to_network_nbt<W>(&self, writer : W) -> bool
    where
        W : Write
    {
        to_network_nbt(writer, self).unwrap();
        true
    }
}
