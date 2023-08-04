#![allow(unused)]
use i_hn_server_proc::protocol_type;

use std::marker::PhantomData;

use derive_codegen::{Codegen, Generation};

pub struct Send<T> {
    _mark: PhantomData<T>,
}

#[protocol_type("agent")]
pub struct GlobalID(String, String);

#[protocol_type("agent")]
pub struct LocalKey(String);

pub enum Agent {
    OFFER { key: LocalKey, protocol: GlobalID },
}

pub struct AgentProtocol {
    name: String,
    send: Send<AgentProtocol>,
}

#[test]
fn generate() {
    // eprintln!(
    //     "{}",
    //     Generation::for_tag("protocol-agent").to_input_json_pretty()
    // )
}
