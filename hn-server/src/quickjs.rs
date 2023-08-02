use anyhow::{Context as _, Result};

#[tracing::instrument(skip_all)]
pub(crate) fn serialize_to_js_value<S: serde::Serialize>(a: S) -> Result<quick_js::JsValue> {
    let json_with_parens = {
        let mut vec = Vec::with_capacity(128);
        vec.push(b'(');
        // This could be a lot faster if we directly serialized into the libsysquickjs types.
        // but that seems like it would be more for the fun of it
        let _json = serde_json::to_writer(&mut vec, &a).context("stringifying value")?;
        vec.push(b')');
        unsafe {
            // We do not emit invalid UTF-8.
            String::from_utf8_unchecked(vec)
        }
    };

    let ctx = quick_js::Context::new().context("creating JS context")?;
    ctx.eval(&json_with_parens).context("evaluating json")
}
