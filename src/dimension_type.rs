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


#[derive(Ser, Deser, Debug, SynDebug)]
#[serde(deny_unknown_fields)]
pub struct DimensionType<'l> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_time                      : Option<u64>,
    pub has_skylight                    : bool,
    pub has_ceiling                     : bool,
    #[serde(rename = "ultrawarm")]
    pub is_ultrawarm                    : bool,
    #[serde(rename = "natural")]
    pub is_natural                      : bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_height                    : Option<i32>,
    #[serde(rename = "coordinate_scale")]
    pub coord_scale                     : f64,
    #[serde(rename = "bed_works")]
    pub beds_work                       : bool,
    #[serde(rename = "respawn_anchor_works")]
    pub anchors_work                    : bool,
    pub min_y                           : Multiple16I32,
    pub height                          : Multiple16U32,
    pub logical_height                  : Multiple16U32,
    #[serde(rename = "infiniburn")]
    pub infiniburn_tag                  : TagIdent,
    pub effects                         : DimensionTypeEffects,
    pub ambient_light                   : f32,
    #[serde(rename = "piglin_safe")]
    pub is_piglin_safe                  : bool,
    pub has_raids                       : bool,
    pub monster_spawn_light_level       : LightLevelProvider<'l>,
    pub monster_spawn_block_light_limit : LightLevel
}

#[derive(Ser, Deser, Debug, SynDebug)]
#[serde(deny_unknown_fields)]
pub enum DimensionTypeEffects {
    #[serde(rename = "minecraft:overworld", alias = "overworld")]
    Overworld,
    #[serde(rename = "minecraft:the_nether", alias = "the_nether")]
    Nether,
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
