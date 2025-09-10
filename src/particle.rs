use serde::{
    Serialize as Ser,
    Deserialize as Deser
};
use syndebug::SynDebug;


#[derive(Ser, Deser, Debug, SynDebug)]
#[serde(tag = "type")]
pub enum Particle {

    #[serde(rename = "minecraft:ash", alias = "ash")]
    Ash,

    #[serde(rename = "minecraft:crimson_spore", alias = "crimson_spore")]
    CrimsonSpore,

    #[serde(rename = "minecraft:warped_spore", alias = "warped_spore")]
    WarpedSpore,

    #[serde(rename = "minecraft:white_ash", alias = "white_ash")]
    WhiteAsh

}
