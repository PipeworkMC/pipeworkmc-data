//! Character data.


use pipeworkmc_codec::{
    decode::{
        PacketDecode,
        DecodeIter
    },
    varint::{
        VarInt,
        VarIntDecodeError
    }
};


#[cfg(feature = "generated")]
include!("../../../pipeworkmc-vanilla-datagen/output/generated/entity_type.rs");

mod pos;
pub use pos::*;


/// A character's networked ID used to track and update the character later.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct CharacterId(pub u32);

impl PacketDecode for CharacterId {
    type Error = VarIntDecodeError;

    #[inline]
    fn decode<I>(iter : &mut DecodeIter<I>) -> Result<Self, Self::Error>
    where
        I : ExactSizeIterator<Item = u8>
    { Ok(Self(<VarInt<u32>>::decode(iter)?.0)) }
}
