//! Modded channels for custom data.


use crate::ident::{
    Ident,
    IdentDecodeError
};
use pipeworkmc_codec::{
    decode::{
        PacketDecode,
        DecodeIter,
        string::StringDecodeError
    },
    encode::{
        PacketEncode,
        EncodeBuf
    }
};
use core::fmt::{ self, Display, Formatter };
use std::borrow::Cow;


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


impl PacketDecode for ChannelData<'_> {
    type Error = ChannelDataDecodeError;

    fn decode<I>(iter : &mut DecodeIter<I>) -> Result<Self, Self::Error>
    where
        I : ExactSizeIterator<Item = u8>
    {
        let channel = Ident::decode(iter).map_err(ChannelDataDecodeError::Channel)?;
        Ok(if (channel == CHANNEL_BRAND) {
            Self::Brand { brand : Cow::Owned(<_>::decode(iter).map_err(ChannelDataDecodeError::Brand)?) }
        } else {
            Self::Custom { channel, data : Cow::Owned(iter.collect::<Vec<_>>().to_vec()) }
        })
    }
}

unsafe impl PacketEncode for ChannelData<'_> {

    fn encode_len(&self) -> usize { match (self) {
        Self::Brand { brand } => {
            CHANNEL_BRAND.encode_len()
            + brand.encode_len()
        },
        Self::Custom { channel, data } => {
            channel.encode_len()
            + data.len()
        }
    } }

    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe { match (self) {
        Self::Brand { brand } => {
            CHANNEL_BRAND.encode(buf);
            brand.encode(buf);
        },
        Self::Custom { channel, data } => {
            channel.encode(buf);
            buf.write_slice(data);
        }
    } } }

}


/// Returned by packet decoders when a `ChannelData` was not decoded successfully.
#[derive(Debug)]
pub enum ChannelDataDecodeError {
    /// The channel ID failed to decode.
    Channel(IdentDecodeError),
    /// A `minecraft:brand` payload failed to decode.
    Brand(StringDecodeError)
}

impl Display for ChannelDataDecodeError {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result { match (self) {
        Self::Channel(err) => write!(f, "channel {err}"),
        Self::Brand(err)   => write!(f, "brand {err}")
    } }
}
