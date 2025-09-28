//! Dimension type registry entries.


use crate::{
    ident::Ident,
    light_level::{ LightLevel, LightLevelProvider },
    nbt::to_network as to_network_nbt,
    num::multiple16::{
        Multiple16I32,
        Multiple16U32
    },
    registry_entry::*,
    tag_ident::TagIdent
};
#[cfg(feature = "generated")]
use crate::num::{
    multiple16::Multiple16,
    provider::IntProvider
};
use std::io::Write;
use serde::{
    Serialize as Ser,
    Deserialize as Deser
};
use syndebug::SynDebug;


/// A dimension type registry entry.
#[derive(Ser, Deser, Debug, SynDebug)]
#[serde(deny_unknown_fields)]
pub struct DimensionType<'l> {
    /// Whether the time (sun/moon position and brightness) does not change.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_time                      : Option<u64>,
    /// Whether the sky should 'emit' light.
    pub has_skylight                    : bool,
    /// Whether the dimension has a roof.
    pub has_ceiling                     : bool,
    /// Whether water evaporates, sponges dry, and lava spreads thinner.
    ///
    /// Unused by pipework.
    #[serde(rename = "ultrawarm")]
    pub is_ultrawarm                    : bool,
    /// When `true`, nether portals spawn zombified piglins.
    /// When `false`, compasses spin randomly.
    ///
    /// Unused by pipework.
    #[serde(rename = "natural")]
    pub is_natural                      : bool,
    /// At what height clouds are rendered, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_height                    : Option<i32>,
    /// The multiplier applied to coordinates when travelling to this dimension.
    ///
    /// Unused by pipework.
    #[serde(rename = "coordinate_scale")]
    pub coord_scale                     : f64,
    /// Whether players can sleep in a bed.
    #[serde(rename = "bed_works")]
    pub beds_work                       : bool,
    /// Whether players can use respawn anchors.
    #[serde(rename = "respawn_anchor_works")]
    pub anchors_work                    : bool,
    /// The minimum Y level.
    pub min_y                           : Multiple16I32,
    /// The height of the dimension, starting at `min_y`.
    pub height                          : Multiple16U32,
    /// The logical height of the dimension, starting at `min_y`.
    /// Vanilla servers use this as a maximum height for chorus fruit and nether portal spawns.
    ///
    /// Unused by pipework.
    pub logical_height                  : Multiple16U32,
    /// The resource location defining what block to use for infiniburn.
    ///
    /// Unused by pipework.
    #[serde(rename = "infiniburn")]
    pub infiniburn_tag                  : TagIdent,
    /// Defines special dimensional effects.
    pub effects                         : DimensionTypeEffects,
    /// How much light the dimension has.
    pub ambient_light                   : f32,
    #[serde(rename = "piglin_safe")]
    /// Whether piglins shake and transform into zombified piglins.
    pub is_piglin_safe                  : bool,
    /// whether players with the Bad Omen status effect will start raids.
    ///
    /// Unused by pipework.
    pub has_raids                       : bool,
    /// The maximum light level for monster spawn attempts.
    ///
    /// Unused by pipework.
    pub monster_spawn_light_level       : LightLevelProvider<'l>,
    /// The maximum block light level for monster spawn attempts.
    ///
    /// Unused by pipework.
    pub monster_spawn_block_light_limit : LightLevel
}

/// Special dimensional effects.
#[derive(Ser, Deser, Debug, SynDebug)]
#[serde(deny_unknown_fields)]
pub enum DimensionTypeEffects {
    /// Normal sky time, normal light map, normal ambient light.
    #[serde(rename = "minecraft:overworld", alias = "overworld")]
    Overworld,
    /// Nether sky type, normal light map, constant ambient light.
    #[serde(rename = "minecraft:the_nether", alias = "the_nether")]
    Nether,
    /// End sky type, forced light map, normal ambient light.
    #[serde(rename = "minecraft:the_end", alias = "the_end")]
    End
}


#[cfg(feature = "generated")]
include!("../../pipeworkmc-vanilla-datagen/output/generated/dimension_type.rs");


impl RegistryEntryType for DimensionType<'_> {
    const REGISTRY_ID : Ident = Ident::new("minecraft:dimension_type");

    fn to_network_nbt<W>(&self, writer : W) -> bool
    where
        W : Write
    {
        to_network_nbt(writer, self).unwrap();
        true
    }
}
