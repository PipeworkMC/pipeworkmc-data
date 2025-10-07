use crate::chunk_pos::ChunkPos;


/// The position of a character.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
#[cfg_attr(feature = "bevy", derive(bevy_ecs::component::Component))]
pub struct CharacterPos {
    /// X
    pub x : f64,
    /// Y
    pub y : f64,
    /// Z
    pub z : f64
}

impl CharacterPos {
    /// A [`CharacterPos`] with all lanes set to 0.0.
    pub const ZERO : Self = Self { x : 0.0, y : 0.0, z : 0.0 };

    /// Returns the position of the chunk that this character position falls into.
    pub fn chunk(self) -> ChunkPos { ChunkPos {
        x : self.x.div_euclid(16.0) as i32,
        z : self.z.div_euclid(16.0) as i32
    } }
}


/// The rotation of a character.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
#[cfg_attr(feature = "bevy", derive(bevy_ecs::component::Component))]
pub struct CharacterRot {
    /// Pitch
    pub pitch    : f64,
    /// Yaw
    pub yaw      : f64,
    /// Head yaw
    pub head_yaw : f64
}

impl CharacterRot {
    /// A [`CharacterRot`] with all lanes set to 0.0 (South).
    pub const ZERO : Self = Self { pitch : 0.0, yaw : 0.0, head_yaw : 0.0 };
}


/// The velocity of a character.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
#[cfg_attr(feature = "bevy", derive(bevy_ecs::component::Component))]
pub struct CharacterVel {
    /// X
    pub x : f64,
    /// Y
    pub y : f64,
    /// Z
    pub z : f64
}

impl CharacterVel {
    /// A [`CharacterVel`] with all lanes set to 0.0.
    pub const ZERO : Self = Self { x : 0.0, y : 0.0, z : 0.0 };
}
