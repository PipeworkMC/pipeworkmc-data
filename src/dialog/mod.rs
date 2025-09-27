//! Dialog menus.


use crate::{
    action::Action,
    item_stack::ItemStack,
    text::Text
};
use crate::slice_is_empty;
use std::borrow::Cow;
use serde::{
    Serialize as Ser,
    Deserialize as Deser
};
use syndebug::SynDebug;


/// A dialog menu.
#[derive(Clone, Ser, Deser, Debug, SynDebug)]
pub struct Dialog {
    /// Menu type.
    #[serde(flatten)]
    pub kind      : DialogKind,
    /// Menu title.
    pub title     : Text,
    /// Body elements in this menu.
    #[serde(skip_serializing_if = "slice_is_empty")]
    pub body      : Cow<'static, [DialogBody]>,
    /// Input elements in this menu.
    #[serde(skip_serializing_if = "slice_is_empty")]
    pub inputs    : Cow<'static, [DialogInput]>,
    /// Whether this menu can be closed by pressing escape.
    #[serde(rename = "can_close_with_escape")]
    pub escapable : bool,
    /// What to do after submitting an action to the server.
    #[serde(rename = "after_action")]
    pub after     : DialogAfterAction
}

/// Types of dialog menus.
#[derive(Clone, Ser, Deser, Debug, SynDebug)]
#[serde(tag = "type")]
pub enum DialogKind {
    /// Notice menu.
    #[serde(rename = "minecraft:notice", alias = "notice")]
    Notice {
        /// Single button.
        #[serde(skip_serializing_if = "Option::is_none")]
        action : Option<DialogButton>
    },
    /// Confirmation menu.
    #[serde(rename = "minecraft:confirmation", alias = "confirmation")]
    Confirmation {
        /// Yes button.
        yes : DialogButton,
        /// No button.
        no  : DialogButton
    },
    /// Mutli-action menu.
    #[serde(rename = "minecraft:multi_action", alias = "multi_action")]
    MultiAction {
        /// Buttons.
        actions : Cow<'static, [DialogButton]>,
        /// Number of columns to display the buttons in.
        #[serde(skip_serializing_if = "Option::is_none")]
        columns : Option<u32>,
        /// Exit button.
        #[serde(rename = "exit_action", skip_serializing_if = "Option::is_none")]
        exit    : Option<DialogButton>
    },
    /// Server links menu.
    #[serde(rename = "minecraft:server_links", alias = "server_links")]
    ServerLinks {
        /// Exit button.
        #[serde(rename = "exit_action", skip_serializing_if = "Option::is_none")]
        exit         : Option<DialogButton>,
        /// Number of columns to display the buttons in.
        #[serde(skip_serializing_if = "Option::is_none")]
        columns      : Option<u32>,
        /// Width of the buttons.
        #[serde(skip_serializing_if = "Option::is_none")]
        button_width : Option<u32>
    },
    /// Dialog list menu.
    #[serde(rename = "minecraft:dialog_list", alias = "dialog_list")]
    DialogList {
        /// Dialog buttons.
        dialogs      : Cow<'static, [Dialog]>,
        #[serde(rename = "exit_action", skip_serializing_if = "Option::is_none")]
        /// Exit button.
        exit         : Option<DialogButton>,
        /// Number of columns to display the buttons in.
        #[serde(skip_serializing_if = "Option::is_none")]
        columns      : Option<u32>,
        /// Width of the buttons.
        #[serde(skip_serializing_if = "Option::is_none")]
        button_width : Option<u32>
    }
}

/// A button in a dialog menu.
#[derive(Clone, Ser, Deser, Debug, SynDebug)]
pub struct DialogButton {
    /// Button label.
    pub label   : Text,
    /// Button tooltip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltip : Option<Text>,
    /// Width of this button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width   : Option<u32>,
    /// Action to execute on press.
    pub action  : Action
}

/// Body element in a dialog menu.
#[derive(Clone, Ser, Deser, Debug, SynDebug)]
#[serde(tag = "type")]
pub enum DialogBody {
    /// Text element.
    #[serde(rename = "minecraft:plain_message", alias = "plain_message")]
    Plain {
        /// Text to display.
        contents : Text,
        /// Width of the text.
        #[serde(skip_serializing_if = "Option::is_none")]
        width    : Option<u32>
    },
    /// Item element.
    #[serde(rename = "minecraft:item", alias = "item")]
    Item {
        /// Item stack to display.
        stack       : ItemStack,
        /// Description of the element.
        description : DialogItemBodyDesc,
        #[serde(rename = "show_decoration")]
        /// Whether to show item count and damage bar.
        decoration  : bool,
        /// Whether to show a tooltip on hover.
        #[serde(rename = "show_tooltip")]
        tooltip     : bool,
        /// Width of the element.
        #[serde(skip_serializing_if = "Option::is_none")]
        width       : Option<u32>,
        /// Height of the element.
        #[serde(skip_serializing_if = "Option::is_none")]
        height      : Option<u32>
    }
}

/// Description of an item body element.
#[derive(Clone, Ser, Deser, Debug, SynDebug)]
pub struct DialogItemBodyDesc {
    /// Message text.
    pub contents : Text,
    /// Width of message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width    : Option<u32>
}

/// Input element in a dialog menu.
#[derive(Clone, Ser, Deser, Debug, SynDebug)]
pub struct DialogInput {
    /// Input type.
    #[serde(flatten)]
    pub kind  : DialogInputKind,
    /// Key to tie input to when sending to the server.
    pub key   : String,
    /// Element label.
    pub label : Text
}

/// Types of inputs elements in dialog menus.
#[derive(Clone, Ser, Deser, Debug, SynDebug)]
#[serde(tag = "type")]
pub enum DialogInputKind {
    /// Textbox element.
    #[serde(rename = "minecraft:text", alias = "text")]
    Text {
        /// Width of the element.
        #[serde(skip_serializing_if = "Option::is_none")]
        width         : Option<u32>,
        /// Whether to show the label.
        label_visible : bool,
        /// Initial text in the textbox.
        initial       : Cow<'static, str>,
        /// Maximum number of characters that can be entered into the textbox.
        #[serde(rename = "max_length")]
        max_len       : u32,
        /// Multiline settings.
        multiline     : DialogTextInputMultiline
    },
    /// Checkbox element.
    #[serde(rename = "minecraft:boolean", alias = "boolean")]
    Boolean {
        /// Initial state of the checkbox.
        initial  : bool,
        /// Value sent to the server if this checkbox is enabled.
        on_true  : Cow<'static, str>,
        /// Value sent to the server if this checkbox is disabled.
        on_false : Cow<'static, str>
    },
    /// Dropdown element.
    #[serde(rename = "minecraft:single_option", alias = "single_option")]
    SingleOption {
        /// Whether to show the label.
        label_visible : bool,
        /// Width of the element.
        #[serde(skip_serializing_if = "Option::is_none")]
        width         : Option<u32>,
        /// Dropdown options.
        options       : Cow<'static, [DialogInputOption]>
    },
    /// Slider element.
    #[serde(rename = "minecraft:number_range", alias = "number_range")]
    NumberRange {
        /// Translation key to use when building the label.
        label_format : Cow<'static, str>,
        /// Width of the element.
        #[serde(skip_serializing_if = "Option::is_none")]
        width        : Option<u32>,
        /// Minimum slider value.
        #[serde(rename = "start")]
        min          : u32,
        /// Maximum slider value.
        #[serde(rename = "end")]
        max          : u32,
        /// Slider step size.
        #[serde(skip_serializing_if = "Option::is_none")]
        step         : Option<u32>,
        /// Initial value of the slider.
        initial      : u32
    }
}

/// Multiline settings for textbox inputs.
#[derive(Clone, Ser, Deser, Debug, SynDebug)]
pub struct DialogTextInputMultiline {
    /// Maximum number of lines.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_lines : Option<u32>,
    /// Height of the element.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height    : Option<u32>
}

/// Option for dropdown inputs.
#[derive(Clone, Ser, Deser, Debug, SynDebug)]
pub struct DialogInputOption {
    /// Value sent to the server if this option is selected.
    pub id      : String,
    /// Display of this option.
    pub display : Text,
    /// Whether this option is the initial value of the dropdown.
    pub initial : bool
}

/// What to do after submitting dialog menu inputs to the server.
#[derive(Clone, Ser, Deser, Debug, SynDebug)]
pub enum DialogAfterAction {
    /// Close the menu.
    #[serde(rename = "close")]
    Close,
    /// Do nothing.
    #[serde(rename = "none")]
    None,
    /// Show a "Waiting for Response" screen.
    #[serde(rename = "wait_for_response")]
    WaitForResponse
}
