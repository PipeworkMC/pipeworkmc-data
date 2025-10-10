//! Data packs present in the game.


use std::borrow::Cow;
use netzer::prelude::*;


/// A data pack present in the game.
#[derive(Debug, Clone, NetEncode, NetDecode)]
pub struct KnownPack<'l> {
    /// The namespace of the pack.
    pub namespace : Cow<'l, str>,
    /// The ID of the pack.
    pub id        : Cow<'l, str>,
    /// The game version of the pack.
    pub version   : Cow<'l, str>
}
