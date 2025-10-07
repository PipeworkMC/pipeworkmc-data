//! Block positions.


use crate::{
    chunk_pos::ChunkPos,
    ident::Ident
};
use pipeworkmc_codec::{
    decode::{
        PacketDecode,
        DecodeIter,
        IncompleteDecodeError
    },
    encode::{
        PacketEncode,
        EncodeBuf
    }
};


/// The position of a block in a dimension.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct BlockPos {
    /// X
    pub x : i32,
    /// Y
    pub y : i32,
    /// Z
    pub z : i32
}

impl BlockPos {
    /// A [`BlockPos`] with all lanes set to 0.
    pub const ZERO : Self = Self { x : 0, y : 0, z : 0 };

    /// Returns the position of the chunk that this block position falls into.
    pub const fn chunk(self) -> ChunkPos { ChunkPos {
        x : self.x.div_euclid(16),
        z : self.z.div_euclid(16)
    } }
}

impl PacketDecode for BlockPos {
    type Error = IncompleteDecodeError;
    fn decode<I>(iter : &mut DecodeIter<I>) -> Result<Self, Self::Error>
    where
        I : ExactSizeIterator<Item = u8>
    {
        let v = <u64>::decode(iter)?;
        Ok(Self {
            x : ((v >> 38) & 0x3FFFFFF) as i32,
            z : ((v >> 12) & 0x3FFFFFF) as i32,
            y : (v & 0xFFF) as i32
        })
    }
}

unsafe impl PacketEncode for BlockPos {

    #[inline]
    fn encode_len(&self) -> usize { 8 }

    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        ((((self.x as u64) & 0x3FFFFFF) << 38) | (((self.z as u64) & 0x3FFFFFF) << 12) | ((self.y as u64) & 0xFFF))
            .encode(buf);
    } }

}


/// The dimension and position of a block.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct DimBlockPos {
    /// The dimension.
    pub dim : Ident,
    /// The position.
    pub pos : BlockPos
}

unsafe impl PacketEncode for DimBlockPos {

    #[inline]
    fn encode_len(&self) -> usize {
        self.dim.encode_len()
        + self.pos.encode_len()
    }

    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        self.dim.encode(buf);
        self.pos.encode(buf);
    } }

}
