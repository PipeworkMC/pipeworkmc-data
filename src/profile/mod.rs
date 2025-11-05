//! Player account profiles.


use crate::{
    Minecraft,
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


impl NetEncode<Minecraft> for AccountProfile {
    async fn encode<W : netzer::AsyncWrite>(&self, mut w : W) -> netzer::Result {
        <Uuid as NetEncode<Minecraft>>::encode(&self.uuid, &mut w).await?;
        <BoundedString<16> as NetEncode<Minecraft>>::encode(&self.username, &mut w).await?;

        let properties = [
            ("textures", self.skin.as_ref())
        ];
        let property_count = properties.iter().filter(|(_, p,)| p.is_some()).count();
        <VarInt<u32> as NetEncode<Minecraft>>::encode(&VarInt(property_count as u32), &mut w).await?;
        for (key, property,) in properties { if let Some(property) = property {
            <str as NetEncode<Minecraft>>::encode(key, &mut w).await?;
            <str as NetEncode<Minecraft>>::encode(&property.value, &mut w).await?;
            <Option<String> as NetEncode<Minecraft>>::encode(&property.sig, &mut w).await?;
        } }

        Ok(())
    }
}

impl NetDecode<Minecraft> for AccountProfile {
    async fn decode<R : netzer::AsyncRead>(mut r : R) -> netzer::Result<Self> {
        let     uuid     = <Uuid as NetDecode<Minecraft>>::decode(&mut r).await?;
        let     username = <BoundedString<16> as NetDecode<Minecraft>>::decode(&mut r).await?;
        let mut skin     = None;

        let property_count = usize::try_from(<VarInt<u32> as NetDecode<Minecraft>>::decode(&mut r).await?.0)?;
        for _ in 0..property_count {
            let key   = <String as NetDecode<Minecraft>>::decode(&mut r).await?;
            let value = <String as NetDecode<Minecraft>>::decode(&mut r).await?;
            let sig   = <Option<String> as NetDecode<Minecraft>>::decode(&mut r).await?;
            match (key.as_str()) {
                "textures" => { skin = Some(AccountProperty { value, sig }) },
                _ => { }
            }
        }

        Ok(Self {
            uuid,
            username,
            skin
        })
    }
}
