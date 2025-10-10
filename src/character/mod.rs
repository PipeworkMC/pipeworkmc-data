//! Character data.


use netzer::prelude::*;


#[cfg(feature = "generated")]
include!("../../../pipeworkmc-vanilla-datagen/output/generated/entity_type.rs");

mod pos;
pub use pos::*;


/// A character's networked ID used to track and update the character later.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, NetEncode, NetDecode)]
pub struct CharacterId(
    #[netzer(format = "Leb128", convert = "VarInt<u32>")]
    pub u32
);
