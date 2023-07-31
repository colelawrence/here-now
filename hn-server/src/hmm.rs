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
        let ctx = quick_js::Context::new().unwrap();
        let test_svelte_output =
            get_crate_path().join("../svelte-tools/tests/json_printer.template.compiled.cjs");
        // get_crate_path().join("../svelte-tools/tests/increment.svelte-preview-component.cjs");
        let code = tokio::fs::read_to_string(&test_svelte_output)
            .await
            .unwrap();
        ctx.set_global("module", quick_js::JsValue::Object(Default::default()))
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
