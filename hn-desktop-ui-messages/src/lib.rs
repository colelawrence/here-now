use i_hn_desktop_ui_messages_proc::{shared, to_executor, to_ui};

pub trait SendToUI: 'static + Send + Sync {
    fn send_to_ui(&self, msg: ToUI);
}

pub trait SendToExecutor: 'static + Send + Sync {
    fn send_to_executor(&self, msg: ToExecutor);
}

/// A string reference to something that exists in the UI.
#[shared]
#[serde(transparent)]
pub struct UID(String);

#[shared]
pub struct Settings {
    pub server_url: Setting<String>,
    pub server_url_2: Setting<String>,
}

#[shared]
pub enum Setting<T> {
    Value(T),
    NoValue,
    Unchanged,
}

#[to_ui]
pub enum ToUI {
    ShowMainWindow,
    ShowSettings(Settings),
}

#[to_executor]
pub enum ToExecutor {
    OpenSettings,
    UpdateSettings(Settings),
    OpenMainWindow,
    HidMainWindow,
    HidSettings,
}
