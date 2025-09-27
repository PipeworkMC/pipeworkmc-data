//! Colours.


use serde::{
    Serialize as Ser,
    Serializer as Serer,
    Deserialize as Deser,
    Deserializer as Deserer,
    de::Error as _
};
use syndebug::SynDebug;


const LOWER_HEX_DIGITS : [u8; 16] = [b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'a', b'b', b'c', b'd', b'e', b'f'];


/// An RGB colour.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, SynDebug)]
pub struct Rgb {
    /// Red
    pub r : u8,
    /// Green
    pub g : u8,
    /// Blue
    pub b : u8
}

impl Rgb {

    /// Create a new [`Rgb`] from a red, green, and blue value.
    #[inline(always)]
    pub const fn new(r : u8, g : u8, b : u8) -> Self {
        Self { r, g, b }
    }

    /// Create a new [`Rgb`] by decoding a [`u32`].
    #[inline]
    pub const fn from_u32(v : u32) -> Self {
        Self {
            r : ((v >> 16) & 0b11111111) as u8,
            g : ((v >>  8) & 0b11111111) as u8,
            b : ((v      ) & 0b11111111) as u8
        }
    }

    /// Create a new greyscale [`Rgb`] with a brightness.
    #[inline(always)]
    pub const fn splat(v : u8) -> Self {
        Self { r : v, g : v, b : v }
    }

    /// Vanilla `§0` colour.
    pub const BLACK      : Self = Self { r :   0, g :   0, b :   0 };
    /// Vanilla `§1` colour.
    pub const DARK_BLUE  : Self = Self { r :   0, g :   0, b : 170 };
    /// Vanilla `§2` colour.
    pub const DARK_GREEN : Self = Self { r :   0, g : 170, b :   0 };
    /// Vanilla `§3` colour.
    pub const DARK_CYAN  : Self = Self { r :   0, g : 170, b : 170 };
    /// Vanilla `§4` colour.
    pub const DARK_RED   : Self = Self { r : 170, g :   0, b :   0 };
    /// Vanilla `§5` colour.
    pub const PURPLE     : Self = Self { r : 170, g :   0, b : 170 };
    /// Vanilla `§6` colour.
    pub const ORANGE     : Self = Self { r : 255, g : 170, b :   0 };
    /// Vanilla `§7` colour.
    pub const GREY       : Self = Self { r : 170, g : 170, b : 170 };
    /// Vanilla `§8` colour.
    pub const DARK_GREY  : Self = Self { r :  85, g :  85, b :  85 };
    /// Vanilla `§9` colour.
    pub const BLUE       : Self = Self { r :  85, g :  85, b : 255 };
    /// Vanilla `§a` colour.
    pub const GREEN      : Self = Self { r :  85, g : 255, b :  85 };
    /// Vanilla `§b` colour.
    pub const CYAN       : Self = Self { r :  85, g : 255, b : 255 };
    /// Vanilla `§c` colour.
    pub const RED        : Self = Self { r : 255, g :  85, b :  85 };
    /// Vanilla `§d` colour.
    pub const PINK       : Self = Self { r : 255, g :  85, b : 255 };
    /// Vanilla `§e` colour.
    pub const YELLOW     : Self = Self { r : 255, g : 255, b :  85 };
    /// Vanilla `§f` colour.
    pub const WHITE      : Self = Self { r : 255, g : 255, b : 255 };

    /// Create a new [`Rgb`] by parsing a hexadecimal string or name.
    pub fn from_hex_or_name<'de, D>(deserer : D) -> Result<Self, D::Error>
    where
        D : Deserer<'de>
    {
        let s = String::deserialize(deserer)?;
        if (s.starts_with("#")) {
            if (s.len() != 7) {
                return Err(D::Error::custom("hex colour must be 7 characters long"));
            }
            Ok(Self {
                r : u8::from_str_radix(&s[1..=2], 16).map_err(D::Error::custom)?,
                g : u8::from_str_radix(&s[3..=4], 16).map_err(D::Error::custom)?,
                b : u8::from_str_radix(&s[5..=6], 16).map_err(D::Error::custom)?
            })
        } else { Ok(match (s.as_str()) {
            "black"                   => Self::BLACK,
            "dark_blue"               => Self::DARK_BLUE,
            "dark_green"              => Self::DARK_GREEN,
            "dark_cyan" | "dark_aqua" => Self::DARK_CYAN,
            "dark_red"                => Self::DARK_RED,
            "purple" | "dark_purple"  => Self::PURPLE,
            "orange" | "gold"         => Self::ORANGE,
            "grey" | "gray"           => Self::GREY,
            "dark_grey" | "dark_gray" => Self::DARK_GREY,
            "blue"                    => Self::BLUE,
            "green"                   => Self::GREEN,
            "cyan" | "aqua"           => Self::CYAN,
            "red"                     => Self::RED,
            "pink" | "light_purple"   => Self::PINK,
            "yellow"                  => Self::YELLOW,
            "white"                   => Self::WHITE,
            _ => { return Err(D::Error::custom("colour must be a valid name or start with `#`")); }
        }) }
    }

}

impl Rgb {

    /// Convert this [`Rgb`] to an [`Argb`] with the given alpha.
    #[inline(always)]
    pub const fn with_alpha(self, a : u8) -> Argb {
        Argb { a, r : self.r, g : self.g, b : self.b }
    }

    /// Convert this [`Rgb`] to a fully opaque [`Argb`].
    #[inline(always)]
    pub const fn opaque(self) -> Argb {
        Argb { a : 255, r : self.r, g : self.g, b : self.b }
    }

    /// Encodes this [`Rgb`] as a [`u32`].
    #[inline]
    pub const fn to_u32(self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }

    /// Encodes this [`Rgb`] as a hexadecimal string.
    pub fn to_hex<S>(&self, serer : S) -> Result<S::Ok, S::Error>
    where
        S : Serer
    {
        let mut value  = ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32);
        let mut buf    = [0u8; 7];
        let mut i      = buf.len();
        while (value > 0) {
            i -= 1;
            let digit = LOWER_HEX_DIGITS[(value % 16) as usize];
            value /= 16;
            buf[i] = digit;
        }
        buf[0] = b'#';
        unsafe { str::from_utf8_unchecked(&buf) }.serialize(serer)
    }

}

impl Ser for Rgb {
    #[inline]
    fn serialize<S>(&self, serer : S) -> Result<S::Ok, S::Error>
    where
        S : Serer
    { self.to_u32().serialize(serer) }
}

impl<'de> Deser<'de> for Rgb {
    #[inline]
    fn deserialize<D>(deserer : D) -> Result<Self, D::Error>
    where
        D : Deserer<'de>
    { Ok(Self::from_u32(u32::deserialize(deserer)?)) }
}


/// An ARGB colour.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, SynDebug)]
pub struct Argb {
    /// Alpha (opacity)
    pub a : u8,
    /// Red
    pub r : u8,
    /// Green
    pub g : u8,
    /// Blue
    pub b : u8
}

impl Argb {

    /// Create a new [`Argb`] from an alpha, red, green, and blue value.
    #[inline(always)]
    pub const fn new(a : u8, r : u8, g : u8, b : u8) -> Self {
        Self { a, r, g, b }
    }

    /// Create a new [`Argb`] by decoding a [`u32`].
    #[inline]
    pub const fn from_u32(v : u32) -> Self {
        Self {
            a : ((v >> 24) & 0b11111111) as u8,
            r : ((v >> 16) & 0b11111111) as u8,
            g : ((v >>  8) & 0b11111111) as u8,
            b : ((v      ) & 0b11111111) as u8
        }
    }

    /// Create a new greyscale [`Argb`] with an alpha and brightness.
    #[inline(always)]
    pub const fn splat(a : u8, v : u8) -> Self {
        Self { a, r : v, g : v, b : v }
    }

    /// Vanilla `§0` colour.
    pub const BLACK       : Self = Self { a : 255, r :   0, g :   0, b :   0 };
    /// Vanilla `§1` colour.
    pub const DARK_BLUE   : Self = Self { a : 255, r :   0, g :   0, b : 170 };
    /// Vanilla `§2` colour.
    pub const DARK_GREEN  : Self = Self { a : 255, r :   0, g : 170, b :   0 };
    /// Vanilla `§3` colour.
    pub const DARK_CYAN   : Self = Self { a : 255, r :   0, g : 170, b : 170 };
    /// Vanilla `§4` colour.
    pub const DARK_RED    : Self = Self { a : 255, r : 170, g :   0, b :   0 };
    /// Vanilla `§5` colour.
    pub const PURPLE      : Self = Self { a : 255, r : 170, g :   0, b : 170 };
    /// Vanilla `§6` colour.
    pub const ORANGE      : Self = Self { a : 255, r : 255, g : 170, b :   0 };
    /// Vanilla `§7` colour.
    pub const GREY        : Self = Self { a : 255, r : 170, g : 170, b : 170 };
    /// Vanilla `§8` colour.
    pub const DARK_GREY   : Self = Self { a : 255, r :  85, g :  85, b :  85 };
    /// Vanilla `§9` colour.
    pub const BLUE        : Self = Self { a : 255, r :  85, g :  85, b : 255 };
    /// Vanilla `§a` colour.
    pub const GREEN       : Self = Self { a : 255, r :  85, g : 255, b :  85 };
    /// Vanilla `§b` colour.
    pub const CYAN        : Self = Self { a : 255, r :  85, g : 255, b : 255 };
    /// Vanilla `§c` colour.
    pub const RED         : Self = Self { a : 255, r : 255, g :  85, b :  85 };
    /// Vanilla `§d` colour.
    pub const PINK        : Self = Self { a : 255, r : 255, g :  85, b : 255 };
    /// Vanilla `§e` colour.
    pub const YELLOW      : Self = Self { a : 255, r : 255, g : 255, b :  85 };
    /// Vanilla `§f` colour.
    pub const WHITE       : Self = Self { a : 255, r : 255, g : 255, b : 255 };
    /// Colour with alpha set to zero.
    pub const TRANSPARENT : Self = Self { a :   0, r :   0, g :   0, b :   0 };

    /// Create a new [`Argb`] by parsing a hexadecimal string.
    pub fn from_hex<'de, D>(deserer : D) -> Result<Self, D::Error>
    where
        D : Deserer<'de>
    {
        let s = <&str>::deserialize(deserer)?;
        if (! s.starts_with("#")) {
            return Err(D::Error::custom("hex colour must start with `#`"));
        }
        if (s.len() != 9) {
            return Err(D::Error::custom("hex colour must be 9 characters long"));
        }
        Ok(Self {
            a : u8::from_str_radix(&s[1..=2], 16).map_err(D::Error::custom)?,
            r : u8::from_str_radix(&s[3..=4], 16).map_err(D::Error::custom)?,
            g : u8::from_str_radix(&s[5..=6], 16).map_err(D::Error::custom)?,
            b : u8::from_str_radix(&s[7..=8], 16).map_err(D::Error::custom)?
        })
    }

}

impl Argb {

    /// Convert this [`Argb`] to an [`Rgb`] by removing the alpha.
    #[inline(always)]
    pub const fn without_alpha(self) -> Rgb {
        Rgb { r : self.r, g : self.g, b : self.b }
    }

    /// Encodes this [`Rgb`] as a [`u32`].
    #[inline]
    pub const fn to_u32(self) -> u32 {
        ((self.a as u32) << 24) | ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }

    /// Encodes this [`Argb`] as a hexadecimal string.
    pub fn to_hex<S>(&self, serer : S) -> Result<S::Ok, S::Error>
    where
        S : Serer
    {
        let mut value  = ((self.a as u32) << 24) | ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32);
        let mut buf    = [0u8; 9];
        let mut i      = buf.len();
        while (value > 0) {
            i -= 1;
            let digit = LOWER_HEX_DIGITS[(value % 16) as usize];
            value /= 16;
            buf[i] = digit;
        }
        buf[0] = b'#';
        unsafe { str::from_utf8_unchecked(&buf) }.serialize(serer)
    }

}

impl From<Rgb> for Argb {
    #[inline(always)]
    fn from(value : Rgb) -> Self { value.opaque() }
}

impl Ser for Argb {
    #[inline]
    fn serialize<S>(&self, serer : S) -> Result<S::Ok, S::Error>
    where
        S : Serer
    { self.to_u32().serialize(serer) }
}

impl<'de> Deser<'de> for Argb {
    #[inline]
    fn deserialize<D>(deserer : D) -> Result<Self, D::Error>
    where
        D : Deserer<'de>
    { Ok(Self::from_u32(u32::deserialize(deserer)?)) }
}
