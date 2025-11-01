//! Player selected hotbar slots.


use crate::Minecraft;
use core::{
    error::Error as StdError,
    fmt::{ self, Display, Formatter }
};
use netzer::prelude::*;


/// A player's selected hotbar slot.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, NetEncode, NetDecode)]
#[cfg_attr(feature = "bevy", derive(bevy_ecs::component::Component))]
#[repr(transparent)]
pub struct SelectedHotbar(
    #[netzer(convert = "u16", decode_with = "decode_slot", try_from)]
    u8
);
async fn decode_slot<R : netzer::AsyncRead>(r : R) -> netzer::Result<u16> {
    let v = <u16 as NetDecode<Minecraft>>::decode(r).await?;
    if (v >= 9) { return Err(BadHotbarSlot(v).into()); }
    Ok(v)
}


/// The hotbar slot was out of range.
#[derive(Debug)]
pub struct BadHotbarSlot(u16);
impl StdError for BadHotbarSlot { }
impl Display for BadHotbarSlot {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result {
        write!(f, "bad hotbar slot `{}`", self.0)
    }
}
