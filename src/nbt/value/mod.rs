use crate::box_cow::BoxCow;
use std::borrow::Cow;


/// An NBT element.
pub enum NbtElement<'l> {
    /// `()`
    Unit,
    /// `i8`
    Byte(i8),
    /// `i16`
    Short(i16),
    /// `i32`
    Int(i32),
    /// `i64`
    Long(i64),
    /// `f32`
    Float(f32),
    /// `f64`
    Double(f64),
    /// `u8` array.
    BArray(Cow<'l, [u8]>),
    /// String.
    String(Cow<'l, str>),
    /// Typed element list.
    List(BoxCow<'l, [NbtElement<'l>], Vec<NbtElement<'l>>>),
    /// Map
    Compound(BoxCow<'l, [(Cow<'l, str>, NbtElement<'l>,)], Vec<(Cow<'l, str>, NbtElement<'l>,)>>),
    /// `i32` array.
    IArray(Cow<'l, [i32]>),
    /// `i64` array.
    LArray(Cow<'l, [i64]>)
}
