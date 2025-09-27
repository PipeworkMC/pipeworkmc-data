//! Stacks of items.


use crate::ident::Ident;
use serde::{
    Serialize as Ser,
    Deserialize as Deser
};
use syndebug::SynDebug;


/// A stack of items.
#[derive(Clone, Ser, Deser, Debug, SynDebug)]
pub struct ItemStack {
    /// The type of item.
    pub id    : Ident, // TODO: Replace this with an item enum.
    /// The number of items in this stack.
    pub count : u32,
    // TODO: components
}
