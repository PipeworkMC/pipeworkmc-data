//! Executable actions.


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


/// An executable action.
#[derive(Ser, Deser, Clone, Debug, SynDebug)]
#[serde(tag = "action")]
pub enum Action {
    /// Open a URL.
    #[serde(rename = "open_url")]
    OpenUrl {
        /// The URL to open.
        url : Cow<'static, str>
    },
    /// Run a command.
    #[serde(rename = "run_command")]
    RunCommand {
        /// The command to run.
        ///
        /// If the command is not prefixed with a `/` character,
        ///  a chat message will be sent instead,
        command : Cow<'static, str>
    },
    /// Replace the text in the chat bar.
    #[serde(rename = "suggest_command")]
    SuggestCommand {
        /// The command to insert.
        command : Cow<'static, str>
    },
    /// Opens a page in the currently opened book.
    #[serde(rename = "set_book_page")]
    SetBookPage {
        /// The page to switch to.
        page : u32
    },
    /// Overwrites the clipboard.
    #[serde(rename = "set_clipboard")]
    SetClipboard {
        /// The text to put in the clipboard.
        text : Cow<'static, str>
    },
    /// Opens a dialog menu.
    #[serde(rename = "show_dialog")]
    ShowDialog {
        /// The dialog to open.
        dialog : Box<Dialog>
    },
    /// Send a custom payload to the server.
    #[serde(rename = "custom")]
    Custom {
        /// ID of the channel to send on.
        id      : Ident,
        /// The payload to send.
        payload : String
    }
}
