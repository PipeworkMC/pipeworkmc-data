//! World difficulties.


use netzer::prelude::*;


/// A world difficulty.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, NetEncode, NetDecode)]
#[netzer(ordinal)]
#[repr(u8)]
pub enum Difficulty {
    /// Peaceful
    Peaceful = 0,
    /// Easy
    Easy     = 1,
    /// Normal
    #[default]
    Normal   = 2,
    /// Hard
    Hard     = 3
}
