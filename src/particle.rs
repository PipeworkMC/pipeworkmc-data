//! Particles.


use serde::{
    Serialize as Ser,
    Deserialize as Deser
};
use syndebug::SynDebug;


/// A particle.
#[derive(Ser, Deser, Debug, SynDebug)]
#[serde(tag = "type")]
pub enum Particle {

    /// `minecraft:ash`
    #[serde(rename = "minecraft:ash", alias = "ash")]
    Ash,

    /// `minecraft:crimson_spore`
    #[serde(rename = "minecraft:crimson_spore", alias = "crimson_spore")]
    CrimsonSpore,

    /// `minecraft:warped_spore`
    #[serde(rename = "minecraft:warped_spore", alias = "warped_spore")]
    WarpedSpore,

    /// `minecraft:white_ash`
    #[serde(rename = "minecraft:white_ash", alias = "white_ash")]
    WhiteAsh

}
