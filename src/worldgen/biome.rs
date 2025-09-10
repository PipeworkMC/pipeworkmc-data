use crate::{
    colour::Rgb,
    ident::Ident,
    nbt::to_network as to_network_nbt,
    num::weighted::Weighted,
    particle::Particle,
    registry_entry::*
};
use crate::{
    is_default,
    slice_is_empty
};
use std::{
    borrow::Cow,
    io::Write
};
use serde::{
    Serialize as Ser,
    Deserialize as Deser,
    de::Deserializer as Deserer
};
use syndebug::SynDebug;


#[derive(Ser, Deser, Debug, SynDebug)]
pub struct WorldgenBiome<'l> {
    #[serde(rename = "has_precipitation")]
    pub can_rain             : bool,
    pub temperature          : f32,
    #[serde(skip_serializing_if = "is_default", default)]
    pub temperature_modifier : WorldgenBiomeTemperatureModifier,
    #[serde(rename = "downfall")]
    pub downfall_factor      : f32,
    pub effects              : WorldgenBiomeEffects<'l>
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Ser, Deser, Debug, SynDebug)]
pub enum WorldgenBiomeTemperatureModifier {
    #[default]
    #[serde(rename = "none")]
    None,
    #[serde(rename = "frozen")]
    Frozen
}

#[derive(Ser, Deser, Debug, SynDebug)]
pub struct WorldgenBiomeEffects<'l> {
    #[serde(rename = "fog_color")]
    pub fog_colour            : Rgb,
    #[serde(rename = "water_color")]
    pub water_color           : Rgb,
    #[serde(rename = "water_fog_color")]
    pub water_fog_colour      : Rgb,
    #[serde(rename = "sky_color")]
    pub sky_colour            : Rgb,
    #[serde(rename = "foliage_color", skip_serializing_if = "Option::is_none")]
    pub foliage_colour        : Option<Rgb>,
    #[serde(rename = "grass_color", skip_serializing_if = "Option::is_none")]
    pub grass_colour          : Option<Rgb>,
    #[serde(rename = "grass_color_modifier", skip_serializing_if = "is_default", default)]
    pub grass_colour_modifier : WorldgenBiomeGrassColourModifier,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub particle              : Option<WorldgenBiomeParticle>,
    #[serde(skip_serializing_if = "Option::is_none", deserialize_with = "deser_ambient_sound", default)]
    pub ambient_sound         : Option<WorldgenBiomeAmbientSound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mood_sound            : Option<WorldgenBiomeMoodSound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additions_sound       : Option<WorldgenBiomeAdditionsSound>,
    #[serde(skip_serializing_if = "slice_is_empty", default)]
    pub music                 : Cow<'l, [Weighted<WorldgenBiomeMusic>]>
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Ser, Deser, Debug, SynDebug)]
pub enum WorldgenBiomeGrassColourModifier {
    #[default]
    #[serde(rename = "none")]
    None,
    #[serde(rename = "dark_forest")]
    DarkForest,
    #[serde(rename = "swamp")]
    Swamp
}

#[derive(Ser, Deser, Debug, SynDebug)]
pub struct WorldgenBiomeParticle {
    pub options     : Particle,
    pub probability : f32
}

#[derive(Ser, Deser, Debug, SynDebug)]
pub struct WorldgenBiomeAmbientSound {
    #[serde(rename = "sound_id")]
    pub sound : Ident,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range  : Option<f32>
}

#[derive(Deser)]
#[serde(untagged)]
enum VerboseableWorldgenBiomeAmbientSound {
    Short(Ident),
    Verbose(WorldgenBiomeAmbientSound)
}
impl From<VerboseableWorldgenBiomeAmbientSound> for WorldgenBiomeAmbientSound {
    fn from(value : VerboseableWorldgenBiomeAmbientSound) -> Self { match (value) {
        VerboseableWorldgenBiomeAmbientSound::Short(sound)   => Self { sound, range : None },
        VerboseableWorldgenBiomeAmbientSound::Verbose(sound) => sound
    } }
}
#[inline]
fn deser_ambient_sound<'de, D>(deserer : D) -> Result<Option<WorldgenBiomeAmbientSound>, D::Error>
where
    D : Deserer<'de>
{ Ok(Some(VerboseableWorldgenBiomeAmbientSound::deserialize(deserer)?.into())) }

#[derive(Ser, Deser, Debug, SynDebug)]
pub struct WorldgenBiomeMoodSound {
    pub sound               : Ident,
    pub tick_delay          : u32,
    pub block_search_extent : u32,
    pub offset              : f64
}

#[derive(Ser, Deser, Debug, SynDebug)]
pub struct WorldgenBiomeAdditionsSound {
    pub sound       : Ident,
    pub tick_chance : f64
}

#[derive(Clone, Ser, Deser, Debug, SynDebug)]
pub struct WorldgenBiomeMusic {
    pub sound           : Ident,
    pub min_delay       : u32,
    pub max_delay       : u32,
    #[serde(rename = "replace_current_music")]
    pub replace_current : bool
}


#[cfg(feature = "generated")]
include!("../../../pipeworkmc-vanilla-datagen/output/generated/worldgen/biome.rs");


impl RegistryEntryType for WorldgenBiome<'_> {
    const REGISTRY_ID : Ident = Ident::new("minecraft:worldgen/biome");

    fn to_network_nbt<W>(&self, writer : W) -> bool
    where
        W : Write
    {
        to_network_nbt(writer, self).unwrap();
        true
    }
}
