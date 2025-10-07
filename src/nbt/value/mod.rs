use core::fmt::{ self, Formatter };
use std::borrow::Cow;
use serde::de::{
    Error as DeserError,
    Deserialize as Deser,
    Deserializer as Deserer,
    Visitor,
    SeqAccess,
    MapAccess
};


/// An NBT element.
#[derive(Clone)]
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
    List(Cow<'l, [NbtElement<'l>]>),
    /// Map
    Compound(Cow<'l, [(Cow<'l, str>, NbtElement<'l>,)]>),
    /// `i32` array.
    IArray(Cow<'l, [i32]>),
    /// `i64` array.
    LArray(Cow<'l, [i64]>)
}

impl<'de> Deser<'de> for NbtElement<'de> {

    fn deserialize<D>(deserer : D) -> Result<Self, D::Error>
    where
        D : Deserer<'de>
    {
        deserer.deserialize_any(NbtElementVisitor)
    }

}


struct NbtElementVisitor;

impl<'de> Visitor<'de> for NbtElementVisitor {
    type Value = NbtElement<'de>;

    fn expecting(&self, f : &mut Formatter) -> fmt::Result {
        write!(f, "valid NBT data")
    }

    fn visit_bool<E>(self, v : bool) -> Result<Self::Value, E>
    where
        E : DeserError
    { Ok(NbtElement::Byte(if (v) { 1 } else { 0 })) }

    fn visit_i8<E>(self, v : i8) -> Result<Self::Value, E>
    where
        E : DeserError
    { Ok(NbtElement::Byte(v)) }

    fn visit_i16<E>(self, v : i16) -> Result<Self::Value, E>
    where
        E : DeserError
    { Ok(NbtElement::Short(v)) }

    fn visit_i32<E>(self, v : i32) -> Result<Self::Value, E>
    where
        E : DeserError
    { Ok(NbtElement::Int(v)) }

    fn visit_i64<E>(self, v : i64) -> Result<Self::Value, E>
    where
        E : DeserError
    { Ok(NbtElement::Long(v)) }

    fn visit_u8<E>(self, v : u8) -> Result<Self::Value, E>
    where
        E : DeserError
    { Ok(NbtElement::Byte(v.cast_signed())) }

    fn visit_u16<E>(self, v : u16) -> Result<Self::Value, E>
    where
        E : DeserError
    { Ok(NbtElement::Short(v.cast_signed())) }

    fn visit_u32<E>(self, v : u32) -> Result<Self::Value, E>
    where
        E : DeserError
    { Ok(NbtElement::Int(v.cast_signed())) }

    fn visit_u64<E>(self, v : u64) -> Result<Self::Value, E>
    where
        E : DeserError
    { Ok(NbtElement::Long(v.cast_signed())) }

    fn visit_f32<E>(self, v : f32) -> Result<Self::Value, E>
    where
        E : DeserError
    { Ok(NbtElement::Float(v)) }

    fn visit_f64<E>(self, v : f64) -> Result<Self::Value, E>
    where
        E : DeserError
    { Ok(NbtElement::Double(v)) }

    fn visit_char<E>(self, v : char) -> Result<Self::Value, E>
    where
        E : DeserError
    { Ok(NbtElement::String(Cow::Owned(v.to_string()))) }

    fn visit_str<E>(self, v : &str) -> Result<Self::Value, E>
    where
        E : DeserError
    { Ok(NbtElement::String(Cow::Owned(v.to_string()))) }

    fn visit_borrowed_str<E>(self, v : &'de str) -> Result<Self::Value, E>
    where
        E : DeserError
    { Ok(NbtElement::String(Cow::Borrowed(v))) }

    fn visit_string<E>(self, v : String) -> Result<Self::Value, E>
    where
        E : DeserError
    { Ok(NbtElement::String(Cow::Owned(v))) }

    fn visit_bytes<E>(self, v : &[u8]) -> Result<Self::Value, E>
    where
        E : DeserError
    { Ok(NbtElement::BArray(Cow::Owned(v.to_vec()))) }

    fn visit_borrowed_bytes<E>(self, v : &'de [u8]) -> Result<Self::Value, E>
    where
        E : DeserError
    { Ok(NbtElement::BArray(Cow::Borrowed(v))) }

    fn visit_byte_buf<E>(self, v : Vec<u8>) -> Result<Self::Value, E>
    where
        E : DeserError
    { Ok(NbtElement::BArray(Cow::Owned(v))) }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E : DeserError
    { Ok(NbtElement::Unit) }

    fn visit_seq<A>(self, mut seq : A) -> Result<Self::Value, A::Error>
    where
        A : SeqAccess<'de>
    {
        let mut elem = seq.size_hint().map_or_else(Vec::new, Vec::with_capacity);
        while let Some(v) = seq.next_element()? {
            elem.push(v);
        }
        Ok(NbtElement::List(Cow::Owned(elem)))
    }

    fn visit_map<A>(self, mut map : A) -> Result<Self::Value, A::Error>
    where
        A : MapAccess<'de>
    {
        let mut elem = map.size_hint().map_or_else(Vec::new, Vec::with_capacity);
        while let Some(e) = map.next_entry::<Cow<'de, str>, NbtElement<'de>>()? {
            elem.push(e);
        }
        Ok(NbtElement::Compound(Cow::Owned(elem)))
    }

}
