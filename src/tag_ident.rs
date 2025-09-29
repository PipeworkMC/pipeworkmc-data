//! Namespaced resource identifier tags.


use pipeworkmc_codec::{
    decode::{
        PacketDecode,
        DecodeIter,
        string::StringDecodeError
    },
    encode::{
        EncodeBuf, PacketEncode
    }
};
use core::fmt::{ self, Display, Debug, Formatter };
use std::borrow::Cow;
use serde::{
    Serialize as Ser,
    Serializer as Serer,
    Deserialize as Deser,
    Deserializer as Deserer,
    de::Error as _
};
use syndebug::SynDebug;


/// A namespaced resource identifier tag.
///
/// [`TagIdent`]s are used to identify groups of asset locations, channel ids, entity types, etc.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TagIdent {
    joined    : Cow<'static, str>,
    split_idx : usize
}

impl Display for TagIdent {
    #[inline]
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.joined)
    }
}

impl Debug for TagIdent {
    #[inline]
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.joined)
    }
}

impl SynDebug for TagIdent {
    #[inline]
    fn fmt(&self, f : &mut Formatter<'_>, _const_like : bool) -> fmt::Result {
        write!(f, "TagIdent::new({:?})", self.joined)
    }
}


impl TagIdent {

    /// Returns the namespace of this tag identifier.
    #[inline]
    pub fn namespace(&self) -> &str {
        // SAFETY: `self.split_idx` is always less than `self.joined.len()`.
        unsafe { self.joined.get_unchecked(1..self.split_idx) }
    }

    /// Returns the path of this tag identifier.
    #[inline]
    pub fn path(&self) -> &str {
        // SAFETY: `self.split_idx + 1` is always less than `self.joined.len()`.
        unsafe { self.joined.get_unchecked((self.split_idx + 1)..) }
    }

    /// Returns this tag identifier as a [`&str`](str), including the `#` character.
    #[inline]
    pub fn as_str(&self) -> &str { &self.joined }

}

impl TagIdent {

    /// Creates a new tag identifier from a joined string without checking validity.
    ///
    /// ### Safety
    /// The caller is responsible for ensuring that the given string is a valid tag identifier:
    /// - Must only contain ASCII characters.
    /// - Must contain a single `#` as the first character.
    /// - Must contain a single `:` character.
    /// - Namespaces or path segments.
    /// - Namespaces and path segments must only contain `[a-z0-9.\-_]` characters.
    /// - Path segments must be `/`-separated.
    pub unsafe fn new_unchecked<S>(joined : S) -> Self
    where
        S : Into<Cow<'static, str>>
    {
        let joined = joined.into();
        let Some(split_idx) = joined.as_bytes().iter().position(|&c| c == b':')
            else { panic!("Ident missing separator character"); };
        unsafe { Self::new_unchecked_manual(joined, split_idx) }
    }

    #[inline]
    const unsafe fn new_unchecked_manual(joined : Cow<'static, str>, split_idx : usize) -> Self {
        Self { joined, split_idx }
    }

    /// Creates a new tag identifier from a joined [`&'static str`](str).
    ///
    /// ### Panics
    /// Panics if the given string is not a valid tag identifier.
    #[track_caller]
    #[inline]
    pub const fn new(s : &'static str) -> Self {
        match (Self::new_checked(s)) {
            Ok(ident) => ident,
            Err(err)  => match (err) {
                TagIdentValidateError::NotAscii       => panic!("Ident contains non-ASCII characters"),
                TagIdentValidateError::NoTag          => panic!("Ident missing `#` character"),
                TagIdentValidateError::EmptyComponent => panic!("Ident contains empty component"),
                TagIdentValidateError::BadChar(_)     => panic!("Ident component contains invalid character"),
                TagIdentValidateError::NoSeparator    => panic!("Ident missing `:` character")
            }
        }
    }

    /// Creates a new tag identifier from a joined [`&'static str`](str),
    ///  returning an error if it is not a valid tag identifier.
    #[inline]
    pub const fn new_checked(s : &'static str) -> Result<Self, TagIdentValidateError> {
        match (Self::validate_joined(s)) {
            Ok(split_idx) => Ok(unsafe { Self::new_unchecked_manual(Cow::Borrowed(s), split_idx) }),
            Err(err)      => Err(err)
        }
    }

    /// Creates a new tag identifier from a separated namespace and path,
    ///  returning and error if they are not valid parts of a tag identifier.
    ///
    /// The `#` character is added automatically. Neither the namespace or the path should contain it.
    #[inline]
    pub fn new_from_pair(namespace : &str, path : &str) -> Result<Self, TagIdentValidateError> {
        Self::try_from((namespace, path,))
    }

    const fn validate_joined(joined : &str) -> Result<usize, TagIdentValidateError> {
        if (! joined.is_ascii()) {
            return Err(TagIdentValidateError::NotAscii);
        }
        let mut i = 0;
        while (i < joined.len()) {
            let ch = joined.as_bytes()[i];
            if (i == 0) {
                if (ch != b'#') {
                    return Err(TagIdentValidateError::NoTag);
                }
            } else if (ch == b':') {
                if (i == 0) {
                    return Err(TagIdentValidateError::EmptyComponent);
                }
                return match (Self::validate_path(joined, i + 1)) {
                    Ok(()) => Ok(i),
                    Err(err) => Err(err),
                };
            } else if (! Self::is_valid_component_char(ch)) {
                return Err(TagIdentValidateError::BadChar(ch as char));
            }
            i += 1;
        }
        Err(TagIdentValidateError::NoSeparator)
    }

    #[inline]
    const fn validate_path(joined : &str, mut i : usize) -> Result<(), TagIdentValidateError> {
        let mut component_len = 0usize;
        while (i < joined.len()) {
            let ch = joined.as_bytes()[i];
            if (ch == b'/') {
                if (component_len == 0) {
                    return Err(TagIdentValidateError::EmptyComponent);
                }
                component_len = 0;
            } else if (Self::is_valid_component_char(ch)) {
                component_len += 1;
            } else {
                return Err(TagIdentValidateError::BadChar(ch as char));
            }
            i += 1;
        }
        if (component_len == 0) {
            return Err(TagIdentValidateError::EmptyComponent);
        }
        Ok(())
    }

    #[inline]
    const fn is_valid_component_char(ch : u8) -> bool {
        (ch >= b'a' && ch <= b'z') || (ch >= b'0' && ch <= b'9') || ch == b'.' || ch == b'-' || ch == b'_'
    }

}


impl TryFrom<Cow<'static, str>> for TagIdent {
    type Error = TagIdentValidateError;
    #[inline]
    fn try_from(s : Cow<'static, str>) -> Result<Self, Self::Error> {
        let split_idx = Self::validate_joined(&s)?;
        Ok(unsafe { Self::new_unchecked_manual(s, split_idx) })
    }
}
impl TryFrom<&'static str> for TagIdent {
    type Error = TagIdentValidateError;
    #[inline]
    fn try_from(s : &'static str) -> Result<Self, Self::Error> {
        Self::try_from(Cow::Borrowed(s))
    }
}
impl TryFrom<String> for TagIdent {
    type Error = TagIdentValidateError;
    #[inline]
    fn try_from(s : String) -> Result<Self, Self::Error> {
        Self::try_from(Cow::Owned(s))
    }
}
impl<N, P> TryFrom<(N, P,)> for TagIdent
where
    N : AsRef<str>,
    P : AsRef<str>
{
    type Error = TagIdentValidateError;
    #[inline]
    fn try_from((n, p,) : (N, P,)) -> Result<Self, Self::Error> {
        Self::try_from(format!("#{}:{}", n.as_ref(), p.as_ref()))
    }
}


impl Ser for TagIdent {
    #[inline]
    fn serialize<S>(&self, serer : S) -> Result<S::Ok, S::Error>
    where
        S : Serer
    { self.as_str().serialize(serer) }
}

impl<'de> Deser<'de> for TagIdent {
    fn deserialize<D>(deserer : D) -> Result<Self, D::Error>
    where
        D : Deserer<'de>
    { Self::try_from(<String>::deserialize(deserer)?).map_err(D::Error::custom) }
}


impl PacketDecode for TagIdent {
    type Error = TagIdentDecodeError;

    fn decode<I>(iter : &mut DecodeIter<I>) -> Result<Self, Self::Error>
    where
        I : ExactSizeIterator<Item = u8>
    {
        let s = <String>::decode(iter).map_err(TagIdentDecodeError::String)?;
        Self::try_from(s).map_err(TagIdentDecodeError::Validate)
    }
}

unsafe impl PacketEncode for TagIdent {

    #[inline]
    fn encode_len(&self) -> usize {
        self.joined.encode_len()
    }

    #[inline]
    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        self.joined.encode(buf)
    } }

}


/// Returned by [`TagIdent`] constructors when invalid parameters are provided.
#[derive(Debug)]
pub enum TagIdentValidateError {
    /// The tag identifier contains non-ASCII characters.
    NotAscii,
    /// The identifier is missing an octothorpe (`#`) as the first character.
    NoTag,
    /// The tag identifier contains an empty component.
    EmptyComponent,
    /// The tag identifier contains an invalid character.
    BadChar(char),
    /// The tag identifier is missing a separator (`:`).
    NoSeparator
}
impl Display for TagIdentValidateError {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result { match (self) {
        Self::NotAscii       => write!(f, "contains non-ASCII characters"),
        Self::NoTag          => write!(f, "missing `#` character"),
        Self::EmptyComponent => write!(f, "contains empty component"),
        Self::BadChar(ch)    => write!(f, "component contains invalid character {ch:?}"),
        Self::NoSeparator    => write!(f, "missing `:` character")
    } }
}

/// Returned by packet decoders when a `TagIdent` was not decoded successfully.
#[derive(Debug)]
pub enum TagIdentDecodeError {
    /// The read data was not a valid string.
    String(StringDecodeError),
    /// The string was not a valid tag identifier.
    Validate(TagIdentValidateError)
}
impl Display for TagIdentDecodeError {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result { match (self) {
        Self::String(err)   => write!(f, "{err}"),
        Self::Validate(err) => write!(f, "{err}")
    } }
}
