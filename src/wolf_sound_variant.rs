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


#[derive(Ser, Deser, Debug, SynDebug)]
#[serde(deny_unknown_fields)]
pub struct WolfSoundVariant {
    pub hurt_sound    : Ident,
    pub pant_sound    : Ident,
    pub whine_sound   : Ident,
    pub ambient_sound : Ident,
    pub death_sound   : Ident,
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
