//! Weighted values.


use serde::{
    Serialize as Ser,
    Deserialize as Deser
};
use syndebug::SynDebug;


/// A weighted value.
#[derive(Clone, Ser, Deser, Debug, SynDebug)]
pub struct Weighted<T>
where
    T : Clone + SynDebug
{
    /// Value of this entry.
    pub data   : T,
    /// Weight of this entry.
    pub weight : u32
}
