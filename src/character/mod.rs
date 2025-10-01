//! Character data.


use bevy_ecs::component::Component;


#[cfg(feature = "generated")]
include!("../../../pipeworkmc-vanilla-datagen/output/generated/entity_type.rs");

mod pos;
pub use pos::*;


/// A character's networked ID used to track and update the character later.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Component)]
pub struct CharacterId(pub u32);
