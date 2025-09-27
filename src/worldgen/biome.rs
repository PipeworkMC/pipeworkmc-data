//! Biome registry entries.


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


/// A biome registry entry.
#[derive(Ser, Deser, Debug, SynDebug)]
pub struct WorldgenBiome<'l> {
    /// Whether the biome has precipitation.
    #[serde(rename = "has_precipitation")]
    pub can_rain             : bool,
    /// The temperature factor of the biome.
    ///
    /// Affects grass and foliage colour.
    pub temperature          : f32,
    /// Modifier that affects the resulting temperature.
    #[serde(skip_serializing_if = "is_default", default)]
    pub temperature_modifier : WorldgenBiomeTemperatureModifier,
    /// The downfall factor of the biome.
    ///
    /// Affects grass and foliage colour.
    #[serde(rename = "downfall")]
    pub downfall_factor      : f32,
    /// Biome special effects.
    pub effects              : WorldgenBiomeEffects<'l>
}

/// Modifier for temperature factor in a biome.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Ser, Deser, Debug, SynDebug)]
pub enum WorldgenBiomeTemperatureModifier {
    /// Static temperature throughout the biome.
    #[default]
    #[serde(rename = "none")]
    None,
    /// Pockets of warm temperature to be randomly distributed throughout the biome.
    #[serde(rename = "frozen")]
    Frozen
}

/// Biome special effects.
#[derive(Ser, Deser, Debug, SynDebug)]
pub struct WorldgenBiomeEffects<'l> {
    /// The colour of the fog when looking past the view distance.
    #[serde(rename = "fog_color")]
    pub fog_colour            : Rgb,
    /// The colour of water.
    #[serde(rename = "water_color")]
    pub water_color           : Rgb,
    /// The colour of the fog when lookin past the view distance while underwater.
    #[serde(rename = "water_fog_color")]
    pub water_fog_colour      : Rgb,
    /// The colour of the sky.
    #[serde(rename = "sky_color")]
    pub sky_colour            : Rgb,
    /// The colour of leaves.
    ///
    /// If not specified, This colour is calculated based on temperature and downfall.
    #[serde(rename = "foliage_color", skip_serializing_if = "Option::is_none")]
    pub foliage_colour        : Option<Rgb>,
    /// The colour of grass.
    ///
    /// If not specified, This colour is calculated based on temperature and downfall.
    #[serde(rename = "grass_color", skip_serializing_if = "Option::is_none")]
    pub grass_colour          : Option<Rgb>,
    /// Modifier that affects the resulting grass colour.
    #[serde(rename = "grass_color_modifier", skip_serializing_if = "is_default", default)]
    pub grass_colour_modifier : WorldgenBiomeGrassColourModifier,
    /// Ambient particles.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub particle              : Option<WorldgenBiomeParticle>,
    /// Ambient soundtrack that starts playing when entering the biome, and fades out when exiting.
    #[serde(skip_serializing_if = "Option::is_none", deserialize_with = "deser_ambient_sound", default)]
    pub ambient_sound         : Option<WorldgenBiomeAmbientSound>,
    /// Additional ambient sound that plays in moody situations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mood_sound            : Option<WorldgenBiomeMoodSound>,
    /// Additional ambient sound that has a chance of playing randomly every tick.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additions_sound       : Option<WorldgenBiomeAdditionsSound>,
    /// Music that plays in the biome.
    #[serde(skip_serializing_if = "slice_is_empty", default)]
    pub music                 : Cow<'l, [Weighted<WorldgenBiomeMusic>]>
}

/// Modifier for grass colour in a biome.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Ser, Deser, Debug, SynDebug)]
pub enum WorldgenBiomeGrassColourModifier {
    /// Static grass colour throughout the biome.
    #[default]
    #[serde(rename = "none")]
    None,
    /// Darker, less saturated shade.
    #[serde(rename = "dark_forest")]
    DarkForest,
    /// Colour overridden with `#4C763C` and `#6A7039`, randomly distributed throughout the biome.
    #[serde(rename = "swamp")]
    Swamp
}

/// Ambient biome particles that show while in a biome.
#[derive(Ser, Deser, Debug, SynDebug)]
pub struct WorldgenBiomeParticle {
    /// The particle to display.
    #[serde(rename = "options")]
    pub particle    : Particle,
    /// The chance for the particle to spawn.
    pub probability : f32
}

/// Ambient soundtrack that plays while in a biome.
#[derive(Ser, Deser, Debug, SynDebug)]
pub struct WorldgenBiomeAmbientSound {
    /// Resource ID of the sound asset to play.
    #[serde(rename = "sound_id")]
    pub sound : Ident,
    /// The range of the sound.
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

/// Additional ambient sound that plays in moody situations while in a biome.
#[derive(Ser, Deser, Debug, SynDebug)]
pub struct WorldgenBiomeMoodSound {
    /// Resource ID of the sound asset to play.
    pub sound               : Ident,
    /// The rate at which moodiness increases, and minimum time between plays.
    pub tick_delay          : u32,
    /// Radius to search for moodiness calculation.
    pub block_search_extent : u32,
    /// The distance offset from the player when playing the sound.
    pub offset              : f64
}

/// Additional ambient sound that plays randomly while in a biome.
#[derive(Ser, Deser, Debug, SynDebug)]
pub struct WorldgenBiomeAdditionsSound {
    /// Resource ID of the sound asset to play.
    pub sound       : Ident,
    /// The change to play this sound each tick.
    pub tick_chance : f64
}

/// Music that plays while in a biome.
#[derive(Clone, Ser, Deser, Debug, SynDebug)]
pub struct WorldgenBiomeMusic {
    /// Resource ID of the sound asset to play.
    pub sound           : Ident,
    /// Minimum delay between plays.
    pub min_delay       : u32,
    /// Maximum delay between plays.
    pub max_delay       : u32,
    /// Whether this track can replace the previous track.
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
