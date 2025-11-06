use crate::Minecraft;
use super::tag;
use std::borrow::Cow;
use netzer::prelude::*;
use cesu8::{
    to_java_cesu8,
    from_java_cesu8
};
use serde::{
    Serialize as Ser,
    Deserialize as Deser
};


/// An NBT element.
#[derive(Clone, Debug, Ser, Deser)]
#[serde(untagged)]
pub enum NbtElement<'l> {
    /// Byte
    Byte(i8),
    /// Short
    Short(i16),
    /// Int
    Int(i32),
    /// Long
    Long(i64),
    /// Float
    Float(f32),
    /// Double
    Double(f64),
    // /// Byte array
    // BArray(NbtBArray<'l>),
    /// String
    String(Cow<'l, str>),
    // /// List
    // List(NbtList<'l>),
    // /// Compound
    // Compound(Cow<'l, [(Cow<'l, str>, NbtElement<'l>,)]>),
    // /// Int array
    // IArray(NbtIArray<'l>),
    // /// Long array
    // LArray(NbtLArray<'l>)
}

impl NetEncode<Minecraft> for NbtElement<'_> {
    async fn encode<W : netzer::AsyncWrite>(&self, mut w : W) -> netzer::Result {
        match (self) {
            NbtElement::Byte(v) => {
                w.write_all(&[tag::BYTE]).await?;
                w.write_all(&v.to_be_bytes()).await?;
            },
            NbtElement::Short(v) => {
                w.write_all(&[tag::SHORT]).await?;
                w.write_all(&v.to_be_bytes()).await?;
            },
            NbtElement::Int(v) => {
                w.write_all(&[tag::INT]).await?;
                w.write_all(&v.to_be_bytes()).await?;
            },
            NbtElement::Long(v) => {
                w.write_all(&[tag::LONG]).await?;
                w.write_all(&v.to_be_bytes()).await?;
            },
            NbtElement::Float(v) => {
                w.write_all(&[tag::FLOAT]).await?;
                w.write_all(&v.to_be_bytes()).await?;
            },
            NbtElement::Double(v) => {
                w.write_all(&[tag::DOUBLE]).await?;
                w.write_all(&v.to_be_bytes()).await?;
            },
            NbtElement::String(v) => {
                w.write_all(&[tag::STRING]).await?;
                Self::encode_string(v, w).await?;
            },
        }
        Ok(())
    }
}
impl NbtElement<'_> {
    async fn encode_string<W : netzer::AsyncWrite>(v : &str, mut w : W) -> netzer::Result {
        let jstring = to_java_cesu8(v);
        w.write_all(&u16::try_from(jstring.len()).unwrap().to_be_bytes()).await?;
        w.write_all(&jstring).await?;
        Ok(())
    }
}


impl NetDecode<Minecraft> for NbtElement<'_> {
    async fn decode<R : netzer::AsyncRead>(mut r : R) -> netzer::Result<Self> {
        let mut tag = [0u8; 1];
        r.read_exact(&mut tag).await?;
        let tag = tag[0];
        Ok(match (tag) {
            tag::BYTE   => NbtElement::Byte(Self::decode_primitive(r, i8::from_be_bytes).await?),
            tag::SHORT  => NbtElement::Short(Self::decode_primitive(r, i16::from_be_bytes).await?),
            tag::INT    => NbtElement::Int(Self::decode_primitive(r, i32::from_be_bytes).await?),
            tag::LONG   => NbtElement::Long(Self::decode_primitive(r, i64::from_be_bytes).await?),
            tag::FLOAT  => NbtElement::Float(Self::decode_primitive(r, f32::from_be_bytes).await?),
            tag::DOUBLE => NbtElement::Double(Self::decode_primitive(r, f64::from_be_bytes).await?),
            tag::STRING => NbtElement::String(Cow::Owned(Self::decode_string(r).await?)),
            _ => { return Err("invalid NBT tag".into()); }
        })
    }
}
impl NbtElement<'_> {
    async fn decode_primitive<R : netzer::AsyncRead, T, const N : usize, F>(mut r : R, f : F) -> netzer::Result<T>
    where F : FnOnce([u8; N]) -> T
    {
        let mut buf = [0u8; N];
        r.read_exact(&mut buf).await?;
        Ok(f(buf))
    }
    async fn decode_string<R : netzer::AsyncRead>(mut r : R) -> netzer::Result<String> {
        let mut len = [0u8; 2];
        r.read_exact(&mut len).await?;
        let len = u16::from_be_bytes(len) as usize;
        let mut buf = Box::new_uninit_slice(len);
        r.read_exact(unsafe { buf.assume_init_mut() }).await?;
        let buf = unsafe { buf.assume_init() };
        Ok(match (from_java_cesu8(&buf)?) {
            Cow::Owned(v)    => v,
            Cow::Borrowed(_) => unsafe { String::from_utf8_unchecked(buf.into_vec()) }
        })
    }
}
