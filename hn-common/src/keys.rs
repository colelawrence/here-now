#![allow(unused)]

pub use hpke;
use rand::{rngs::StdRng, SeedableRng};

use base64::Engine;

use hpke::{
    aead::{AeadTag, ChaCha20Poly1305},
    kdf::HkdfSha384,
    kem::X25519HkdfSha256,
    Deserializable, Kem as KemTrait, OpModeR, OpModeS, Serializable,
};
use serde::de::Visitor;

// These are the only algorithms we're gonna use for this example
pub type Kem = X25519HkdfSha256;
pub type Aead = ChaCha20Poly1305;
pub type Kdf = HkdfSha384;

pub type PrivateKey = <Kem as KemTrait>::PrivateKey;
pub type PublicKey = <Kem as KemTrait>::PublicKey;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub enum PublicKeyKind {
    #[serde(serialize_with = "serde_ser_key", deserialize_with = "serde_des_key")]
    X25519HkdfSha256(<Kem as KemTrait>::PublicKey),
}

impl std::fmt::Debug for PublicKeyKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::X25519HkdfSha256(arg0) => f
                .debug_tuple("X25519HkdfSha256")
                .field(&"(public)")
                .finish(),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub enum PrivateKeyKind {
    #[serde(serialize_with = "serde_ser_key", deserialize_with = "serde_des_key")]
    X25519HkdfSha256(<Kem as KemTrait>::PrivateKey),
}

const BASE_64_URL_ENGINE: base64::engine::GeneralPurpose =
    base64::engine::general_purpose::URL_SAFE_NO_PAD;

fn serde_ser_key<S: serde::Serializer, K: Serializable>(
    key: &K,
    ser: S,
) -> Result<S::Ok, S::Error> {
    ser.collect_str(&ser_key(key))
}

struct KeyVisitor<K> {
    _marker: std::marker::PhantomData<K>,
}
impl<K> KeyVisitor<K> {
    fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}
impl<'de, K: Deserializable> Visitor<'de> for KeyVisitor<K> {
    type Value = K;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a base64 url key string")
    }
    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut seq = seq;
        let mut bytes = Vec::new();
        while let Some(item) = seq.next_element::<u8>()? {
            bytes.push(item);
        }
        K::from_bytes(&bytes).map_err(|k| <A::Error as serde::de::Error>::custom(k))
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        des_key(v)
    }
}
fn serde_des_key<'de, D: serde::Deserializer<'de>, K: Deserializable>(
    des: D,
) -> Result<K, D::Error> {
    des.deserialize_string(KeyVisitor::<K>::new())
}
fn ser_key<K: Serializable>(key: &K) -> String {
    BASE_64_URL_ENGINE.encode(key.to_bytes())
}

fn des_key<K: Deserializable, E: serde::de::Error>(base64_key: &str) -> Result<K, E> {
    K::from_bytes(
        &BASE_64_URL_ENGINE
            .decode(base64_key)
            .map_err(|e| E::custom(e))?,
    )
    .map_err(|k| E::custom(k))
}

impl std::fmt::Debug for PrivateKeyKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::X25519HkdfSha256(arg0) => f
                .debug_tuple("X25519HkdfSha256")
                .field(&"(private)")
                .finish(),
        }
    }
}

/// Initialize a fresh keypair
pub fn init() -> (PrivateKeyKind, PublicKeyKind) {
    let mut csprng = StdRng::from_entropy();
    let (privatek, publ) = Kem::gen_keypair(&mut csprng);
    (
        PrivateKeyKind::X25519HkdfSha256(privatek),
        PublicKeyKind::X25519HkdfSha256(publ),
    )
}

#[test]
fn test() {
    let v = serde_json::to_string(&init()).unwrap();
    eprintln!("init = {v}");
    let v = serde_json::from_str::<(PrivateKeyKind, PublicKeyKind)>(&v).unwrap();
    eprintln!("back = {v:?}");
}
