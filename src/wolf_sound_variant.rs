//! Wolf sound variant registry entries.


use crate::{
    ident::Ident,
    nbt::to_network as to_network_nbt,
    registry_entry::*
};
use std::io::Write;
use serde::{
    Serialize as Ser,
    Deserialize as Deser
};
use syndebug::SynDebug;


/// A wolf sound variant registry entry.
#[derive(Ser, Deser, Debug, SynDebug)]
#[serde(deny_unknown_fields)]
pub struct WolfSoundVariant {
    /// The resource ID of the hurt sound asset.
    pub hurt_sound    : Ident,
    /// The resource ID of the pant sound asset.
    pub pant_sound    : Ident,
    /// The resource ID of the whine sound asset.
    pub whine_sound   : Ident,
    /// The resource ID of the ambient sound asset.
    pub ambient_sound : Ident,
    /// The resource ID of the death sound asset.
    pub death_sound   : Ident,
    /// The resource ID of the growl sound asset.
    pub growl_sound   : Ident
}


#[cfg(feature = "generated")]
include!("../../pipeworkmc-vanilla-datagen/output/generated/wolf_sound_variant.rs");


impl RegistryEntryType for WolfSoundVariant {
    const REGISTRY_ID : Ident = Ident::new("minecraft:wolf_sound_variant");

    fn to_network_nbt<W>(&self, writer : W) -> bool
    where
        W : Write
    {
        to_network_nbt(writer, self).unwrap();
        true
    }
}
