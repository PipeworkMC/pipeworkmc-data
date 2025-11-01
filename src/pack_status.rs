//! Resource pack status updates sent by the client.


use netzer::prelude::*;


/// Resource pack status updates sent by the client.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, NetEncode)]
// #[netzer(ordinal, convert = "VarInt<u32>", try_from)]
#[netzer(ordinal, convert = "VarInt<u32>", try_into)]
#[repr(u8)]
pub enum PackStatus {
    /// The client has successfully loaded the resource pack.
    Loaded         = 0,
    /// The client explicitely declined the resource pack by clicking on the reject button.
    Declined       = 1,
    /// The client failed to download the resource pack.
    DownloadFailed = 2,
    /// The client has explicitely accepted the resource pack by clicking on the accept button, but has not yet loaded it.
    Accepted       = 3,
    /// The client has downloaded the resource pack, but has not yet loaded it.
    Downloaded     = 4,
    /// The resource pack URL was invalid.
    InvalidURL     = 5,
    /// The client failed to reload its resource packs.
    FailedToReload = 6,
    /// The client has discarded the resource pack.
    Discarded      = 7
}
