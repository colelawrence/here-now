#![allow(non_camel_case_types, unused)]
use derive_codegen::{Codegen, Generation};
use i_hn_server_proc::protocol_type;
use std::marker::PhantomData;

use crate::prelude::get_crate_path;

#[protocol_type("global")]
#[serde(transparent)]
pub struct UsrString(String);

#[protocol_type("global")]
#[serde(transparent)]
pub struct DevString(String);

#[protocol_type("global", as = "`${string}//${string}`")]
pub struct GlobalID(String, String);

#[protocol_type("global")]
#[serde(transparent)]
pub struct ChannelID(String);

#[protocol_type("global")]
#[serde(transparent)]
pub struct Key(String);

#[protocol_type("global")]
#[serde(transparent)]
pub struct KeyTarget(String);

#[protocol_type("global")]
#[serde(transparent)]
pub struct LiveID(String);

mod driver {
    use super::*;

    #[protocol_type("driver")]
    pub enum Out {
        /// TODO: Is this replaceable by "OFFER" semantics?
        DECLARE_SERVICE {
            title: UsrString,
            key: Key,
            /// Protocols would be kinda similar to traits in Rust
            /// I'm wondering if this is too "red pale".
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            #[cfg(feature = "protocols")]
            protocols: Vec<GlobalID>,
        },
    }

    #[protocol_type("driver")]
    pub enum In {
        CREATE_SERVICE {
            service_key: KeyTarget,
            channel: ChannelID,
        },
        // Do we need to "resume" a service, or can we just replay the preious conversation like in Temporal?
        // RESUME_SERVICE {
        //     key: LocalKeyFilter,
        //     channel: ChannelID,
        //     // // hmm, is state here necessary?
        //     // // why doesn't "iam" need save and load?
        //     // state: serde_json::Value,
        // },
    }
}

mod iam {
    use super::*;

    #[protocol_type("iam")]
    pub enum InputValueType {
        TEXT { text: Option<String> },
        CHOICE { choice_key: Option<KeyTarget> },
    }

    #[protocol_type("iam")]
    pub struct InputValue {
        input_key: KeyTarget,
        r#type: InputValueType,
        /// Context for this value?
        reason: DevString,
    }

    #[protocol_type("iam")]
    #[cfg(feature = "interact")]
    pub enum SetInputType {
        TEXT { to_text: Option<String> },
        CHOICE { to_choice_key: Option<KeyTarget> },
    }

    #[protocol_type("iam")]
    #[cfg(feature = "interact")]
    pub enum InteractionType {
        SET_INPUT {
            /// [UIInput] key.
            input_key: KeyTarget,
            #[serde(rename = "type")]
            r#type: SetInputType,
        },
    }

    #[protocol_type("iam")]
    pub enum In {
        /// should "interact" be just "ASK"?
        #[cfg(feature = "interact")]
        INTERACT {
            /// [Out::UI] key.
            ui_key: KeyTarget,
            #[serde(rename = "type")]
            r#type: InteractionType,
        },
        ASK {
            offer_key: KeyTarget,
            channel: ChannelID,
            given_params: Vec<InputValueType>,
        }, // LOAD {
           //     state: serde_json::Value,
           // },
    }

    #[protocol_type("iam")]
    pub enum Out {
        IDENTIFY {
            title: UsrString,
            /// Protocols would be kinda similar to traits in Rust
            /// I'm wondering if this is too "red pale".
            #[cfg(feature = "protocols")]
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            protocols: Vec<GlobalID>,
        },
        // what if we need to remove the section?
        UI {
            key: Key,
            title: UsrString,
            order: f64,
            items: Vec<UIItem>,
        },
        /// Alternative of DECLARE_SERVICE?
        OFFER {
            key: Key,
            title: UsrString,
            known_params: Vec<UIItem>,
        },
        // what if we need to remove/resolve the error?
        RAISE {
            key: Key,
            ui_key: KeyTarget,
            related_input_keys: Vec<KeyTarget>,
            summary: UsrString,
            /// The origin of this raise
            reason: DevString,
            // llm: resolution strategies ?
            // Maybe these should be part of some other message? Maybe multiple messages should be combined into a "render" like function?
            // suggestions: Vec<{ key: LocalKey, summary: UsrString, interactions: Vec<In.INTERACT>, reason: DevString }>
        },
        RESOLVE {
            raise_key: KeyTarget,
            reason: DevString,
        },
        // SAVE {
        //     state: serde_json::Value,
        //     reason: DevString,
        // },
    }

    #[protocol_type("iam")]
    pub enum UIItem {
        INPUT(UIInput),
        CONTENT(UIContent),
        WARNING {
            summarized: Option<UIContent>,
            content: Vec<UIContent>,
        },
    }

    #[protocol_type("iam")]
    pub struct UIInput {
        key: Key,
        label: UsrString,
        #[serde(rename = "type")]
        r#type: UIInputType,
    }

    #[protocol_type("iam")]
    pub enum UIContent {
        HEADING {
            content: UsrString,
            // level
        },
        PARAGRAPH {
            content: UsrString,
        },
    }

    #[protocol_type("iam")]
    pub enum UIInputType {
        TEXT {
            #[serde(default, skip_serializing_if = "Option::is_none")]
            #[cfg(feature = "stateful-iam")]
            current_text: Option<String>,
            /// e.g. `"+1 (913) 555-1234"`, `"+1 (917) 555-4000"`, `"+1 (816) 555-9000"`
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            examples: Vec<String>,
            /// e.g. `"Full phone number with country code"`
            #[serde(default, skip_serializing_if = "Option::is_none")]
            format_description: Option<String>,
        },
        CHOICE {
            /// Current "choice key"
            #[cfg(feature = "stateful-iam")]
            current_choice_key: Option<KeyTarget>,
            choices: Vec<UIInputChoice>,
        },
    }

    #[protocol_type("iam")]
    pub struct UIInputChoice {
        key: Key,
        /// Label for this choice such as `"On"` or `"Off"`.
        label: UsrString,
        /// Optionally supply additional inputs
        /// that can be set for when this choice is selected.
        /// This enables a configuration approach similar to enums in Rust.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        inputs: Vec<UIInput>,
    }
}

mod thread {
    //! Something like Zulip threads?
    use super::*;

    #[protocol_type("thread")]
    pub enum Out {
        
    }

    #[protocol_type("thread")]
    pub enum In {
        
    }
}

#[test]
#[ignore]
fn generate() {
    use std::process::Command;
    Generation::for_tag("protocol-global")
        .include_tag("protocol-driver")
        .as_arg_of(
            Command::new("deno")
                .arg("run")
                .current_dir(get_crate_path().join("protocols"))
                .arg("./generator/generate-protocol.ts")
                .arg("--includeLocationsRelativeTo=")
                .arg("--fileName=protocol/is/driver.v0.gen.ts"),
        )
        .write()
        .print();
    Generation::for_tag("protocol-global")
        .include_tag("protocol-iam")
        .as_arg_of(
            Command::new("deno")
                .arg("run")
                .current_dir(get_crate_path().join("protocols"))
                .arg("./generator/generate-protocol.ts")
                .arg("--includeLocationsRelativeTo=")
                .arg("--fileName=protocol/is/iam.v0.gen.ts"),
        )
        .write()
        .print();
}
