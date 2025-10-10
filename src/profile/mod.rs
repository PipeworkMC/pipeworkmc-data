//! Player account profiles.


use crate::{
    bounded_string::BoundedString,
    uuid::Uuid
};
use netzer::prelude::*;
use serde::Deserialize as Deser;


mod deser;


/// A player account profile.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy", derive(bevy_ecs::component::Component))]
#[cfg_attr(feature = "bevy", component(immutable))]
pub struct AccountProfile {
    /// The player account UUID.
    pub uuid     : Uuid,
    /// The player account username.
    pub username : BoundedString<16>,
    /// The player's skin, if they have one.
    pub skin     : Option<AccountProperty>
}

/// A player account property.
#[derive(Clone, Deser, Debug)]
pub struct AccountProperty {
    /// The value of the property.
    pub value : String,
    /// The signature of the property.
    #[serde(rename = "signature")]
    pub sig   : Option<String>,
}


unsafe impl PacketEncode for AccountProfile {

    fn encode_len(&self) -> usize {
        self.uuid.encode_len()
        + self.username.encode_len()
        + KeyedAccountProperties([
            KeyedAccountProperty { key : "textures", property : self.skin.as_ref() }
        ]).encode_len()
    }

    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        self.uuid.encode(buf);
        self.username.encode(buf);
        KeyedAccountProperties([
            KeyedAccountProperty { key : "textures", property : self.skin.as_ref() }
        ]).encode(buf);
    } }

}


struct KeyedAccountProperties<'l> ([KeyedAccountProperty<'l>; 1]);

unsafe impl PacketEncode for KeyedAccountProperties<'_> {

    fn encode_len(&self) -> usize {
        let len = self.0.iter().filter(|p| p.property.is_some()).count();
        VarInt::<u32>(len as u32).encode_len()
        + self.0.iter().map(|p| p.encode_len()).sum::<usize>()
    }

    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        let len = self.0.iter().filter(|p| p.property.is_some()).count();
        VarInt::<u32>(len as u32).encode(buf);
        for p in &self.0 {
            p.encode(buf);
        }
    } }

}


struct KeyedAccountProperty<'l> {
    key      : &'static str,
    property : Option<&'l AccountProperty>
}

unsafe impl PacketEncode for KeyedAccountProperty<'_> {

    fn encode_len(&self) -> usize {
        if let Some(property) = self.property {
            self.key.encode_len()
            + property.value.encode_len()
            + property.sig.encode_len()
        } else { 0 }
    }

    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        if let Some(property) = self.property {
            self.key.encode(buf);
            property.value.encode(buf);
            property.sig.encode(buf);
        }
    } }

}


#[derive(Deser)]
enum AccountPropertyKey {
    #[serde(rename = "textures")]
    Skin
}
