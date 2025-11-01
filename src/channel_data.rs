//! Modded channels for custom data.


use crate::{
    Minecraft,
    ident::Ident
};
use std::borrow::Cow;
use netzer::prelude::*;


const CHANNEL_BRAND : Ident = Ident::new("minecraft:brand");


/// Custom data sent over modded channels.
#[derive(Debug)]
pub enum ChannelData<'l> {
    /// The `minecraft:brand` channel used by the vanilla game to tell the peer what distribution of the game they are playing on.
    Brand {
        /// The brand (Vanilla, fabric, forge, etc).
        brand : Cow<'l, str>
    },
    /// Some other channel unused by the vanilla game.
    Custom {
        /// The channel ID.
        channel : Ident,
        /// The payload data.
        data    : Cow<'l, [u8]>
    }
}


impl NetEncode<Minecraft> for ChannelData<'_> {
    async fn encode<W : netzer::AsyncWrite>(&self, mut w : W) -> netzer::Result {
        match (self) {
            ChannelData::Brand { brand } => {
                <Ident as NetEncode<Minecraft>>::encode(&CHANNEL_BRAND, &mut w).await?;
                <str as NetEncode<Minecraft>>::encode(brand, w).await?;
            },
            ChannelData::Custom { channel, data } => {
                <Ident as NetEncode<Minecraft>>::encode(channel, &mut w).await?;
                w.write_all(data).await?;
            }
        }
        Ok(())
    }
}

impl NetDecode<Minecraft> for ChannelData<'_> {
    async fn decode<R : netzer::AsyncRead>(mut r : R) -> netzer::Result<Self> {
        let channel = <Ident as NetDecode<Minecraft>>::decode(&mut r).await?;
        Ok(if (channel == CHANNEL_BRAND) {
            Self::Brand { brand : <Cow<'_, str> as NetDecode<Minecraft>>::decode(r).await? }
        } else {
            todo!()
        })
    }
}
