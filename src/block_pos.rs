//! Block positions.


use crate::{
    Minecraft,
    chunk_pos::ChunkPos,
    ident::Ident
};
use netzer::prelude::*;


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

impl NetEncode<Minecraft> for BlockPos {
    async fn encode<W : netzer::AsyncWrite>(&self, w : W) -> netzer::Result {
        <u64 as NetEncode<Minecraft>>::encode(
            &((((self.x as u64) & 0x3FFFFFF) << 38)
                | (((self.z as u64) & 0x3FFFFFF) << 12)
                | ((self.y as u64) & 0xFFF)),
            w
        ).await
    }
}

impl NetDecode<Minecraft> for BlockPos {
    async fn decode<R : netzer::AsyncRead>(r : R) -> netzer::Result<Self> {
        let v = <u64 as NetDecode<Minecraft>>::decode(r).await?;
        Ok(Self {
            x : ((v >> 38) & 0x3FFFFFF) as i32,
            z : ((v >> 12) & 0x3FFFFFF) as i32,
            y : (v & 0xFFF) as i32
        })
    }
}


/// The dimension and position of a block.
#[derive(Clone, PartialEq, Eq, Hash, Debug, NetEncode, NetDecode)]
pub struct DimBlockPos {
    /// The dimension.
    pub dim : Ident,
    /// The position.
    pub pos : BlockPos
}
