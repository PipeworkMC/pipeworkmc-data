//! NBT (de)serialisation.


mod ser;
pub use ser::*;

mod deser;
pub use deser::*;

mod value;
pub use value::*;

mod tag {
    pub const END      : u8 = 0;
    pub const BYTE     : u8 = 1;
    pub const SHORT    : u8 = 2;
    pub const INT      : u8 = 3;
    pub const LONG     : u8 = 4;
    pub const FLOAT    : u8 = 5;
    pub const DOUBLE   : u8 = 6;
    pub const BARRAY   : u8 = 7; // TODO: Serialiser
    pub const STRING   : u8 = 8;
    pub const LIST     : u8 = 9;
    pub const COMPOUND : u8 = 10;
    // pub const IARRAY   : u8 = 11; // TODO: Serialiser
    // pub const LARRAY   : u8 = 12; // TODO: Serialiser
}
