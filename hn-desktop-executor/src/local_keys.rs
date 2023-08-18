use std::path::PathBuf;

use crate::prelude::*;

pub(super) fn get_keys() -> Result<keys::LocalKeys> {
    match get_existing_keys() {
        Ok(keys) => return Ok(keys),
        Err(err) => {
            eprintln!("failed to get existing keys: {err}");
        }
    }

    let key_path = get_key_path()?;
    let local_keys = keys::init();
    std::fs::write(
        key_path,
        serde_json::to_string(&local_keys).context("serialize private key")?,
    )
    .context("write local keys")?;

    Ok(local_keys)
}

fn get_key_path() -> Result<PathBuf> {
    let base_dir = PathBuf::from("hn");
    let key_dir = base_dir.join("keys");
    std::fs::create_dir_all(&key_dir).context("created key directory")?;
    Ok(key_dir.join("secret-local-keys"))
}

fn get_existing_keys() -> Result<keys::LocalKeys> {
    let key_path = get_key_path()?;
    let local_keys = serde_json::from_slice(&std::fs::read(&key_path).context("read local keys")?)
        .context("parse private key")?;
    Ok(local_keys)
}
