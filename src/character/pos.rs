use crate::chunk_pos::ChunkPos;
use netzer::prelude::*;


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


/// The movement flags of a character.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "bevy", derive(bevy_ecs::component::Component))]
pub struct CharacterMoveFlags {
    /// Whether the character is on the ground.
    pub on_ground    : bool,
    /// Whether the character is pushing against a wall.
    pub against_wall : bool
}

impl NetEncode<crate::Minecraft> for CharacterMoveFlags {
    async fn encode<W : netzer::AsyncWrite>(&self, mut writer : W) -> netzer::Result {
        writer.write_all(&[
            (if (self.on_ground) { 0b00000001 } else { 0u8 })
            | (if (self.against_wall) { 0b00000010 } else { 0u8 })
        ]).await?;
        Ok(())
    }
}

impl NetDecode<crate::Minecraft> for CharacterMoveFlags {
    async fn decode<R : netzer::AsyncRead>(mut reader : R) -> netzer::Result<Self> {
        let mut buf = [0u8];
        reader.read_exact(&mut buf).await?;
        let b = buf[0];
        Ok(Self {
            on_ground    : (b & 0b00000001) != 0,
            against_wall : (b & 0b00000010) != 0
        })
    }
}
