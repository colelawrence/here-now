use toml_edit::{Decor, Item, Value};

pub(crate) fn update_toml_key(
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

fn get_item_suffix(item: &Item) -> Option<&str> {
    get_item_decor(item)
        .and_then(|a| a.suffix())
        .and_then(|a| a.as_str())
}
fn get_item_decor(item: &Item) -> Option<&Decor> {
    match item {
        Item::None => None,
        Item::ArrayOfTables(_) => None,
        Item::Value(v) => Some(get_decor(v)),
        Item::Table(t) => Some(t.decor()),
    }
}
fn get_decor(value: &Value) -> &Decor {
    match value {
        Value::String(a) => a.decor(),
        Value::Integer(a) => a.decor(),
        Value::Float(a) => a.decor(),
        Value::Boolean(a) => a.decor(),
        Value::Datetime(a) => a.decor(),
        Value::Array(a) => a.decor(),
        Value::InlineTable(a) => a.decor(),
    }
}
