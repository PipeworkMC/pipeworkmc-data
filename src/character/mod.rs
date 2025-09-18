use core::sync::atomic::{
    AtomicU32,
    Ordering as AtomicOrdering
};
use bevy_ecs::{
    component::Component,
    resource::Resource
};


#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Component)]
pub struct CharacterId(u32);

impl CharacterId {

    pub const ZERO : Self = Self(0);

    pub const fn new(id : u32) -> Self { Self(id) }

    pub const fn as_u32(&self) -> u32 { self.0 }

}

impl From<u32> for CharacterId {
    fn from(id : u32) -> Self { Self::new(id) }
}


#[derive(Resource, Default)]
pub struct NextCharacterId(AtomicU32);

impl Iterator for NextCharacterId {
    type Item = CharacterId;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        Some(NextCharacterId::next(self))
    }
}

impl NextCharacterId {
    #[expect(clippy::should_implement_trait)]
    pub fn next(&self) -> CharacterId {
        CharacterId(self.0.fetch_add(1, AtomicOrdering::Relaxed))
    }
}
