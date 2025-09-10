use serde::{
    Serialize as Ser,
    Deserialize as Deser
};
use syndebug::SynDebug;


#[derive(Clone, Ser, Deser, Debug, SynDebug)]
pub struct Weighted<T>
where
    T : Clone + SynDebug
{
    pub data   : T,
    pub weight : u32
}
