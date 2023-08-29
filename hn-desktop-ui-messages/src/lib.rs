use hn_hinted_id::HintedID;
use i_hn_desktop_ui_messages_proc::{shared, to_executor, to_ui};

pub trait SendToUI: 'static + Send + Sync {
    fn send_all_to_ui(&self, msgs: Vec<ToUI>);
    fn send_to_ui(&self, msg: ToUI) {
        self.send_all_to_ui(vec![msg]);
    }
}

pub trait SendToExecutor: 'static + Send + Sync {
    fn send_all_to_executor(&self, msgs: Vec<ToExecutor>);
    fn send_to_executor(&self, msg: ToExecutor) {
        self.send_all_to_executor(vec![msg]);
    }
}

#[shared]
pub struct PServerSettings {
    pub uid: HintedID,
    pub label: Setting<String>,
    #[serde(alias = "serverUrl")]
    pub server_url: Setting<String>,
}

#[to_ui]
pub struct PServerSummary {
    pub uid: HintedID,
    pub label: Option<String>,
    pub server_url: Option<String>,
    /// Automatically generated on connection.
    pub server_device_id: String,
}

/// Profile settings
#[shared]
pub struct ProfileSettings {
    pub uid: HintedID,
    pub label: Setting<String>,
}

#[to_ui]
pub struct ProfileSummary {
    pub uid: HintedID,
    pub label: Option<String>,
    pub servers: Vec<PServerSummary>,
    /// Automatically generated on creation.
    pub public_key_debug: String,
}

#[shared]
pub enum Setting<T> {
    Value(T),
    NoValue,
    Unchanged,
}

impl<T: Clone> Setting<T> {
    pub fn from_option(original: &Option<T>) -> Self {
        original
            .as_ref()
            .map_or(Self::NoValue, |v| Self::Value(v.clone()))
    }
}

impl<T> Setting<T> {
    pub fn from_compared<E: PartialEq + Into<T>>(original: E, update: E) -> Self {
        if original == update {
            Self::Unchanged
        } else {
            Self::Value(update.into())
        }
    }
    pub fn changed(&self) -> Option<Option<&T>> {
        match &self {
            Self::Value(v) => Some(Some(v)),
            Self::NoValue => Some(None),
            Self::Unchanged => None,
        }
    }
}

#[to_ui]
pub enum ToUI {
    ShowMainWindow,
    ShowPServerSettings(PServerSettings),
    UpdateProfiles(Vec<ProfileSummary>),
    NotifyProfileSettings(HintedID, UINotification),
    NotifyPServerSettings(HintedID, UINotification),
    HideProfileSettings(HintedID),
    HidePServerSettings(HintedID),
    // Some kind of "update which profile you're looking at" ?
    // ChangeProfileTo(UID),
}

#[to_ui]
pub struct UINotification {
    pub key: String,
    pub title: String,
    pub body: String,
}

#[to_executor]
pub enum ToExecutor {
    OpenMainWindow,
    HidMainWindow,
    HidSettings,
    CreateProfile,
    OpenProfileSettings(HintedID),
    OpenPServerSettings(HintedID),
    SwitchProfileTo(HintedID),
    DeleteProfile(HintedID),
    AddServerByURL(executor::AddServerByURL),
    UpdateProfileSettings(executor::UpdateProfileSettings),
    UpdatePServerSettings(executor::UpdatePServerSettings),
}

pub mod executor {
    use i_hn_desktop_ui_messages_proc::to_executor;

    use crate::{HintedID, PServerSettings, ProfileSettings};

    #[to_executor]
    pub struct AddServerByURL {
        pub target_profile: HintedID,
        pub server_url: String,
    }

    #[to_executor]
    pub struct UpdatePServerSettings(pub PServerSettings);

    #[to_executor]
    pub struct UpdateProfileSettings(pub ProfileSettings);
}
