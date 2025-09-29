//! Character data.


use core::sync::atomic::{
    AtomicU32,
    Ordering as AtomicOrdering
};
use bevy_ecs::component::Component;


#[cfg(feature = "generated")]
include!("../../../pipeworkmc-vanilla-datagen/output/generated/entity_type.rs");

mod pos;
pub use pos::*;


static NEXT_CHARACTER_ID : AtomicU32 = AtomicU32::new(0);


/// A character's networked ID used to track and update the character later.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Component)]
pub struct CharacterId(u32);

impl CharacterId {

    /// Create a new [`CharacterId`] with the next character ID.
    #[inline]
    pub fn next() -> Self { Self(NEXT_CHARACTER_ID.fetch_add(1, AtomicOrdering::Relaxed)) }

    /// Returns the inner value as a primitive type.
    #[inline]
    pub const fn as_u32(&self) -> u32 { self.0 }

}

impl Default for CharacterId {
    #[inline]
    fn default() -> Self { Self::next() }
}
