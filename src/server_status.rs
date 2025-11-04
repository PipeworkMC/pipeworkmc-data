//! Server list information.


use crate::{
    bounded_string::BoundedString,
    text::{
        Text,
        TextContent,
        TextComponent
    },
    uuid::Uuid,
    version::Version
};
use std::borrow::Cow;
use serde::{
    Serialize as Ser,
    Serializer as Serer,
    Deserialize as Deser,
    Deserializer as Deserer
};


/// Server list information.
#[derive(Ser, Deser, Debug)]
pub struct Status<'l> {
    /// Server version.
    pub version               : StatusVersion<'l>,
    /// Online player information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub players               : Option<StatusPlayers>,
    /// The server list message of the day text.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub motd                  : Option<Text>,
    /// A base 64 PNG image.
    ///
    /// Do not include `data:image/png;base64,` at the start.
    #[serde(skip_serializing_if = "Option::is_none", serialize_with = "add_png_b64_header", deserialize_with = "remove_png_b64_header")]
    pub favicon               : Option<Cow<'l, str>>,
    /// Whether the server requires chat message signatures.
    #[serde(rename = "enforcesSecureChat")]
    pub requires_chat_signing : bool,
    /// Whether the server has a mod like No Chat Reports which strips chat message signatures.
    #[serde(rename = "preventsChatReports")]
    pub prevents_chat_reports : bool
}

const PNG_B64_HEADER : &str = "data:image/png;base64,";
fn add_png_b64_header<'l, S : Serer>(png_b64 : &Option<Cow<'l, str>>, ser : S) -> Result<S::Ok, S::Error> {
    if let Some(png_b64) = png_b64 {
        ser.serialize_str(&format!("{PNG_B64_HEADER}{png_b64}"))
    } else { ser.serialize_none() }
}
fn remove_png_b64_header<'de, D : Deserer<'de>>(deser : D) -> Result<Option<Cow<'static, str>>, D::Error> {
    Ok(if let Some(mut png_b64) = <Option<String> as Deser<'de>>::deserialize(deser)? {
        if (png_b64.starts_with(PNG_B64_HEADER)) {
            png_b64.replace_range(..PNG_B64_HEADER.len(), "");
        }
        png_b64.shrink_to_fit();
        Some(Cow::Owned(png_b64))
    } else { None })
}

/// Server list version information.
#[derive(Ser, Deser, Debug)]
pub struct StatusVersion<'l> {
    /// Name of the version.
    pub name     : Cow<'l, str>,
    /// Protocol ID of the version.
    pub protocol : u32
}

/// Server list player information.
#[derive(Ser, Deser, Debug)]
pub struct StatusPlayers {
    /// Number of players currently online.
    #[serde(rename = "online")]
    pub current : u32,
    /// Maximum number of players allowed on the server.
    pub max     : u32,
    /// A sample of online players.
    pub sample  : Cow<'static, [StatusPlayer]>
}

/// An entry in the server list player sample.
#[derive(Ser, Deser, Clone, Debug)]
pub struct StatusPlayer {
    /// UUID of the player.
    #[serde(rename = "id")]
    pub uuid : Uuid,
    /// Name of the player.
    #[serde(flatten)]
    pub name : StatusPlayerName
}

/// The name of a player in the server list player sample.
#[derive(Ser, Deser, Clone, Debug)]
#[serde(untagged)]
pub enum StatusPlayerName {
    /// BoundedString.
    Bounded(BoundedString<16>),
    /// String.
    String(String)
}


impl Default for Status<'_> {
    fn default() -> Self { Self {
        version               : StatusVersion::default(),
        players               : None,
        motd                  : Some(Text { components : Cow::Borrowed(&[TextComponent {
            content : TextContent::Literal { text : Cow::Borrowed("A PipeworkMC Server") },
            .. TextComponent::EMPTY
        }]) }),
        favicon               : None,
        requires_chat_signing : false,
        prevents_chat_reports : true
    } }
}

impl Default for StatusVersion<'_> {
    #[inline]
    fn default() -> Self { Self {
        name     : Cow::Borrowed(Version::CURRENT.earliest_name()),
        protocol : Version::CURRENT.id()
    } }
}
