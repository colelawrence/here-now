use toml_edit::Item;

pub fn update_toml_key(
    toml: &mut dyn toml_edit::TableLike,
    key: &str,
    item: Item,
    mut key_comment: Option<String>,
    force_replace_comment: bool,
) {
    let existing = toml
        .key_decor(key)
        .and_then(|decor| {
            decor
                .prefix()
                .and_then(|a| a.as_str())
                .map(|s| s.to_string())
        })
        .unwrap_or_default();
    if !force_replace_comment && !existing.trim().is_empty() {
        key_comment = None;
    }
    let commented_key = match key_comment {
        Some(a) => a
            .lines()
            .map(|ln| ln.trim())
            .map(|s| {
                if s.is_empty() {
                    format!("#\n")
                } else {
                    format!("# {s}\n")
                }
            })
            .collect::<String>(),
        None => existing,
    };
    toml.insert(key, item);
    if let Some(dec) = toml.key_decor_mut(key) {
        dec.set_prefix(commented_key);
        dec.set_suffix(" ");
    }
}
