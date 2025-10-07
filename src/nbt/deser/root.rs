use super::{
    NbtDeserError,
    tag
};
use std::{
    borrow::Cow,
    io::{ self, Read }
};
use cesu8::from_java_cesu8;
use serde::de::{
    Deserializer as Deserer,
    Visitor
};


pub(super) struct NbtRootDeserer<'l, R>
where
    R : Read
{
    reader    : &'l mut R,
    tag_read : TagRead
}

impl<'l, R> NbtRootDeserer<'l, R>
where
    R : Read
{

    pub(super) fn new(reader : &'l mut R, tag_read : TagRead) -> Self {
        Self { reader, tag_read }
    }

    fn handle_tag_read(&mut self, expected_tag : u8) -> Result<(), NbtDeserError> { match (&self.tag_read) {
        TagRead::None => Ok(()),
        TagRead::Tag => {
            let tag = self.read_u8()?;
            if (tag == expected_tag) {
                Ok(())
            } else {
                Err(NbtDeserError::UnknownTag(tag))
            }
        }
    } }

    fn read_u8(&mut self) -> io::Result<u8> {
        let mut buf = [0u8];
        self.reader.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    fn read_u16(&mut self) -> io::Result<u16> {
        let mut buf = [0u8; 2];
        self.reader.read_exact(&mut buf)?;
        Ok(u16::from_be_bytes(buf))
    }

    fn read_u32(&mut self) -> io::Result<u32> {
        let mut buf = [0u8; 4];
        self.reader.read_exact(&mut buf)?;
        Ok(u32::from_be_bytes(buf))
    }

    fn read_u64(&mut self) -> io::Result<u64> {
        let mut buf = [0u8; 8];
        self.reader.read_exact(&mut buf)?;
        Ok(u64::from_be_bytes(buf))
    }

    fn read_f32(&mut self) -> io::Result<f32> {
        let mut buf = [0u8; 4];
        self.reader.read_exact(&mut buf)?;
        Ok(f32::from_be_bytes(buf))
    }

    fn read_f64(&mut self) -> io::Result<f64> {
        let mut buf = [0u8; 8];
        self.reader.read_exact(&mut buf)?;
        Ok(f64::from_be_bytes(buf))
    }

    fn read_string(&mut self) -> Result<String, NbtDeserError> {
        let     len = self.read_u16()? as usize;
        let mut buf = Box::new_uninit_slice(len);
        self.reader.read_exact(unsafe { buf.assume_init_mut() })?;
        let buf    = unsafe { buf.assume_init() };
        Ok(match (from_java_cesu8(&buf)?) {
            Cow::Owned(v)    => v,
            Cow::Borrowed(_) => unsafe { String::from_utf8_unchecked(buf.into_vec()) }
        })
    }

    fn read_barray(&mut self) -> io::Result<Vec<u8>> {
        let    len = self.read_u32()? as usize;
        let mut buf = Box::new_uninit_slice(len);
        self.reader.read_exact(unsafe { buf.assume_init_mut() })?;
        Ok(unsafe { buf.assume_init() }.into_vec())
    }

}


pub(super) enum TagRead {
    None,
    Tag
}


impl<'de, R> Deserer<'de> for &mut NbtRootDeserer<'_, R>
where
    R : Read
{
    type Error = NbtDeserError;

    fn deserialize_any<V>(self, visitor : V) -> Result<V::Value, Self::Error>
    where
        V : Visitor<'de>
    {
        let TagRead::Tag = self.tag_read
            else { panic!("Can not tagless any NBT deserialize."); };
        let mut prefix = [0u8];
        self.reader.read_exact(&mut prefix)?;
        self.tag_read = TagRead::None;
        match (prefix[0]) {
            tag::END      => self.deserialize_unit(visitor),
            tag::BYTE     => self.deserialize_i8(visitor),
            tag::SHORT    => self.deserialize_i16(visitor),
            tag::INT      => self.deserialize_i32(visitor),
            tag::LONG     => self.deserialize_i64(visitor),
            tag::FLOAT    => self.deserialize_f32(visitor),
            tag::DOUBLE   => self.deserialize_i64(visitor),
            // tODO: BArray
            tag::STRING   => self.deserialize_string(visitor),
            tag::LIST     => self.deserialize_seq(visitor),
            tag::COMPOUND => self.deserialize_map(visitor),
            // TODO: IArray
            // TODO: LArray
            v => Err(NbtDeserError::UnknownTag(v))
        }
    }

    fn deserialize_bool<V>(self, visitor : V) -> Result<V::Value, Self::Error>
    where
        V : Visitor<'de>
    {
        self.handle_tag_read(tag::BYTE)?;
        visitor.visit_bool(self.read_u8()? != 0)
    }

    fn deserialize_i8<V>(self, visitor : V) -> Result<V::Value, Self::Error>
    where
        V : Visitor<'de>
    {
        self.handle_tag_read(tag::BYTE)?;
        visitor.visit_i8(self.read_u8()?.cast_signed())
    }

    fn deserialize_i16<V>(self, visitor : V) -> Result<V::Value, Self::Error>
    where
        V : Visitor<'de>
    {
        self.handle_tag_read(tag::SHORT)?;
        visitor.visit_i16(self.read_u16()?.cast_signed())
    }

    fn deserialize_i32<V>(self, visitor : V) -> Result<V::Value, Self::Error>
    where
        V : Visitor<'de>
    {
        self.handle_tag_read(tag::INT)?;
        visitor.visit_i32(self.read_u32()?.cast_signed())
    }

    fn deserialize_i64<V>(self, visitor : V) -> Result<V::Value, Self::Error>
    where
        V : Visitor<'de>
    {
        self.handle_tag_read(tag::LONG)?;
        visitor.visit_i64(self.read_u64()?.cast_signed())
    }

    fn deserialize_u8<V>(self, visitor : V) -> Result<V::Value, Self::Error>
    where
        V : Visitor<'de>
    {
        self.handle_tag_read(tag::BYTE)?;
        visitor.visit_u8(self.read_u8()?)
    }

    fn deserialize_u16<V>(self, visitor : V) -> Result<V::Value, Self::Error>
    where
        V : Visitor<'de>
    {
        self.handle_tag_read(tag::SHORT)?;
        visitor.visit_u16(self.read_u16()?)
    }

    fn deserialize_u32<V>(self, visitor : V) -> Result<V::Value, Self::Error>
    where
        V : Visitor<'de>
    {
        self.handle_tag_read(tag::INT)?;
        visitor.visit_u32(self.read_u32()?)
    }

    fn deserialize_u64<V>(self, visitor : V) -> Result<V::Value, Self::Error>
    where
        V : Visitor<'de>
    {
        self.handle_tag_read(tag::LONG)?;
        visitor.visit_u64(self.read_u64()?)
    }

    fn deserialize_f32<V>(self, visitor : V) -> Result<V::Value, Self::Error>
    where
        V : Visitor<'de>
    {
        self.handle_tag_read(tag::FLOAT)?;
        visitor.visit_f32(self.read_f32()?)
    }

    fn deserialize_f64<V>(self, visitor : V) -> Result<V::Value, Self::Error>
    where
        V : Visitor<'de>
    {
        self.handle_tag_read(tag::DOUBLE)?;
        visitor.visit_f64(self.read_f64()?)
    }

    fn deserialize_char<V>(self, visitor : V) -> Result<V::Value, Self::Error>
    where
        V : Visitor<'de>
    {
        self.handle_tag_read(tag::STRING)?;
        let     string = self.read_string()?;
        let mut chars  = string.chars();
        let (Some(char), None,) = (chars.next(), chars.next(),)
            else { return Err(NbtDeserError::BadChar); };
        visitor.visit_char(char)
    }

    fn deserialize_str<V>(self, visitor : V) -> Result<V::Value, Self::Error>
    where
        V : Visitor<'de>
    {
        self.handle_tag_read(tag::STRING)?;
        visitor.visit_string(self.read_string()?)
    }

    fn deserialize_string<V>(self, visitor : V) -> Result<V::Value, Self::Error>
    where
        V : Visitor<'de>
    {
        self.handle_tag_read(tag::STRING)?;
        visitor.visit_string(self.read_string()?)
    }

    fn deserialize_bytes<V>(self, visitor : V) -> Result<V::Value, Self::Error>
    where
        V : Visitor<'de>
    {
        self.handle_tag_read(tag::BARRAY)?;
        visitor.visit_byte_buf(self.read_barray()?)
    }

    fn deserialize_byte_buf<V>(self, visitor : V) -> Result<V::Value, Self::Error>
    where
        V : Visitor<'de>
    {
        self.handle_tag_read(tag::BARRAY)?;
        visitor.visit_byte_buf(self.read_barray()?)
    }

    fn deserialize_option<V>(self, _visitor : V) -> Result<V::Value, Self::Error>
    where
        V : Visitor<'de>
    {
        todo!()
    }

    fn deserialize_unit<V>(self, visitor : V) -> Result<V::Value, Self::Error>
    where
        V : Visitor<'de>
    {
        self.handle_tag_read(tag::END)?;
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(
        self,
        _name    : &'static str,
        _visitor : V,
    ) -> Result<V::Value, Self::Error>
    where
        V : Visitor<'de>
    {
        unreachable!("unit structs are not supported by the NBT deserialiser");
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name    : &'static str,
        _visitor : V,
    ) -> Result<V::Value, Self::Error>
    where
        V : Visitor<'de>
    {
        unreachable!("newtype structs are not supported by the NBT deserialiser");
    }

    fn deserialize_seq<V>(self, _visitor : V) -> Result<V::Value, Self::Error>
    where
        V : Visitor<'de> {
        todo!()
    }

    fn deserialize_tuple<V>(self, _len : usize, _visitor : V) -> Result<V::Value, Self::Error>
    where
        V : Visitor<'de> {
        todo!()
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name    : &'static str,
        _len     : usize,
        _visitor : V,
    ) -> Result<V::Value, Self::Error>
    where
        V : Visitor<'de>
    {
        unreachable!("tuple structs are not supported by the NBT deserialiser");
    }

    fn deserialize_map<V>(self, _visitor : V) -> Result<V::Value, Self::Error>
    where
        V : Visitor<'de>
    {
        todo!()
    }

    fn deserialize_struct<V>(
        self,
        _name    : &'static str,
        _fields  : &'static [&'static str],
        _visitor : V,
    ) -> Result<V::Value, Self::Error>
    where
        V : Visitor<'de> {
        todo!()
    }

    fn deserialize_enum<V>(
        self,
        _name     : &'static str,
        _variants : &'static [&'static str],
        _visitor  : V,
    ) -> Result<V::Value, Self::Error>
    where
        V : Visitor<'de>
    {
        todo!()
    }

    fn deserialize_identifier<V>(self, _visitor : V) -> Result<V::Value, Self::Error>
    where
        V : Visitor<'de>
    {
        unreachable!("identifiers are not supported by the NBT deserialiser");
    }

    fn deserialize_ignored_any<V>(self, visitor : V) -> Result<V::Value, Self::Error>
    where
        V : Visitor<'de>
    {
        self.deserialize_any(visitor)
    }
}
