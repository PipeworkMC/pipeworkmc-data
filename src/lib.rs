#![feature(

    // Language
    const_precise_live_drops,

    // Syntax
    decl_macro

)]


pub mod action;
pub mod block_pos;
pub mod bounded_string;
pub mod box_cow;
pub mod channel_data;
pub mod character;
pub mod client_info;
pub mod colour;
pub mod dialog;
pub mod game_mode;
pub mod ident;
pub mod itemstack;
pub mod light_level;
pub mod nbt;
pub mod num;
pub mod particle;
pub mod profile;
pub mod redacted;
pub mod registry_entry;
pub mod tag_ident;
pub mod text;
pub use pipeworkmc_codec::uuid;
pub use pipeworkmc_codec::varint;


#[inline(always)]
pub(crate) fn slice_is_empty<T>(slice : &[T]) -> bool {
    slice.is_empty()
}

#[inline(always)]
pub(crate) fn is_default<T>(value : &T) -> bool
where
    T : Default + PartialEq
{ *value == T::default() }
