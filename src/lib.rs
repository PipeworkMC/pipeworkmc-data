#![doc = include_str!("../README.md")]


#![feature(

    // Language
    const_precise_live_drops,

    // Syntax
    decl_macro,

    // Standard Library
    maybe_uninit_slice

)]


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
pub use netzer::varint::VarInt;

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


#[non_exhaustive]
pub struct Minecraft;
impl netzer::NetFormat for Minecraft { }
macro_rules! impl_netendecode_minecraft_for {
    ( $ty:ty $(,)? ) => {
        impl netzer::NetEncode<Minecraft> for $ty {
            #[inline(always)]
            fn encode<W : netzer::AsyncWrite>(&self, w : W) -> impl Future<Output = netzer::Result> {
                <$ty as netzer::NetEncode::<netzer::numeric::BigEndian>>::encode(self, w)
            }
        }
        impl netzer::NetDecode<Minecraft> for $ty {
            #[inline(always)]
            fn decode<R : netzer::AsyncRead>(r : R) -> impl Future<Output = netzer::Result<Self>> {
                <$ty as netzer::NetDecode::<netzer::numeric::BigEndian>>::decode(r)
            }
        }
    };
}
impl_netendecode_minecraft_for!(bool);
impl_netendecode_minecraft_for!(u8);
impl_netendecode_minecraft_for!(i8);
impl_netendecode_minecraft_for!(u16);
impl_netendecode_minecraft_for!(i16);
impl_netendecode_minecraft_for!(u32);
impl_netendecode_minecraft_for!(i32);
impl_netendecode_minecraft_for!(u64);
impl_netendecode_minecraft_for!(i64);
impl_netendecode_minecraft_for!(u128);
impl_netendecode_minecraft_for!(i128);
