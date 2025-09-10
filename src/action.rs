use crate::{
    dialog::Dialog,
    ident::Ident
};
use std::borrow::Cow;
use serde::{
    Serialize as Ser,
    Deserialize as Deser
};
use syndebug::SynDebug;


#[derive(Ser, Deser, Clone, Debug, SynDebug)]
#[serde(tag = "action")]
pub enum Action {
    #[serde(rename = "open_url")]
    OpenUrl {
        url : Cow<'static, str>
    },
    #[serde(rename = "run_command")]
    RunCommand {
        command : Cow<'static, str>
    },
    #[serde(rename = "suggest_command")]
    SuggestCommand {
        command : Cow<'static, str>
    },
    #[serde(rename = "set_book_page")]
    SetBookPage {
        page : u32
    },
    #[serde(rename = "set_clipboard")]
    SetClipboard {
        text : Cow<'static, str>
    },
    #[serde(rename = "show_dialog")]
    ShowDialog {
        dialog : Box<Dialog>
    },
    #[serde(rename = "custom")]
    Custom {
        id      : Ident,
        payload : String
    }
}
