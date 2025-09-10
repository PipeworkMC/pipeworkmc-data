use super::{
    Text,
    TextComponent,
    TextContent,
    TextStyle
};
use std::borrow::Cow;
use serde::{
    Serialize as Ser,
    Serializer as Serer,
    Deserialize as Deser,
    Deserializer as Deserer
};


impl<'de> Deser<'de> for Text {
    #[inline]
    fn deserialize<D>(deserer : D) -> Result<Self, D::Error>
    where
        D : Deserer<'de>
    { Ok(TextComponent::deserialize(deserer)?.into()) }
}

#[derive(Deser)]
#[serde(untagged)]
pub(super) enum SingleableText {
    Single(TextComponent),
    Many(Cow<'static, [TextComponent]>)
}

impl From<SingleableText> for Text {
    fn from(value : SingleableText) -> Self {
        Self { components : match (value) {
            SingleableText::Single(component) => Cow::Owned(vec![component]),
            SingleableText::Many(components)  => components
        } }
    }
}


#[derive(Ser, Deser)]
#[serde(untagged)]
pub(super) enum ExtraedTextComponent {
    Literal {
        text  : Cow<'static, str>,
        extra : [ExtraedTextComponentInner; 1]
    }
}
#[derive(Ser, Deser)]
pub(super) struct ExtraedTextComponentInner {
    #[serde(flatten)]
    content : TextContent,
    #[serde(flatten)]
    style   : TextStyle
}

impl From<TextComponent> for ExtraedTextComponent {
    fn from(component : TextComponent) -> Self {
        Self::Literal {
            text  : Cow::Borrowed(""),
            extra : [ExtraedTextComponentInner {
                content : component.content,
                style   : component.style
            }]
        }
    }
}


#[derive(Ser, Deser)]
#[serde(tag = "action")]
pub(super) enum HoverEventedTooltip<'l> {
    #[serde(rename = "show_text")]
    Tooltip {
        #[serde(rename = "value")]
        text : Cow<'l, Text>
    }
}


pub(super) fn ser_hover_event_tooltip<S>(tooltip : &Option<Text>, serer : S) -> Result<S::Ok, S::Error>
where
    S : Serer
{
    let Some(tooltip) = tooltip
        else { unreachable!(); };
    HoverEventedTooltip::Tooltip { text : Cow::Borrowed(tooltip) }.serialize(serer)
}

pub(super) fn deser_hover_event_tooltip<'de, D>(deserer : D) -> Result<Option<Text>, D::Error>
where
    D : Deserer<'de>
{
    Ok(Option::<HoverEventedTooltip>::deserialize(deserer)?
        .map(|HoverEventedTooltip::Tooltip { text }| text.into_owned())
    )
}
