use i_hn_desktop_ui_messages_proc::{shared, to_executor, to_ui};

/// A string reference to something that exists in the UI.
#[shared]
#[serde(transparent)]
pub struct UID(String);

#[to_ui]
pub enum ToUI {
    ShowMainWindow,
    ShowScreenShare,
}

#[to_executor]
pub enum ToExecutor {
    OpenScreenShare,
    OpenMainWindow,
    HidMainWindow,
    HidScreenShare,
}
