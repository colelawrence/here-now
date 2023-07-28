mod quick_svelte_template {
    use crate::prelude::*;
    use derive_codegen::Codegen;

    #[derive(Serialize, Codegen)]
    #[codegen(tags = "svelte")]
    #[codegen(outputname = "discord")]
    struct DiscordSettings {
        app_id: Option<String>,
        app_secret: Option<String>,
        oauth2_client_secret: Option<String>,
    }

    #[test]
    #[ignore]
    fn generate() {
        use std::process::Command;
        derive_codegen::Generation::for_tag("svelte")
            .as_arg_of(
                Command::new("deno")
                    .args("run ./generator/generate-typescript.ts".split(' '))
                    .current_dir(get_crate_path().join("templates")),
            )
            .write()
            .print();
    }
}
mod quick {
    use crate::prelude::*;
    use quick_js::{Context, JsValue};
    use serde::{ser::SerializeStruct, Serializer};

    fn serialize_to_js_value<S: Serialize>(a: S) -> Result<JsValue> {
        let json_with_parens = {
            let mut vec = Vec::with_capacity(128);
            vec.push(b'(');
            // This could be a lot faster if we directly serialized into the libsysquickjs types.
            // but that seems like it would be more for the fun of it
            let mut json = serde_json::to_writer(&mut vec, &a).context("stringifying value")?;
            vec.push(b')');
            unsafe {
                // We do not emit invalid UTF-8.
                String::from_utf8_unchecked(vec)
            }
        };

        let ctx = Context::new().context("creating JS context")?;
        ctx.eval(&json_with_parens).context("evaluating json")
    }

    #[derive(Serialize)]
    struct Data {
        people: Vec<Person>,
    }

    #[derive(Serialize)]
    struct Person {
        name: String,
        favorite_color: String,
    }

    #[tokio::test]
    async fn test_quick() {
        test_logger();
        let ctx = Context::new().unwrap();
        let test_svelte_output =
            get_crate_path().join("../svelte-tools/tests/json_printer.template.gen.cjs");
        // get_crate_path().join("../svelte-tools/tests/increment.svelte-preview-component.cjs");
        let code = tokio::fs::read_to_string(&test_svelte_output)
            .await
            .unwrap();
        ctx.set_global("module", JsValue::Object(Default::default()))
            .unwrap();
        ctx.eval(&code).expect("success");

        let values = Data {
            people: vec![
                Person {
                    favorite_color: "blue".to_string(),
                    name: "Cole".to_string(),
                },
                Person {
                    favorite_color: "green".to_string(),
                    name: "Anne".to_string(),
                },
                Person {
                    favorite_color: "orange".to_string(),
                    name: "Craig".to_string(),
                },
            ],
        };

        let input = serialize_to_js_value(&values).expect("serialized");
        ctx.set_global("_input_", input).unwrap();
        let val = ctx
            .eval("module.exports.render({ value: _input_ })")
            .unwrap();

        dbg!(val);
    }
}
