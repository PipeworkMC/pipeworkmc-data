//! Chunk positions.


/// The position of a chunk in a dimension.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ChunkPos {
    /// X
    pub x : i32,
    /// Z
    pub z : i32
}

impl ChunkPos {
    /// A [`ChunkPos`] with all lanes set to 0.
    pub const ZERO : Self = Self { x : 0, z : 0 };

    /// Gets the shortest distance along an axis.
    pub const fn cardinal_dist(self, other : Self) -> u32 {
        let x = self.x.abs_diff(other.x);
        let z = self.z.abs_diff(other.z);
        if (x < z) { x } else { z }
    }

}
