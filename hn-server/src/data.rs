#![allow(non_camel_case_types, unused)]
use i_hn_server_proc::protocol_type;

use std::marker::PhantomData;

use derive_codegen::{Codegen, Generation};

use crate::prelude::get_crate_path;

pub struct Send<T> {
    _mark: PhantomData<T>,
}

#[protocol_type("agent", as = "`${string}//${string}`")]
pub struct GlobalID(String, String);

#[protocol_type("agent")]
#[serde(transparent)]
pub struct ChannelID(String);

#[protocol_type("agent")]
#[serde(transparent)]
pub struct LocalKey(String);

#[protocol_type("agent")]
#[serde(transparent)]
pub struct LiveID(String);

#[protocol_type("agent")]
pub enum Out {
    IDENTIFY {
        key: LocalKey,
    },
    DECLARE_SERVICE {
        key: LocalKey,
        /// Protocols are kinda like traits in Rust
        protocols: Vec<GlobalID>,
    },
}

#[protocol_type("agent")]
pub enum In {
    CREATE_SERVICE {
        key: LocalKey,
        channel: ChannelID,
    },
    RESUME_SERVICE {
        key: LocalKey,
        channel: ChannelID,
        state: serde_json::Value,
    },
}

pub struct AgentProtocol {
    name: String,
    send: Send<AgentProtocol>,
}

#[test]
fn generate() {
    use std::process::Command;
    Generation::for_tag("protocol-agent")
        .as_arg_of(
            Command::new("deno")
                .arg("run")
                .current_dir(get_crate_path().join("./protocols"))
                .arg("./generator/generate-typescript.ts"),
        )
        .write()
        .print();
}
