//! Painting variant registry entries.


use crate::{
    ident::Ident,
    nbt::to_network as to_network_nbt,
    registry_entry::*,
    text::*
};
use core::num::NonZeroU32;
use std::io::Write;
use serde::{
    Serialize as Ser,
    Deserialize as Deser
};
use syndebug::SynDebug;


/// A painting variant registr yentry.
#[derive(Ser, Deser, Debug, SynDebug)]
#[serde(deny_unknown_fields)]
pub struct PaintingVariant {
    /// The resource ID of the texture asset.
    #[serde(rename = "asset_id")]
    pub texture_asset : Ident,
    /// The height of the painting in blocks.
    pub height        : NonZeroU32,
    /// The width of the painting in blocks.
    pub width         : NonZeroU32,
    /// The title of the painting, shown in the item tooltip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title         : Option<Text>,
    /// The author of the painting, shown in the item tooltip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author        : Option<Text>
}


#[cfg(feature = "generated")]
use {
    crate::colour::Rgb,
    core::num::NonZero,
    std::borrow::Cow
};
#[cfg(feature = "generated")]
include!("../../pipeworkmc-vanilla-datagen/output/generated/painting_variant.rs");


impl RegistryEntryType for PaintingVariant {
    const REGISTRY_ID : Ident = Ident::new("minecraft:painting_variant");

    fn to_network_nbt<W>(&self, writer : W) -> bool
    where
        W : Write
    {
        to_network_nbt(writer, self).unwrap();
        true
    }
}
