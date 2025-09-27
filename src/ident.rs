//! Namespaced resource identifiers.


use pipeworkmc_codec::{
    decode::{
        PacketDecode,
        DecodeIter,
        string::StringDecodeError
    },
    encode::{
        PacketEncode,
        EncodeBuf
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


/// A namespaced resource identifier.
///
/// [`Ident`]s are used to identify everything from asset locations, channel ids, entity types, etc.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Ident {
    joined    : Cow<'static, str>,
    split_idx : usize
}

impl Display for Ident {
    #[inline]
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.joined)
    }
}

impl Debug for Ident {
    #[inline]
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.joined)
    }
}

impl SynDebug for Ident {
    #[inline]
    fn fmt(&self, f : &mut Formatter<'_>, _const_like : bool) -> fmt::Result {
        write!(f, "Ident::new({:?})", self.joined)
    }
}


impl Ident {

    /// Returns the namespace of this identifier.
    #[inline]
    pub fn namespace(&self) -> &str {
        // SAFETY: `self.split_idx` is always less than `self.joined.len()`.
        unsafe { self.joined.get_unchecked(0..self.split_idx) }
    }

    /// Returns the path of this identifier.
    #[inline]
    pub fn path(&self) -> &str {
        // SAFETY: `self.split_idx + 1` is always less than `self.joined.len()`.
        unsafe { self.joined.get_unchecked((self.split_idx + 1)..) }
    }

    /// Returns this identifier as a [`&str`](str).
    #[inline(always)]
    pub fn as_str(&self) -> &str { &self.joined }

}

impl Ident {

    /// Creates a new identifier from a joined string without checking validity.
    ///
    /// ### Safety
    /// The caller is responsible for ensuring that the given string is a valid identifier:
    /// - Must only contain ASCII characters.
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

    #[inline(always)]
    const unsafe fn new_unchecked_manual(joined : Cow<'static, str>, split_idx : usize) -> Self {
        Self { joined, split_idx }
    }

    /// Creates a new identifier from a joined [`&'static str`](str).
    ///
    /// ### Panics
    /// Panics if the given string is not a valid identifier.
    #[track_caller]
    #[inline]
    pub const fn new(s : &'static str) -> Self {
        match (Self::new_checked(s)) {
            Ok(ident) => ident,
            Err(err)  => match (err) {
                IdentValidateError::NotAscii       => panic!("Ident contains non-ASCII characters"),
                IdentValidateError::EmptyComponent => panic!("Ident contains empty component"),
                IdentValidateError::BadChar(_)     => panic!("Ident component contains invalid character"),
                IdentValidateError::NoSeparator    => panic!("Ident missing separator character")
            }
        }
    }

    /// Creates a new identifier from a joined [`&'static str`](str),
    ///  returning an error if it is not a valid identifier.
    #[inline]
    pub const fn new_checked(s : &'static str) -> Result<Self, IdentValidateError> {
        match (Self::validate_joined(s)) {
            // SAFETY: `s` was validated in the line above.
            Ok(split_idx) => Ok(unsafe { Self::new_unchecked_manual(Cow::Borrowed(s), split_idx) }),
            Err(err)      => Err(err)
        }
    }

    /// Creates a new identifier from a separated namespace and path,
    ///  returning and error if they are not valid parts of an identifier.
    #[inline(always)]
    pub fn new_from_pair<N, P>(namespace : N, path : P) -> Result<Self, IdentValidateError>
    where
        N : AsRef<str>,
        P : AsRef<str>
    {
        Self::try_from((namespace, path,))
    }

    const fn validate_joined(joined : &str) -> Result<usize, IdentValidateError> {
        if (! joined.is_ascii()) {
            return Err(IdentValidateError::NotAscii);
        }
        let mut i = 0;
        while (i < joined.len()) {
            let ch = joined.as_bytes()[i];
            if (ch == b':') {
                if (i == 0) {
                    return Err(IdentValidateError::EmptyComponent);
                }
                return match (Self::validate_path(joined, i + 1)) {
                    Ok(()) => Ok(i),
                    Err(err) => Err(err),
                };
            } else if (! Self::is_valid_component_char(ch)) {
                return Err(IdentValidateError::BadChar(ch as char));
            }
            i += 1;
        }
        Err(IdentValidateError::NoSeparator)
    }

    #[inline]
    const fn validate_path(joined : &str, mut i : usize) -> Result<(), IdentValidateError> {
        let mut component_len = 0usize;
        while (i < joined.len()) {
            let ch = joined.as_bytes()[i];
            if (ch == b'/') {
                if (component_len == 0) {
                    return Err(IdentValidateError::EmptyComponent);
                }
                component_len = 0;
            } else if (Self::is_valid_component_char(ch)) {
                component_len += 1;
            } else {
                return Err(IdentValidateError::BadChar(ch as char));
            }
            i += 1;
        }
        if (component_len == 0) {
            return Err(IdentValidateError::EmptyComponent);
        }
        Ok(())
    }

    #[inline(always)]
    const fn is_valid_component_char(ch : u8) -> bool {
        (ch >= b'a' && ch <= b'z') || (ch >= b'0' && ch <= b'9') || ch == b'.' || ch == b'-' || ch == b'_'
    }

}


impl TryFrom<Cow<'static, str>> for Ident {
    type Error = IdentValidateError;
    #[inline]
    fn try_from(s : Cow<'static, str>) -> Result<Self, Self::Error> {
        let split_idx = Self::validate_joined(&s)?;
        // SAFETY: `s` was validated in the line above.
        Ok(unsafe { Self::new_unchecked_manual(s, split_idx) })
    }
}
impl TryFrom<&'static str> for Ident {
    type Error = IdentValidateError;
    #[inline(always)]
    fn try_from(s : &'static str) -> Result<Self, Self::Error> {
        Self::try_from(Cow::Borrowed(s))
    }
}
impl TryFrom<String> for Ident {
    type Error = IdentValidateError;
    #[inline(always)]
    fn try_from(s : String) -> Result<Self, Self::Error> {
        Self::try_from(Cow::Owned(s))
    }
}
impl<N, P> TryFrom<(N, P,)> for Ident
where
    N : AsRef<str>,
    P : AsRef<str>
{
    type Error = IdentValidateError;
    #[inline]
    fn try_from((n, p,) : (N, P,)) -> Result<Self, Self::Error> {
        Self::try_from(format!("{}:{}", n.as_ref(), p.as_ref()))
    }
}


impl Ser for Ident {
    #[inline]
    fn serialize<S>(&self, serer : S) -> Result<S::Ok, S::Error>
    where
        S : Serer
    { serer.serialize_str(self.as_str()) }
}

impl<'de> Deser<'de> for Ident {
    fn deserialize<D>(deserer : D) -> Result<Self, D::Error>
    where
        D : Deserer<'de>
    { Self::try_from(<String>::deserialize(deserer)?).map_err(D::Error::custom) }
}


impl PacketDecode for Ident {
    type Error = IdentDecodeError;

    fn decode<I>(iter : &mut DecodeIter<I>) -> Result<Self, Self::Error>
    where
        I : ExactSizeIterator<Item = u8>
    {
        let s = <String>::decode(iter).map_err(IdentDecodeError::String)?;
        Self::try_from(s).map_err(IdentDecodeError::Validate)
    }
}

unsafe impl PacketEncode for Ident {

    #[inline(always)]
    fn encode_len(&self) -> usize {
        self.joined.encode_len()
    }

    #[inline(always)]
    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        self.joined.encode(buf)
    } }

}


/// Returned by [`Ident`] constructors when invalid parameters are provided.
#[derive(Debug)]
pub enum IdentValidateError {
    /// The identifier contains non-ASCII characters.
    NotAscii,
    /// The identifier contains an empty component.
    EmptyComponent,
    /// The identifier contains an invalid character.
    BadChar(char),
    /// The identifier is missing a separator (`:`).
    NoSeparator
}
impl Display for IdentValidateError {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result { match (self) {
        Self::NotAscii       => write!(f, "contains non-ASCII characters"),
        Self::EmptyComponent => write!(f, "contains empty component"),
        Self::BadChar(ch)    => write!(f, "component contains invalid character {ch:?}"),
        Self::NoSeparator    => write!(f, "missing separator character")
    } }
}

/// Returned by packet decoders when an `Ident` was not decoded successfully.
#[derive(Debug)]
pub enum IdentDecodeError {
    /// The read data was not a valid string.
    String(StringDecodeError),
    /// The string was not a valid identifier.
    Validate(IdentValidateError)
}
impl Display for IdentDecodeError {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result { match (self) {
        Self::String(err)   => write!(f, "{err}"),
        Self::Validate(err) => write!(f, "{err}")
    } }
}
