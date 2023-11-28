#[derive(thiserror::Error)]
pub enum Error {
    Tauri(#[from] tauri::Error),
    Other(String),
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{self:?}"))
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Tauri(err) => write!(f, "Tauri error: {err}"),
            Error::Other(err) => write!(f, "Other error: {err}"),
        }
    }
}
impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Tauri(err) => write!(f, "Tauri error: {err:?}"),
            Error::Other(err) => write!(f, "Other error: {err:?}"),
        }
    }
}
