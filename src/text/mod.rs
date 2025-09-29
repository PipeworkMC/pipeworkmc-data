//! Formatted text.


use crate::{
    action::Action,
    colour::{ Rgb, Argb },
    ident::Ident
};
use crate::slice_is_empty;
use core::{
    mem,
    ops::{ Add, AddAssign }
};
use std::borrow::Cow;
use serde::{
    Serialize as Ser,
    Deserialize as Deser
};
use syndebug::SynDebug;


mod fmt;
mod ser;


/// A collection of formatted text components.
#[derive(Ser, Clone, Debug, SynDebug)]
#[serde(transparent)]
pub struct Text {
    /// Components within this text, each with individual styling information.
    pub components : Cow<'static, [TextComponent]>
}

/// A formatted text copmonent.
#[derive(Ser, Deser, Clone, Debug, SynDebug)]
#[serde(into = "ser::ExtraedTextComponent")]
pub struct TextComponent {
    /// The content in this text component,.
    #[serde(flatten)]
    pub content : TextContent,
    /// The styling information for this text component.
    #[serde(flatten)]
    pub style   : TextStyle
}

/// The content of a text component.
#[derive(Ser, Deser, Clone, Debug, SynDebug)]
#[serde(untagged)]
pub enum TextContent {
    /// A literal string.
    Literal {
        /// Text to display.
        text : Cow<'static, str>
    },
    /// A translated component.
    Translate {
        /// Translation key.
        #[serde(rename = "translate")]
        key      : Cow<'static, str>,
        /// Fallback text to use when the translation key does not exist.
        #[serde(skip_serializing_if = "Option::is_none")]
        fallback : Option<Cow<'static, str>>,
        /// Interpolation replacements.
        #[serde(skip_serializing_if = "slice_is_empty", default)]
        with     : Cow<'static, [Text]>
    },
    /// A keybind component.
    Keybind {
        /// ID of the keybind.
        #[serde(rename = "keybind")]
        id : Cow<'static, str>
    }
}

/// The styling information for a textc component.
#[derive(Ser, Deser, Clone, Debug, SynDebug)]
pub struct TextStyle {
    /// Text display colour.
    #[serde(rename = "color", serialize_with = "Rgb::to_hex", deserialize_with = "Rgb::from_hex_or_name")]
    pub colour    : Rgb,
    /// Font resource ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font      : Option<Ident>,
    /// Bolded.
    #[serde(default)]
    pub bold      : bool,
    /// Italicised
    #[serde(default)]
    pub italic    : bool,
    /// Underlined
    #[serde(rename = "underlined", default)]
    pub underline : bool,
    /// Crossed out
    #[serde(rename = "strikethrough", default)]
    pub strike    : bool,
    /// Obfuscated
    #[serde(rename = "obfuscated", default)]
    pub obfuscate : bool,
    /// Text shadow colour.
    #[serde(rename = "shadow_color", skip_serializing_if = "Option::is_none")]
    pub shadow    : Option<Argb>,
    /// Text inserted to chat bar on component shift-clicked.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insertion : Option<Cow<'static, str>>,
    /// Action to execute on component clicked.
    #[serde(rename = "click_event", skip_serializing_if = "Option::is_none")]
    pub on_click  : Option<Action>,
    /// Text to display on component hovered.
    #[serde(rename = "hover_event", skip_serializing_if = "Option::is_none", serialize_with = "ser::ser_hover_event_tooltip", deserialize_with = "ser::deser_hover_event_tooltip", default)]
    pub tooltip   : Option<Text>
}


impl Text {

    /// Create a new [`Text`] with a single literal component.
    #[inline]
    pub fn literal<S>(text : S) -> Self
    where
        S : Into<Cow<'static, str>>
    { TextContent::Literal {
        text : text.into()
    }.into() }

    /// Create a new [`Text`] with a single translatable component.
    #[inline]
    pub fn translate<S>(key : S) -> Self
    where
        S : Into<Cow<'static, str>>
    { TextContent::Translate {
        key      : key.into(),
        fallback : None,
        with     : Cow::Borrowed(&[])
    }.into() }

    /// Create a new [`Text`] with a single translatable component with fallback.
    #[inline]
    pub fn translate_or<S, F>(key : S, fallback : F) -> Self
    where
        S : Into<Cow<'static, str>>,
        F : Into<Cow<'static, str>>
    { TextContent::Translate {
        key      : key.into(),
        fallback : Some(fallback.into()),
        with     : Cow::Borrowed(&[])
    }.into() }

    /// Create a new [`Text`] with a single translatable component with interpolation components.
    #[inline]
    pub fn translate_with<S, W>(key : S, with : W) -> Self
    where
        S : Into<Cow<'static, str>>,
        W : Into<Cow<'static, [Text]>>
    { TextContent::Translate {
        key      : key.into(),
        fallback : None,
        with     : with.into()
    }.into() }

    /// Create a new [`Text`] with a single translatable component with fallback and interpolation components.
    #[inline]
    pub fn translate_with_or<S, W, F>(key : S, with : W, fallback : F) -> Self
    where
        S : Into<Cow<'static, str>>,
        W : Into<Cow<'static, [Text]>>,
        F : Into<Cow<'static, str>>
    { TextContent::Translate {
        key      : key.into(),
        fallback : Some(fallback.into()),
        with     : with.into()
    }.into() }

    /// Create a new [`Text`] with a single keybind component.
    #[inline]
    pub fn keybind<S>(id : S) -> Self
    where
        S : Into<Cow<'static, str>>
    { TextContent::Keybind {
        id : id.into()
    }.into() }

}

impl Text {

    /// Apply a function to every component in this [`Text`].
    pub fn apply<F>(mut self, mut f : F) -> Self
    where
        F : FnMut(&mut TextComponent)
    {
        let mut components = self.components.into_owned();
        for component in &mut components {
            f(component);
        }
        self.components = Cow::Owned(components);
        self
    }

}


impl TextComponent {
    /// An empty literal component.
    pub const EMPTY : Self = Self {
        content : TextContent::Literal { text : Cow::Borrowed("") },
        style   : TextStyle::EMPTY
    };
}
impl Default for TextComponent {
    #[inline]
    fn default() -> Self { Self::EMPTY }
}
impl From<&'static str> for TextComponent {
    fn from(value : &'static str) -> Self { Self {
        content : TextContent::Literal { text : Cow::Borrowed(value) },
        ..Self::EMPTY
    } }
}
impl From<String> for TextComponent {
    fn from(value : String) -> Self { Self {
        content : TextContent::Literal { text : Cow::Owned(value) },
        ..Self::EMPTY
    } }
}
impl From<Cow<'static, str>> for TextComponent {
    fn from(value : Cow<'static, str>) -> Self { Self {
        content : TextContent::Literal { text : value },
        ..Self::EMPTY
    } }
}
impl From<TextContent> for TextComponent {
    fn from(value : TextContent) -> Self { Self {
        content : value,
        ..Self::EMPTY
    } }
}

impl TextStyle {
    /// A text style with no styling behaviour.
    pub const EMPTY : Self = Self {
        colour    : Rgb::WHITE,
        font      : None,
        bold      : false,
        italic    : false,
        underline : false,
        strike    : false,
        obfuscate : false,
        shadow    : None,
        insertion : None,
        on_click  : None,
        tooltip   : None
    };
}
impl Default for TextStyle {
    #[inline]
    fn default() -> Self { Self::EMPTY }
}


impl<T> From<T> for Text
where
    TextComponent : From<T>
{ fn from(value : T) -> Self {
    Self { components : Cow::Owned(vec![value.into()]) }
} }


/// A trait which enables `self` to be given colour.
pub trait TextFormatted
where
    Self : Sized
{
    /// Set this text's display colour.
    fn colour<C>(self, colour : C) -> Text
        where C : Into<Rgb>;
    /// Set this text's display colour to `§0`.
    fn black(self) -> Text { self.colour(Rgb::BLACK) }
    /// Set this text's display colour to `§1`.
    fn dark_blue(self) -> Text { self.colour(Rgb::DARK_BLUE) }
    /// Set this text's display colour to `§2`.
    fn dark_green(self) -> Text { self.colour(Rgb::DARK_GREEN) }
    /// Set this text's display colour to `§3`.
    fn dark_cyan(self) -> Text { self.colour(Rgb::DARK_CYAN) }
    /// Set this text's display colour to `§4`.
    fn dark_red(self) -> Text { self.colour(Rgb::DARK_RED) }
    /// Set this text's display colour to `§5`.
    fn purple(self) -> Text { self.colour(Rgb::PURPLE) }
    /// Set this text's display colour to `§6`.
    fn orange(self) -> Text { self.colour(Rgb::ORANGE) }
    /// Set this text's display colour to `§7`.
    fn grey(self) -> Text { self.colour(Rgb::GREY) }
    /// Set this text's display colour to `§8`.
    fn dark_grey(self) -> Text { self.colour(Rgb::DARK_GREY) }
    /// Set this text's display colour to `§9`.
    fn blue(self) -> Text { self.colour(Rgb::BLUE) }
    /// Set this text's display colour to `§a`.
    fn green(self) -> Text { self.colour(Rgb::GREEN) }
    /// Set this text's display colour to `§b`.
    fn cyan(self) -> Text { self.colour(Rgb::CYAN) }
    /// Set this text's display colour to `§c`.
    fn red(self) -> Text { self.colour(Rgb::RED) }
    /// Set this text's display colour to `§d`.
    fn pink(self) -> Text { self.colour(Rgb::PINK) }
    /// Set this text's display colour to `§e`.
    fn yellow(self) -> Text { self.colour(Rgb::YELLOW) }
    /// Set this text's display colour to `§f`.
    fn white(self) -> Text { self.colour(Rgb::WHITE) }
    /// Set this text's font resource ID.
    fn font<R>(self, resource : R) -> Text
        where R : Into<Ident>;
    /// Set this text's font to the default resource.
    fn no_font(self) -> Text;
    /// Make this text bolded.
    fn bold(self) -> Text;
    /// Make this text unbolded.
    fn no_bold(self) -> Text;
    /// Make this text italicised.
    fn italic(self) -> Text;
    /// Make this text unitalicised.
    fn no_italic(self) -> Text;
    /// Make this text underlined.
    fn underline(self) -> Text;
    /// Make this text not underlined.
    fn no_underline(self) -> Text;
    /// Make this text crossed out.
    fn strike(self) -> Text;
    /// Make this text not crossed out.
    fn no_strike(self) -> Text;
    /// Make this text obfuscated.
    fn obfuscate(self) -> Text;
    /// Make this text unobfuscated.
    fn no_obfuscate(self) -> Text;
    /// Set this text's shadow colour.
    fn shadow<C>(self, colour : C) -> Text
        where C : Into<Argb>;
    /// Remove this text's shadow.
    fn no_shadow(self) -> Text { self.shadow(Argb::TRANSPARENT) }
    /// Set this text's shadow color to the default.
    fn default_shadow(self) -> Text;
    /// Set the string that is inserted into chat bar when this text is shift-clicked.
    fn insertion<S>(self, text : S) -> Text
        where S : Into<Cow<'static, str>>;
    /// Disable chat bar string insertion when this text is shift-clicked.
    fn no_insertion(self) -> Text;
    /// Set the action that is executed when this text is clicked.
    fn on_click(self, action : Action) -> Text;
    /// Disable actions executed when this text is clicked.
    fn no_on_click(self) -> Text;
    /// Set this text's tooltip.
    fn tooltip<S>(self, text : S) -> Text
        where S : Into<Text>;
    /// Disable this text's tooltip.
    fn no_tooltip(self) -> Text;
    /// Disable all styling information on this text.
    fn reset(self) -> Text;
}

impl TextFormatted for Text {
    fn colour<C>(self, colour : C) -> Text
    where C : Into<Rgb> {
        let colour = colour.into();
        self.apply(|component| { component.style.colour = colour; })
    }
    fn font<R>(self, resource : R) -> Text
    where R : Into<Ident> {
        let resource = resource.into();
        self.apply(|component| { component.style.font = Some(resource.clone()); })
    }
    fn no_font(self) -> Text {
        self.apply(|component| { component.style.font = None; })
    }
    fn bold(self) -> Text {
        self.apply(|component| { component.style.bold = true; })
    }
    fn no_bold(self) -> Text {
        self.apply(|component| { component.style.bold = false; })
    }
    fn italic(self) -> Text {
        self.apply(|component| { component.style.italic = true; })
    }
    fn no_italic(self) -> Text {
        self.apply(|component| { component.style.italic = true; })
    }
    fn underline(self) -> Text {
        self.apply(|component| { component.style.underline = true; })
    }
    fn no_underline(self) -> Text {
        self.apply(|component| { component.style.underline = true; })
    }
    fn strike(self) -> Text {
        self.apply(|component| { component.style.strike = true; })
    }
    fn no_strike(self) -> Text {
        self.apply(|component| { component.style.strike = true; })
    }
    fn obfuscate(self) -> Text {
        self.apply(|component| { component.style.obfuscate = true; })
    }
    fn no_obfuscate(self) -> Text {
        self.apply(|component| { component.style.obfuscate = true; })
    }
    fn shadow<C>(self, colour : C) -> Text
    where C : Into<Argb> {
        let colour = colour.into();
        self.apply(|component| { component.style.shadow = Some(colour); })
    }
    fn no_shadow(self) -> Text {
        self.apply(|component| { component.style.shadow = Some(Argb::TRANSPARENT); })
    }
    fn default_shadow(self) -> Text {
        self.apply(|component| { component.style.shadow = None; })
    }
    fn insertion<S>(self, text : S) -> Text
    where S : Into<Cow<'static, str>> {
        let text = text.into();
        self.apply(|component| { component.style.insertion = Some(text.clone()); })
    }
    fn no_insertion(self) -> Text {
        self.apply(|component| { component.style.insertion = None; })
    }
    fn on_click(self, action : Action) -> Text {
        self.apply(|component| { component.style.on_click = Some(action.clone()); })
    }
    fn no_on_click(self) -> Text {
        self.apply(|component| { component.style.on_click = None; })
    }
    fn tooltip<S>(self, text : S) -> Text
    where S : Into<Text> {
        let text = text.into();
        self.apply(|component| { component.style.tooltip = Some(text.clone()); })
    }
    fn no_tooltip(self) -> Text {
        self.apply(|component| { component.style.tooltip = None; })
    }
    fn reset(self) -> Text {
        self.apply(|component| {
            component.style.colour    = Rgb::WHITE;
            component.style.font      = None;
            component.style.bold      = false;
            component.style.italic    = false;
            component.style.underline = false;
            component.style.strike    = false;
            component.style.obfuscate = false;
            component.style.shadow    = None;
            component.style.insertion = None;
            component.style.on_click  = None;
            component.style.tooltip   = None;
        })
    }
}

impl<T> TextFormatted for T
where
    TextComponent : From<T>
{
    #[inline]
    fn colour<C>(self, colour : C) -> Text
    where C : Into<Rgb> { Text::from(self).colour(colour) }
    #[inline]
    fn font<R>(self, resource : R) -> Text
    where R : Into<Ident> { Text::from(self).font(resource) }
    #[inline]
    fn no_font(self) -> Text { Text::from(self).no_font() }
    #[inline]
    fn bold(self) -> Text { Text::from(self).bold() }
    #[inline]
    fn no_bold(self) -> Text { Text::from(self).no_bold() }
    #[inline]
    fn italic(self) -> Text { Text::from(self).italic() }
    #[inline]
    fn no_italic(self) -> Text { Text::from(self).no_italic() }
    #[inline]
    fn underline(self) -> Text { Text::from(self).underline() }
    #[inline]
    fn no_underline(self) -> Text { Text::from(self).no_underline() }
    #[inline]
    fn strike(self) -> Text { Text::from(self).strike() }
    #[inline]
    fn no_strike(self) -> Text { Text::from(self).no_strike() }
    #[inline]
    fn obfuscate(self) -> Text { Text::from(self).obfuscate() }
    #[inline]
    fn no_obfuscate(self) -> Text { Text::from(self).no_obfuscate() }
    #[inline]
    fn shadow<C>(self, colour : C) -> Text
    where C : Into<Argb> { Text::from(self).shadow(colour) }
    #[inline]
    fn no_shadow(self) -> Text { Text::from(self).no_shadow() }
    #[inline]
    fn default_shadow(self) -> Text { Text::from(self).default_shadow() }
    #[inline]
    fn insertion<S>(self, text : S) -> Text
    where S : Into<Cow<'static, str>> { Text::from(self).insertion(text) }
    #[inline]
    fn no_insertion(self) -> Text { Text::from(self).no_insertion() }
    #[inline]
    fn on_click(self, action : Action) -> Text { Text::from(self).on_click(action) }
    #[inline]
    fn no_on_click(self) -> Text { Text::from(self).no_on_click() }
    #[inline]
    fn tooltip<S>(self, text : S) -> Text
    where S : Into<Text> { Text::from(self).tooltip(text) }
    #[inline]
    fn no_tooltip(self) -> Text { Text::from(self).no_tooltip() }
    #[inline]
    fn reset(self) -> Text { Text::from(self).reset() }
}


impl<T> Add<T> for Text
where
    T : Into<Text>
{
    type Output = Text;
    #[inline]
    fn add(mut self, rhs : T) -> Self::Output {
        self += rhs;
        self
    }
}

impl<T> AddAssign<T> for Text
where
    T : Into<Text>
{
    fn add_assign(&mut self, rhs : T) {
        let mut components = mem::replace(&mut self.components, Cow::Borrowed(&[])).into_owned();
        components.extend_from_slice(&rhs.into().components);
        self.components = Cow::Owned(components);
    }
}

macro impl_addtext_for($ty:ty) {
    impl Add<Text> for $ty {
        type Output = Text;
        fn add(self, mut rhs : Text) -> Self::Output {
            let mut components = rhs.components.into_owned();
            components.insert(0, self.into());
            rhs.components = Cow::Owned(components);
            rhs
        }
    }
}
 impl_addtext_for!(&'static str);
 impl_addtext_for!(String);
 impl_addtext_for!(Cow<'static, str>);
 impl_addtext_for!(TextComponent);
 impl_addtext_for!(TextContent);
