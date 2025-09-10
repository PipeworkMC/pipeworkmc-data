use crate::num::provider::IntProvider;
use core::fmt::{ self, Formatter };
use serde::{
    Serialize as Ser,
    Serializer as Serer,
    Deserialize as Deser,
    Deserializer as Deserer,
    de::Error as _
};
use syndebug::SynDebug;


#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct LightLevel(u8);

impl LightLevel {

    #[inline]
    pub const fn new(n : u8) -> Option<Self> {
        if (n < 16) {
            Some(Self(n))
        } else { None }
    }

    #[inline(always)]
    pub const unsafe fn new_unchecked(n : u8) -> Self {
        Self(n)
    }

    #[inline]
    pub const fn get(self) -> Self { Self(self.0) }

}

impl SynDebug for LightLevel {
    #[inline]
    fn fmt(&self, f : &mut Formatter<'_>, _const_like : bool) -> fmt::Result {
        write!(f, "LightLevel::new({}).unwrap()", self.0)
    }
}


impl Ser for LightLevel {
    #[inline(always)]
    fn serialize<S>(&self, serer : S) -> Result<S::Ok, S::Error>
    where
        S : Serer
    { (self.0 as u32).serialize(serer) }
}

impl<'de> Deser<'de> for LightLevel {
    fn deserialize<D>(deserer : D) -> Result<Self, D::Error>
    where
        D : Deserer<'de>
    {
        Self::new(<u32>::deserialize(deserer)?.try_into().map_err(D::Error::custom)?)
            .ok_or_else(|| D::Error::custom("light level outside of 0-15 range"))
    }
}

impl TryFrom<i8> for LightLevel {
    type Error = ();
    fn try_from(value : i8) -> Result<Self, Self::Error> {
        Self::new(value.try_into().map_err(|_| ())?).ok_or(())
    }
}
impl TryFrom<u8> for LightLevel {
    type Error = ();
    fn try_from(value : u8) -> Result<Self, Self::Error> {
        Self::new(value.try_into().map_err(|_| ())?).ok_or(())
    }
}
impl TryFrom<i16> for LightLevel {
    type Error = ();
    fn try_from(value : i16) -> Result<Self, Self::Error> {
        Self::new(value.try_into().map_err(|_| ())?).ok_or(())
    }
}
impl TryFrom<u16> for LightLevel {
    type Error = ();
    fn try_from(value : u16) -> Result<Self, Self::Error> {
        Self::new(value.try_into().map_err(|_| ())?).ok_or(())
    }
}
impl TryFrom<i32> for LightLevel {
    type Error = ();
    fn try_from(value : i32) -> Result<Self, Self::Error> {
        Self::new(value.try_into().map_err(|_| ())?).ok_or(())
    }
}
impl TryFrom<u32> for LightLevel {
    type Error = ();
    fn try_from(value : u32) -> Result<Self, Self::Error> {
        Self::new(value.try_into().map_err(|_| ())?).ok_or(())
    }
}
impl TryFrom<i64> for LightLevel {
    type Error = ();
    fn try_from(value : i64) -> Result<Self, Self::Error> {
        Self::new(value.try_into().map_err(|_| ())?).ok_or(())
    }
}
impl TryFrom<u64> for LightLevel {
    type Error = ();
    fn try_from(value : u64) -> Result<Self, Self::Error> {
        Self::new(value.try_into().map_err(|_| ())?).ok_or(())
    }
}
impl TryFrom<i128> for LightLevel {
    type Error = ();
    fn try_from(value : i128) -> Result<Self, Self::Error> {
        Self::new(value.try_into().map_err(|_| ())?).ok_or(())
    }
}
impl TryFrom<u128> for LightLevel {
    type Error = ();
    fn try_from(value : u128) -> Result<Self, Self::Error> {
        Self::new(value.try_into().map_err(|_| ())?).ok_or(())
    }
}


pub type LightLevelProvider<'l> = IntProvider<'l, LightLevel>;
