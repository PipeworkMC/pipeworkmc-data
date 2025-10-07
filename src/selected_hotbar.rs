//! Player selected hotbar slots.


use pipeworkmc_codec::decode::{
    PacketDecode,
    DecodeIter,
    IncompleteDecodeError
};
use core::fmt::{ self, Display, Formatter };


/// A player's selected hotbar slot.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
#[cfg_attr(feature = "bevy", derive(bevy_ecs::component::Component))]
#[repr(transparent)]
pub struct SelectedHotbar(u8);


impl PacketDecode for SelectedHotbar {
    type Error = SelectedHotbarDecodeError;

    fn decode<I>(iter : &mut DecodeIter<I>) -> Result<Self, Self::Error>
    where
        I : ExactSizeIterator<Item = u8>
    {
        let v = <u16>::decode(iter).map_err(SelectedHotbarDecodeError::Slot)?;
        if (v < 9) { Ok(Self(v as u8)) }
        else { Err(SelectedHotbarDecodeError::BadSlot(v)) }
    }
}

/// Returned by packet decoders when a `SelectedHotbar` was not decoded successfully.
#[derive(Debug)]
pub enum SelectedHotbarDecodeError {
    /// The slot failed to decode.
    Slot(IncompleteDecodeError),
    /// An unknown slot was found.
    BadSlot(u16)
}
impl Display for SelectedHotbarDecodeError {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result { match (self) {
        Self::Slot(err)  => write!(f, "slot {err}"),
        Self::BadSlot(v) => write!(f, "bad slot {v}")
    } }
}
