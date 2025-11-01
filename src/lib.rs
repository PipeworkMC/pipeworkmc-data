#![doc = include_str!("../README.md")]


#![feature(

    // Language
    const_precise_live_drops,

    // Syntax
    decl_macro,

    // Standard Library
    maybe_uninit_slice

)]


mod format;
pub use format::*;

pub mod action;
pub mod angle;
pub mod banner_pattern;
pub mod block_pos;
pub mod bounded_string;
pub mod box_cow;
pub mod cat_variant;
pub mod channel_data;
pub mod character;
pub mod chicken_variant;
pub mod chunk_pos;
pub mod client_info;
pub mod colour;
pub mod cow_variant;
pub mod damage_type;
pub mod dialog;
pub mod difficulty;
pub mod dimension_type;
pub mod frog_variant;
pub mod game_mode;
pub mod hand;
pub mod ident;
pub mod item_stack;
pub mod known_pack;
pub mod light_level;
pub mod nbt;
pub mod num;
pub mod operator_level;
pub mod pack_status;
pub mod painting_variant;
pub mod particle;
pub mod pig_variant;
pub mod profile;
pub mod redacted;
pub mod registry_entry;
pub mod selected_hotbar;
pub mod tag_ident;
pub mod text;
pub mod version;
pub mod wolf_variant;
pub mod wolf_sound_variant;
pub mod worldgen;
pub use uuid;
pub use netzer::varint;

pub use syndebug;
pub use disqualified;


#[inline]
pub(crate) fn slice_is_empty<T>(slice : &[T]) -> bool {
    slice.is_empty()
}

#[inline]
pub(crate) fn is_default<T>(value : &T) -> bool
where
    T : Default + PartialEq
{ *value == T::default() }
